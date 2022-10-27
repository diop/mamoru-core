use thiserror::Error;

#[derive(Error, Debug)]
pub enum RetrieveValueError {
    #[error("There is no such path as {0}")]
    NoPath(String),
    #[error("This object does not include a list of events")]
    NoEventsInObject,
    #[error("This object does not include a list of calltraces")]
    NoCalltracesInObject,
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonPathParsingError(#[from] jsonpath_lib::JsonPathError),
}

#[derive(Error, Debug)]
pub enum ValidationClientError {
    #[error("Failed to connect to the Validation Chain")]
    Connect(#[from] tonic::transport::Error),

    #[error("Failed to call the Validation Chain API")]
    Request(#[from] tonic::Status),
}
