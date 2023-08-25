use playbook_engine::error::ConditionParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid condition: {0}")]
    InvalidCondition(ConditionParseError),

    #[error("Unknown task: {0}")]
    UnknownTask(String),

    #[error("Missing required parameters for task \"{task}\": \"{parameters:?}\"")]
    MissingParameters {
        task: String,
        parameters: Vec<String>,
    },

    #[error("Unknown parameters for task \"{task}\": \"{parameters:?}\"")]
    UnknownParameters {
        task: String,
        parameters: Vec<String>,
    },

    #[error("Invalid playbook: {0}")]
    JsonParse(serde_json::Error),

    #[error("Invalid datetime: {0}")]
    DateTimeParse(String),
}
