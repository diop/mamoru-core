mod errors;
mod sniffer;

pub mod validation_chain;
pub use errors::*;
pub use sniffer::*;

fn from_env<T>() -> T
where
    T: serde::de::DeserializeOwned,
{
    envy::prefixed("MAMORU_")
        .from_env()
        .expect("Missing environment variables")
}
