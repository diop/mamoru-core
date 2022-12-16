mod blockchain_data;
mod errors;
mod rule;
mod sniffer;

pub mod validation_chain;

pub use blockchain_data::{BlockchainData, BlockchainDataCtx, BlockchainDataCtxBuilder};
pub use errors::{DataError, RuleParseError, SnifferError, ValidationClientError};
pub use rule::Rule;
pub use sniffer::{Sniffer, SnifferConfig};

fn from_env<T>() -> T
where
    T: serde::de::DeserializeOwned,
{
    envy::prefixed("MAMORU_")
        .from_env()
        .expect("Missing environment variables")
}
