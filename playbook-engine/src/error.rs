use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConditionEvalError {
    #[error("Literal \"{literal}\" is not a condition.")]
    LiteralEval { literal: String },

    #[error("Field \"{field}\" is not a condition.")]
    FieldEval { field: String },

    #[error("No such field \"{field}")]
    NoField { field: String },

    #[error("Task with id \"{task}\" is not found.")]
    NoTask { task: String },
}

#[derive(Error, Debug)]
pub enum ConditionParseError {
    #[error("Invalid condition: {condition}, error: {error}")]
    InvalidCondition { condition: String, error: String },
}

#[derive(Error, Debug)]
pub enum ParamsEvalError {
    #[error("Failed to render parameter: {0}")]
    InvalidParameter(handlebars::RenderError),
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("Playbook \"{0}\" is already started")]
    AlreadyStarted(String),

    #[error("Playbook \"{0}\" is not started")]
    NotStarted(String),
}
