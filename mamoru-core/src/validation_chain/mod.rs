mod message_client;
mod proto;
mod query_client;

pub type ClientResult<T> = Result<T, ValidationClientError>;

use crate::errors::ValidationClientError;
pub use message_client::*;
pub use query_client::*;
