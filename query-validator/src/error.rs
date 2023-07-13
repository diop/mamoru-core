use mamoru_core::{DataError, RenderError};

#[derive(thiserror::Error, Debug)]
pub enum ValidateError {
    #[error(transparent)]
    DataError(#[from] DataError),

    #[error("Failed to render SQL: {0}")]
    RenderSql(RenderError),

    #[error("The provided expression matches an empty database. It means the expression would probably match any rule.")]
    MatchesEmptyDatabase,

    #[error("Version {0} is not semver.")]
    InvalidVersion(String),
}
