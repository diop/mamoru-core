mod config;
mod message_client;
mod proto;
mod query_client;
mod type_urls;
use crate::errors::ValidationClientError;

pub type ClientResult<T> = Result<T, ValidationClientError>;

pub use config::*;
pub use message_client::*;
pub use query_client::*;
