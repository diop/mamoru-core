use crate::condition::Condition;
use crate::task::Task;
use chrono::{DateTime, Utc};
use mamoru_core::IncidentSeverity;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub severity: IncidentSeverity,
    pub incident_id: String,
}

#[derive(Debug)]
pub struct Playbook {
    pub task: Step,
}

#[derive(Debug)]
pub struct StepsBlock {
    pub condition: Condition,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub struct ParallelBlock {
    pub condition: Condition,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub enum Step {
    Single(SingleStep),
    Steps(StepsBlock),
    Parallel(ParallelBlock),
}

#[derive(Debug)]
pub struct SingleStep {
    pub seq: u32,
    pub id: Option<String>,
    pub condition: Condition,
    pub run: Task,
    pub params: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum PlaybookRunStatus {
    Running,
    Success {
        finished_at: DateTime<Utc>,
    },
    Failed {
        finished_at: DateTime<Utc>,
        step_seq: u32,
        message: String,
    },
}

#[derive(Clone, Debug)]
pub struct PlaybookRun {
    pub status: PlaybookRunStatus,
    pub steps: BTreeMap<u32, StepRun>,
}

#[derive(Clone, Debug)]
pub struct StepRun {
    pub step_seq: u32,
    pub started_at: Option<DateTime<Utc>>,
    pub status: StepRunStatus,
    pub logs: Vec<String>,
}

pub type StepOutputs = BTreeMap<String, String>;

#[derive(Clone, Debug)]
pub enum StepRunStatus {
    Pending,
    Skipped,
    Running {
        waiting_for_confirmation: bool,
    },
    Success {
        finished_at: DateTime<Utc>,
        outputs: StepOutputs,
    },
    Failed {
        failed_at: DateTime<Utc>,
    },
}

impl PlaybookRun {
    pub fn new(playbook: &Playbook) -> Self {
        Self {
            status: PlaybookRunStatus::Running,
            steps: make_step_runs(
                match &playbook.task {
                    Step::Steps(block) => &block.steps,
                    Step::Parallel(block) => &block.steps,
                    single if matches!(single, Step::Single(_)) => std::slice::from_ref(single),
                    _ => panic!(
                        "Playbook task must be either steps, parallel block or a single step"
                    ),
                },
                StepRunStatus::Pending,
            ),
        }
    }

    pub fn has_unfinished_steps(&self) -> bool {
        self.steps.values().any(|step_run| {
            matches!(
                step_run.status,
                StepRunStatus::Pending | StepRunStatus::Running { .. }
            )
        })
    }

    pub fn failed_steps(&self) -> Vec<&StepRun> {
        self.steps
            .values()
            .filter(|step_run| matches!(step_run.status, StepRunStatus::Failed { .. }))
            .collect()
    }
}

#[derive(Debug)]
pub struct ExternalAction {
    pub step_seq: u32,
    pub action: String,
    pub params: BTreeMap<String, String>,
}

pub(crate) fn skip_downstream_tasks(steps: &[Step], runs: &mut BTreeMap<u32, StepRun>) {
    let skipped = make_step_runs(steps, StepRunStatus::Skipped);

    for (seq, step_run) in skipped {
        runs.insert(seq, step_run);
    }
}

pub(crate) fn fail_downstream_tasks(
    steps: &[Step],
    runs: &mut BTreeMap<u32, StepRun>,
    now: DateTime<Utc>,
    message: String,
) {
    let failed = make_step_runs(steps, StepRunStatus::Failed { failed_at: now });

    for (seq, mut step_run) in failed {
        step_run.logs.push(message.clone());
        runs.insert(seq, step_run);
    }
}

pub(crate) fn make_step_runs(steps: &[Step], status: StepRunStatus) -> BTreeMap<u32, StepRun> {
    let mut step_runs = BTreeMap::new();

    for step in steps {
        match step {
            Step::Single(single_step) => {
                step_runs.insert(
                    single_step.seq,
                    StepRun {
                        step_seq: single_step.seq,
                        logs: Vec::new(),
                        status: status.clone(),
                        started_at: None,
                    },
                );
            }
            Step::Steps(block) => {
                step_runs.extend(make_step_runs(&block.steps, status.clone()));
            }
            Step::Parallel(block) => {
                step_runs.extend(make_step_runs(&block.steps, status.clone()));
            }
        }
    }

    step_runs
}

pub enum RunConfirmationStatus {
    Success { outputs: StepOutputs },
    Failed,
}

pub struct RunConfirmation {
    pub logs: Vec<String>,
    pub status: RunConfirmationStatus,
    pub step_seq: u32,
}
