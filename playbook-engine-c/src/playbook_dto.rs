use chrono::{DateTime, Utc};
use playbook_engine::{PlaybookRunStatus, StepRunStatus};
use serde::{Deserialize, Serialize};

/// Structures to communicate with VC

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookDto {
    pub task: StepDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepsBlockDto {
    #[serde(default)]
    pub condition: Option<String>,
    pub steps: Vec<StepDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParallelBlockDto {
    #[serde(default)]
    pub condition: Option<String>,
    pub steps: Vec<StepDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StepDto {
    Single(SingleStepDto),
    Steps(StepsBlockDto),
    Parallel(ParallelBlockDto),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleStepDto {
    pub seq: u32,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub condition: Option<String>,
    pub run: String,
    #[serde(default)]
    pub params: Vec<StepParamDto>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepParamDto {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOutputDto {
    pub name: String,
    pub value: String,
}

pub type StepOutputsDto = Vec<StepOutputDto>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum RunConfirmationStatusDto {
    Success { outputs: StepOutputsDto },
    Failed,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunConfirmationDto {
    pub logs: Vec<String>,
    pub status: RunConfirmationStatusDto,
    pub step_seq: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineResponseDto {
    pub external_actions: Vec<ExternalActionDto>,
    pub run: PlaybookRunDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalActionDto {
    pub step_seq: u32,
    pub action: String,
    pub params: Vec<StepParamDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookRunDto {
    pub status: PlaybookRunStatusDto,
    pub steps: Vec<StepRunDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepRunDto {
    pub step_seq: u32,
    pub started_at: Option<DateTime<Utc>>,
    pub status: StepRunStatusDto,
    pub logs: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum StepRunStatusDto {
    Pending,
    Skipped,
    Running {
        #[serde(rename = "waitingForConfirmation")]
        waiting_for_confirmation: bool,
    },
    Success {
        #[serde(rename = "finishedAt")]
        finished_at: DateTime<Utc>,
        outputs: StepOutputsDto,
    },
    Failed {
        #[serde(rename = "failedAt")]
        failed_at: DateTime<Utc>,
    },
}

impl From<StepRunStatus> for StepRunStatusDto {
    fn from(value: StepRunStatus) -> Self {
        match value {
            StepRunStatus::Pending => Self::Pending,
            StepRunStatus::Skipped => Self::Skipped,
            StepRunStatus::Running {
                waiting_for_confirmation,
            } => Self::Running {
                waiting_for_confirmation,
            },
            StepRunStatus::Success {
                finished_at,
                outputs,
            } => Self::Success {
                finished_at,
                outputs: outputs
                    .into_iter()
                    .map(|(name, value)| StepOutputDto { name, value })
                    .collect(),
            },
            StepRunStatus::Failed { failed_at } => Self::Failed { failed_at },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PlaybookRunStatusDto {
    Running,
    Success {
        #[serde(rename = "finishedAt")]
        finished_at: DateTime<Utc>,
    },
    #[serde(rename_all = "camelCase")]
    Failed {
        #[serde(rename = "finishedAt")]
        finished_at: DateTime<Utc>,
        #[serde(rename = "stepSeq")]
        step_seq: u32,
        message: String,
    },
}

impl From<PlaybookRunStatus> for PlaybookRunStatusDto {
    fn from(value: PlaybookRunStatus) -> Self {
        match value {
            PlaybookRunStatus::Running => Self::Running,
            PlaybookRunStatus::Success { finished_at } => Self::Success { finished_at },
            PlaybookRunStatus::Failed {
                finished_at,
                step_seq,
                message,
            } => Self::Failed {
                finished_at,
                step_seq,
                message,
            },
        }
    }
}
