use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use serde::Serialize;

use crate::condition::{Context, Incident, PipelineStatus};
use crate::error::{ParamsEvalError, RunError};
use crate::playbook::{
    fail_downstream_tasks, skip_downstream_tasks, Playbook, SingleStep, Step, Trigger,
};
use crate::task::TaskStatus;
use crate::{
    ExternalAction, PlaybookRun, PlaybookRunStatus, RunConfirmation, RunConfirmationStatus,
    StepOutputs, StepRun, StepRunStatus,
};

struct RunState {
    trigger: Trigger,
    playbook: Playbook,
    run: PlaybookRun,
}

type PlaybookRunId = String;

pub struct EngineCtx {
    pub now: DateTime<Utc>,
    pub eval_ctx: Context,
}

pub struct Engine<'a> {
    params_render: Handlebars<'a>,
    state: BTreeMap<PlaybookRunId, RunState>,
}

impl<'a> Default for Engine<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Engine<'a> {
    /// Creates a new playbook engine
    /// The same engine must be used for the same playbook run
    pub fn new() -> Self {
        let mut params_render = Handlebars::new();
        params_render.set_strict_mode(true);

        Self {
            params_render,
            state: BTreeMap::new(),
        }
    }

    /// Starts a new playbook run
    pub fn start_playbook(
        &mut self,
        now: DateTime<Utc>,
        run_id: PlaybookRunId,
        playbook: Playbook,
        trigger: Trigger,
    ) -> Result<(PlaybookRun, Vec<ExternalAction>), RunError> {
        if self.state.contains_key(&run_id) {
            return Err(RunError::AlreadyStarted(run_id));
        }

        let run = PlaybookRun::new(&playbook);

        let state = RunState {
            trigger,
            playbook,
            run,
        };

        let result = self.run_playbook(now, run_id, state, vec![]);

        Ok(result)
    }

    /// Resumes a playbook run after an external action is completed
    pub fn resume_playbook(
        &mut self,
        now: DateTime<Utc>,
        run_id: PlaybookRunId,
        confirmations: Vec<RunConfirmation>,
    ) -> Result<(PlaybookRun, Vec<ExternalAction>), RunError> {
        let state = self
            .state
            .remove(&run_id)
            .ok_or_else(|| RunError::NotStarted(run_id.clone()))?;

        let result = self.run_playbook(now, run_id, state, confirmations);

        Ok(result)
    }

    fn run_playbook(
        &mut self,
        now: DateTime<Utc>,
        run_id: PlaybookRunId,
        mut state: RunState,
        mut confirmations: Vec<RunConfirmation>,
    ) -> (PlaybookRun, Vec<ExternalAction>) {
        let mut ctx = EngineCtx {
            now,
            eval_ctx: Context {
                pipeline_status: PipelineStatus::Success,
                incident: Incident {
                    id: state.trigger.incident_id.clone(),
                    level: state.trigger.severity.to_str().to_string(),
                },
                outputs: Default::default(),
            },
        };

        let actions = self.run_step(
            &state.playbook.task,
            &mut ctx,
            &mut state.run,
            &mut confirmations,
        );

        if state.run.has_unfinished_steps() {
            state.run.status = PlaybookRunStatus::Running;
            let response = (state.run.clone(), actions);

            self.state.insert(run_id, state);

            return response;
        }

        let failed_steps = state.run.failed_steps();

        if !failed_steps.is_empty() {
            let first_failed_step = failed_steps.first().expect("Failed steps exist");

            state.run.status = PlaybookRunStatus::Failed {
                finished_at: ctx.now,
                step_seq: first_failed_step.step_seq,
                message: format!("Task #{} has failed", first_failed_step.step_seq),
            };
        } else {
            state.run.status = PlaybookRunStatus::Success { finished_at: now };
        }

        (state.run.clone(), actions)
    }

