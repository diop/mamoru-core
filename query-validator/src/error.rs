use mamoru_core::DataError;

#[derive(thiserror::Error, Debug)]
pub enum ValidateError {
    #[error(transparent)]
    DataError(#[from] DataError),

    #[error("The provided expression matches an empty database. It means the expression would probably match any rule.")]
    MatchesEmptyDatabase,
}
