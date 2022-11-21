mod sniffer;

pub mod blockchain_data_types;
pub mod errors;
pub mod rule;
pub mod rules_engine;
pub mod validation_chain;
pub mod value;

pub mod vendor {
    pub use ethnum;
}

pub use sniffer::*;

fn from_env<T>() -> T
where
    T: serde::de::DeserializeOwned,
{
    envy::prefixed("MAMORU_")
        .from_env()
        .expect("Missing environment variables")
}