    fn run_step(
        &self,
        step: &Step,
        ctx: &mut EngineCtx,
        run: &mut PlaybookRun,
        confirmations: &mut Vec<RunConfirmation>,
    ) -> Vec<ExternalAction> {
        match step {
            Step::Single(single) => {
                let mut actions = vec![];
                if let Some(action) = self.run_single_step(single, ctx, run, confirmations) {
                    actions.push(action);
                }

                return actions;
            }
            Step::Steps(block) => {
                let must_run = match block.condition.evaluate(&ctx.eval_ctx) {
                    Ok(run) => run,
                    Err(err) => {
                        self.fail_block(
                            ctx,
                            run,
                            &block.steps,
                            format!("Condition evaluation failed: {}", err),
                        );

                        return vec![];
                    }
                };

                if must_run {
                    // Steps block return immediately if task requires confirmation
                    for step in &block.steps {
                        let response = self.run_step(step, ctx, run, confirmations);

                        if !response.is_empty() {
                            return response;
                        }
                    }
                } else {
                    skip_downstream_tasks(&block.steps, &mut run.steps);
                }
            }
            Step::Parallel(block) => {
                let must_run = match block.condition.evaluate(&ctx.eval_ctx) {
                    Ok(run) => run,
                    Err(err) => {
                        self.fail_block(
                            ctx,
                            run,
                            &block.steps,
                            format!("Condition evaluation failed: {}", err),
                        );

                        return vec![];
                    }
                };

                let mut external_actions = vec![];

                if must_run {
                    // Parallel block return after all tasks are executed
                    for step in &block.steps {
                        let response = self.run_step(step, ctx, run, confirmations);

                        external_actions.extend(response);
                    }
                } else {
                    skip_downstream_tasks(&block.steps, &mut run.steps);
                }

                return external_actions;
            }
        };

        vec![]
    }

    /// Runs a single step and returns an external action if it requires it
    /// If the step is already finished, it only applies outputs to the context
    fn run_single_step(
        &self,
        single: &SingleStep,
        ctx: &mut EngineCtx,
        run: &mut PlaybookRun,
        confirmations: &mut Vec<RunConfirmation>,
    ) -> Option<ExternalAction> {
        let step_run = run.steps.get_mut(&single.seq).expect("Step exists");

        match &step_run.status {
            StepRunStatus::Pending => {
                let run = match single.condition.evaluate(&ctx.eval_ctx) {
                    Ok(run) => run,
                    Err(err) => {
                        self.fail_step(
                            ctx,
                            step_run,
                            Some(format!("Condition evaluation failed: {}", err)),
                        );

                        return None;
                    }
                };

                if !run {
                    step_run.status = StepRunStatus::Skipped;
                    return None;
                }

                let params = match self.render_params(&single.params, &ctx.eval_ctx) {
                    Ok(params) => params,
                    Err(err) => {
                        self.fail_step(
                            ctx,
                            step_run,
                            Some(format!("Params evaluation failed: {}", err)),
                        );

                        return None;
                    }
                };

                step_run.started_at = Some(ctx.now);

                match single.run.run(single.seq, params) {
                    TaskStatus::Success { outputs } => {
                        self.succeed_step(ctx, step_run, outputs, single.id.clone());
                    }
                    TaskStatus::Failed { message } => {
                        self.fail_step(ctx, step_run, Some(message));
                    }
                    TaskStatus::External {
                        step_seq,
                        action,
                        params,
                    } => {
                        step_run.status = StepRunStatus::Running {
                            waiting_for_confirmation: true,
                        };

                        return Some(ExternalAction {
                            step_seq,
                            action,
                            params,
                        });
                    }
                }
            }
            StepRunStatus::Running {
                waiting_for_confirmation,
            } => {
                if *waiting_for_confirmation {
                    if let Some(index) = confirmations.iter().position(|c| c.step_seq == single.seq)
                    {
                        let confirmation = confirmations.remove(index);

                        step_run.logs.extend(confirmation.logs);

                        match confirmation.status {
                            RunConfirmationStatus::Success { outputs } => {
                                self.succeed_step(ctx, step_run, outputs, single.id.clone());
                            }
                            RunConfirmationStatus::Failed => {
                                self.fail_step(ctx, step_run, None);
                            }
                        }
                    }
                }
            }
            StepRunStatus::Skipped => {
                ctx.eval_ctx.pipeline_status = PipelineStatus::Skipped;
            }
            StepRunStatus::Success {
                finished_at: _,
                outputs,
            } => {
                ctx.eval_ctx.pipeline_status = PipelineStatus::Success;

                if let Some(id) = &single.id {
                    ctx.eval_ctx.outputs.insert(id.to_string(), outputs.clone());
                }
            }
            StepRunStatus::Failed { failed_at: _ } => {
                ctx.eval_ctx.pipeline_status = PipelineStatus::Failed;
            }
        };

        None
    }

