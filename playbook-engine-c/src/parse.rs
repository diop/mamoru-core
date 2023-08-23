use crate::error::ParseError;
use crate::playbook_dto::{
    EngineResponseDto, ExternalActionDto, PlaybookDto, PlaybookRunDto, RunConfirmationDto,
    RunConfirmationStatusDto, StepDto, StepParamDto, StepRunDto,
};
use playbook_engine::{
    task_registry, Condition, ExternalAction, ParallelBlock, Playbook, PlaybookRun,
    RunConfirmation, RunConfirmationStatus, SingleStep, Step, StepsBlock, Trigger,
};

pub fn parse_playbook(playbook_json: &str) -> Result<Playbook, ParseError> {
    let playbook_dto: PlaybookDto =
        serde_json::from_str(playbook_json).map_err(ParseError::JsonParse)?;
    let task = parse_step(playbook_dto.task)?;

    Ok(Playbook { task })
}

pub fn parse_trigger(trigger_json: &str) -> Result<Trigger, ParseError> {
    serde_json::from_str(trigger_json).map_err(ParseError::JsonParse)
}

pub fn parse_confirmations(confirmations_json: &str) -> Result<Vec<RunConfirmation>, ParseError> {
    let confirmations: Vec<RunConfirmationDto> =
        serde_json::from_str(confirmations_json).map_err(ParseError::JsonParse)?;

    let result = confirmations
        .into_iter()
        .map(|confirmation| RunConfirmation {
            logs: confirmation.logs,
            step_seq: confirmation.step_seq,
            status: match confirmation.status {
                RunConfirmationStatusDto::Success { outputs } => RunConfirmationStatus::Success {
                    outputs: outputs
                        .into_iter()
                        .map(|output| (output.name, output.value))
                        .collect(),
                },
                RunConfirmationStatusDto::Failed => RunConfirmationStatus::Failed,
            },
        })
        .collect();

    Ok(result)
}

pub fn make_engine_response(response: (PlaybookRun, Vec<ExternalAction>)) -> EngineResponseDto {
    let (run, actions) = response;

    EngineResponseDto {
        run: make_playbook_run(run),
        external_actions: make_external_actions(actions),
    }
}

fn make_playbook_run(run: PlaybookRun) -> PlaybookRunDto {
    PlaybookRunDto {
        status: run.status.into(),
        steps: run
            .steps
            .into_values()
            .map(|step| StepRunDto {
                step_seq: step.step_seq,
                started_at: step.started_at,
                status: step.status.into(),
                logs: step.logs,
            })
            .collect(),
    }
}

fn make_external_actions(actions: Vec<ExternalAction>) -> Vec<ExternalActionDto> {
    actions
        .into_iter()
        .map(|a| ExternalActionDto {
            step_seq: a.step_seq,
            action: a.action,
            params: a
                .params
                .into_iter()
                .map(|(name, value)| StepParamDto { name, value })
                .collect(),
        })
        .collect()
}

fn parse_step(step_dto: StepDto) -> Result<Step, ParseError> {
    match step_dto {
        StepDto::Single(single) => {
            let task = task_registry()
                .iter()
                .find(|task| task.name() == single.run)
                .ok_or_else(|| ParseError::UnknownTask(single.run.clone()))?
                .clone();

            let missing_parameters: Vec<_> = task
                .required_params()
                .iter()
                .filter(|param| !single.params.iter().any(|p| &p.name == *param))
                .cloned()
                .collect();

            if !missing_parameters.is_empty() {
                return Err(ParseError::MissingParameters {
                    task: single.run,
                    parameters: missing_parameters,
                });
            }

            let condition = parse_condition(single.condition)?;

            let params = single
                .params
                .into_iter()
                .map(|param| (param.name, param.value))
                .collect();

            Ok(Step::Single(SingleStep {
                seq: single.seq,
                id: single.id,
                condition,
                run: task,
                params,
            }))
        }
        StepDto::Steps(block) => {
            let condition = parse_condition(block.condition)?;

            let steps = block
                .steps
                .into_iter()
                .map(parse_step)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Step::Steps(StepsBlock { condition, steps }))
        }
        StepDto::Parallel(block) => {
            let condition = parse_condition(block.condition)?;

            let steps = block
                .steps
                .into_iter()
                .map(parse_step)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Step::Parallel(ParallelBlock { condition, steps }))
        }
    }
}

fn parse_condition(condition: Option<String>) -> Result<Condition, ParseError> {
    condition
        .map_or(Ok(Default::default()), |condition| condition.parse())
        .map_err(ParseError::InvalidCondition)
}