    fn fail_step(&self, ctx: &mut EngineCtx, step_run: &mut StepRun, message: Option<String>) {
        ctx.eval_ctx.pipeline_status = PipelineStatus::Failed;
        step_run.status = StepRunStatus::Failed { failed_at: ctx.now };

        if let Some(message) = message {
            step_run.logs.push(message);
        }
    }

    fn succeed_step(
        &self,
        ctx: &mut EngineCtx,
        step_run: &mut StepRun,
        outputs: StepOutputs,
        id: Option<String>,
    ) {
        ctx.eval_ctx.pipeline_status = PipelineStatus::Success;

        step_run.status = StepRunStatus::Success {
            finished_at: ctx.now,
            outputs: outputs.clone(),
        };

        if let Some(id) = id {
            ctx.eval_ctx.outputs.insert(id, outputs);
        }
    }

    fn fail_block(
        &self,
        ctx: &mut EngineCtx,
        run: &mut PlaybookRun,
        steps: &[Step],
        message: String,
    ) {
        fail_downstream_tasks(
            steps,
            &mut run.steps,
            ctx.now,
            format!("Block condition failed: {}", &message),
        );

        ctx.eval_ctx.pipeline_status = PipelineStatus::Failed;
    }

    fn render_params(
        &self,
        params: &BTreeMap<String, String>,
        ctx: &impl Serialize,
    ) -> Result<BTreeMap<String, String>, ParamsEvalError> {
        let mut substituted = BTreeMap::new();

        for (key, value) in params {
            let rendered = self
                .params_render
                .render_template(value, &ctx)
                .map_err(ParamsEvalError::InvalidParameter)?;

            substituted.insert(key.clone(), rendered);
        }

        Ok(substituted)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::str::FromStr;

    use chrono::DateTime;
    use maplit::btreemap;

    use mamoru_core::IncidentSeverity;

    use crate::condition::Condition;
    use crate::engine::{Engine, RunConfirmation, RunConfirmationStatus};
    use crate::playbook::{ParallelBlock, Playbook, SingleStep, Step, StepsBlock, Trigger};
    use crate::task::{ExternalTask, Task};

    #[test]
    fn test_run() {
        let playbook = Playbook {
            task: Step::Steps(StepsBlock {
                condition: Default::default(),
                steps: vec![
                    Step::Single(SingleStep {
                        seq: 1,
                        id: Some("test".into()),
                        condition: Default::default(),
                        run: Task::Success,
                        params: btreemap! {
                            "foo".to_string() => "bar".to_string(),
                            "something".to_string() => "{{ incident.id }}".to_string(),
                        },
                    }),
                    Step::Steps(StepsBlock {
                        condition: Condition::from_str("outputs.test.foo == 'bar'").unwrap(),
                        steps: vec![
                            Step::Single(SingleStep {
                                seq: 2,
                                id: Default::default(),
                                condition: Default::default(),
                                run: Task::Success,
                                params: btreemap! {
                                    "foo".to_string() => "hello, {{ outputs.test.something }}".to_string(),
                                },
                            }),
                            Step::Single(SingleStep {
                                seq: 3,
                                id: Default::default(),
                                condition: Default::default(),
                                run: Task::Fail,
                                params: BTreeMap::new(),
                            }),
                            Step::Single(SingleStep {
                                seq: 4,
                                id: Default::default(),
                                condition: Default::default(),
                                run: Task::Success,
                                params: BTreeMap::new(),
                            }),
                        ],
                    }),
                    Step::Parallel(ParallelBlock {
                        condition: Condition::Always,
                        steps: vec![
                            Step::Single(SingleStep {
                                seq: 5,
                                id: Default::default(),
                                condition: Condition::Always,
                                run: Task::External(ExternalTask::new("some-task", &[])),
                                params: BTreeMap::new(),
                            }),
                            Step::Single(SingleStep {
                                seq: 6,
                                id: Default::default(),
                                condition: Condition::Always,
                                run: Task::External(ExternalTask::new("some-task", &["foo"])),
                                params: btreemap! {
                                    "foo".to_string() => "{{ outputs.test.something }}".to_string(),
                                },
                            }),
                        ],
                    }),
                ],
            }),
        };

        let trigger = Trigger {
            severity: IncidentSeverity::Info,
            incident_id: "test".to_string(),
        };

        let mut engine = Engine::new();
        let now = DateTime::from_str("2023-08-22T11:07:34.932862Z").unwrap();

        let (response, actions) = engine
            .start_playbook(now, "test".to_string(), playbook, trigger)
            .unwrap();

        expect_test::expect![[r#"
            [
                ExternalAction {
                    step_seq: 5,
                    action: "some-task",
                    params: {},
                },
                ExternalAction {
                    step_seq: 6,
                    action: "some-task",
                    params: {
                        "foo": "test",
                    },
                },
            ]
        "#]]
        .assert_debug_eq(&actions);

        expect_test::expect![[r#"
            PlaybookRun {
                status: Running,
                steps: {
                    1: StepRun {
                        step_seq: 1,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "bar",
                                "something": "test",
                            },
                        },
                        logs: [],
                    },
                    2: StepRun {
                        step_seq: 2,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "hello, test",
                            },
                        },
                        logs: [],
                    },
                    3: StepRun {
                        step_seq: 3,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Failed {
                            failed_at: 2023-08-22T11:07:34.932862Z,
                        },
                        logs: [
                            "i am failed",
                        ],
                    },
                    4: StepRun {
                        step_seq: 4,
                        started_at: None,
                        status: Skipped,
                        logs: [],
                    },
                    5: StepRun {
                        step_seq: 5,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Running {
                            waiting_for_confirmation: true,
                        },
                        logs: [],
                    },
                    6: StepRun {
                        step_seq: 6,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Running {
                            waiting_for_confirmation: true,
                        },
                        logs: [],
                    },
                },
            }
        "#]]
        .assert_debug_eq(&response);

        let now = now + chrono::Duration::seconds(1);

        let (response, actions) = engine
            .resume_playbook(
                now,
                "test".to_string(),
                vec![RunConfirmation {
                    logs: vec!["test".to_string()],
                    status: RunConfirmationStatus::Success {
                        outputs: btreemap! {
                            "yo".to_string() => "i am an external action".to_string(),
                        },
                    },
                    step_seq: 5,
                }],
            )
            .unwrap();

        expect_test::expect![[r#"
            []
        "#]]
        .assert_debug_eq(&actions);

        expect_test::expect![[r#"
            PlaybookRun {
                status: Running,
                steps: {
                    1: StepRun {
                        step_seq: 1,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "bar",
                                "something": "test",
                            },
                        },
                        logs: [],
                    },
                    2: StepRun {
                        step_seq: 2,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "hello, test",
                            },
                        },
                        logs: [],
                    },
                    3: StepRun {
                        step_seq: 3,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Failed {
                            failed_at: 2023-08-22T11:07:34.932862Z,
                        },
                        logs: [
                            "i am failed",
                        ],
                    },
                    4: StepRun {
                        step_seq: 4,
                        started_at: None,
                        status: Skipped,
                        logs: [],
                    },
                    5: StepRun {
                        step_seq: 5,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:35.932862Z,
                            outputs: {
                                "yo": "i am an external action",
                            },
                        },
                        logs: [
                            "test",
                        ],
                    },
                    6: StepRun {
                        step_seq: 6,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Running {
                            waiting_for_confirmation: true,
                        },
                        logs: [],
                    },
                },
            }
        "#]]
        .assert_debug_eq(&response);

        let now = now + chrono::Duration::seconds(1);

        let (response, actions) = engine
            .resume_playbook(
                now,
                "test".to_string(),
                vec![RunConfirmation {
                    logs: vec!["test".to_string()],
                    status: RunConfirmationStatus::Success {
                        outputs: btreemap! {
                            "yo".to_string() => "i am an external action 2".to_string(),
                        },
                    },
                    step_seq: 6,
                }],
            )
            .unwrap();

        expect_test::expect![[r#"
            []
        "#]]
        .assert_debug_eq(&actions);

        expect_test::expect![[r#"
            PlaybookRun {
                status: Failed {
                    finished_at: 2023-08-22T11:07:36.932862Z,
                    step_seq: 3,
                    message: "Task #3 has failed",
                },
                steps: {
                    1: StepRun {
                        step_seq: 1,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "bar",
                                "something": "test",
                            },
                        },
                        logs: [],
                    },
                    2: StepRun {
                        step_seq: 2,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "hello, test",
                            },
                        },
                        logs: [],
                    },
                    3: StepRun {
                        step_seq: 3,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Failed {
                            failed_at: 2023-08-22T11:07:34.932862Z,
                        },
                        logs: [
                            "i am failed",
                        ],
                    },
                    4: StepRun {
                        step_seq: 4,
                        started_at: None,
                        status: Skipped,
                        logs: [],
                    },
                    5: StepRun {
                        step_seq: 5,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:35.932862Z,
                            outputs: {
                                "yo": "i am an external action",
                            },
                        },
                        logs: [
                            "test",
                        ],
                    },
                    6: StepRun {
                        step_seq: 6,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:36.932862Z,
                            outputs: {
                                "yo": "i am an external action 2",
                            },
                        },
                        logs: [
                            "test",
                        ],
                    },
                },
            }
        "#]]
        .assert_debug_eq(&response);
    }

    // tests condition eval error in a block
    #[test]
    fn condition_eval_in_block() {
        let playbook = Playbook {
            task: Step::Steps(StepsBlock {
                condition: Default::default(),
                steps: vec![
                    Step::Single(SingleStep {
                        seq: 1,
                        id: Some("test".into()),
                        condition: Default::default(),
                        run: Task::Success,
                        params: btreemap! {
                            "foo".to_string() => "bar".to_string(),
                            "something".to_string() => "{{ incident.id }}".to_string(),
                        },
                    }),
                    Step::Steps(StepsBlock {
                        condition: Condition::from_str("outputs.test_dummy.foo == 'bar'").unwrap(),
                        steps: vec![
                            Step::Single(SingleStep {
                                seq: 2,
                                id: Default::default(),
                                condition: Default::default(),
                                run: Task::Success,
                                params: btreemap! {
                                    "foo".to_string() => "hello, {{ outputs.test.something }}".to_string(),
                                },
                            }),
                            Step::Single(SingleStep {
                                seq: 3,
                                id: Default::default(),
                                condition: Default::default(),
                                run: Task::Fail,
                                params: BTreeMap::new(),
                            }),
                            Step::Single(SingleStep {
                                seq: 4,
                                id: Default::default(),
                                condition: Default::default(),
                                run: Task::Success,
                                params: BTreeMap::new(),
                            }),
                        ],
                    }),
                    Step::Parallel(ParallelBlock {
                        condition: Condition::Always,
                        steps: vec![Step::Single(SingleStep {
                            seq: 5,
                            id: Default::default(),
                            condition: Condition::Always,
                            run: Task::Success,
                            params: BTreeMap::new(),
                        })],
                    }),
                ],
            }),
        };

        let trigger = Trigger {
            severity: IncidentSeverity::Info,
            incident_id: "test".to_string(),
        };

        let mut engine = Engine::new();
        let now = DateTime::from_str("2023-08-22T11:07:34.932862Z").unwrap();

        let (response, _) = engine
            .start_playbook(now, "test".to_string(), playbook, trigger)
            .unwrap();

        expect_test::expect![[r#"
            PlaybookRun {
                status: Failed {
                    finished_at: 2023-08-22T11:07:34.932862Z,
                    step_seq: 2,
                    message: "Task #2 has failed",
                },
                steps: {
                    1: StepRun {
                        step_seq: 1,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {
                                "foo": "bar",
                                "something": "test",
                            },
                        },
                        logs: [],
                    },
                    2: StepRun {
                        step_seq: 2,
                        started_at: None,
                        status: Failed {
                            failed_at: 2023-08-22T11:07:34.932862Z,
                        },
                        logs: [
                            "Block condition failed: Condition evaluation failed: Task with id \"test_dummy\" is not found.",
                        ],
                    },
                    3: StepRun {
                        step_seq: 3,
                        started_at: None,
                        status: Failed {
                            failed_at: 2023-08-22T11:07:34.932862Z,
                        },
                        logs: [
                            "Block condition failed: Condition evaluation failed: Task with id \"test_dummy\" is not found.",
                        ],
                    },
                    4: StepRun {
                        step_seq: 4,
                        started_at: None,
                        status: Failed {
                            failed_at: 2023-08-22T11:07:34.932862Z,
                        },
                        logs: [
                            "Block condition failed: Condition evaluation failed: Task with id \"test_dummy\" is not found.",
                        ],
                    },
                    5: StepRun {
                        step_seq: 5,
                        started_at: Some(
                            2023-08-22T11:07:34.932862Z,
                        ),
                        status: Success {
                            finished_at: 2023-08-22T11:07:34.932862Z,
                            outputs: {},
                        },
                        logs: [],
                    },
                },
            }
        "#]].assert_debug_eq(&response);
    }
}
