use std::sync::Arc;

use cosmrs::{
    crypto::{secp256k1, PublicKey},
    tendermint::chain,
    AccountId, Denom,
};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

use crate::{errors::ValidationClientError, from_env_or_fail, validation_chain::ClientResult};

/// The configuration required for creating MessageClient
#[derive(Deserialize, Clone)]
pub struct MessageClientConfig {
    #[serde(flatten)]
    pub connection: ConnectionConfig,

    #[serde(flatten)]
    pub chain: ChainConfig,

    #[serde(flatten)]
    pub account: AccountConfig,

    #[serde(default)]
    pub send_mode: SendMode,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub enum SendMode {
    /// Send a transaction and wait for it to be included in a block.
    /// You will receive a complete tx response.
    /// This mode is useful for testing.
    Block,
    /// Send a transaction and return immediately.
    /// You will receive an empty tx response.
    Sync,
    /// Send a transaction and return immediately. The transaction will be
    /// included in a block asynchronously.
    /// You will receive an empty tx response.
    Async,
}

impl Default for SendMode {
    fn default() -> Self {
        Self::Sync
    }
}

impl MessageClientConfig {
    pub fn from_env() -> Self {
        from_env_or_fail()
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key().public_key()
    }

    pub fn private_key(&self) -> &secp256k1::SigningKey {
        &self.account.private_key
    }

    pub fn address(&self) -> AccountId {
        self.public_key()
            .account_id(&self.chain.account_id_prefix)
            .expect("Never fails with `secp256k1` key.")
    }

    pub fn chain_id(&self) -> ClientResult<chain::Id> {
        self.chain
            .chain_id
            .clone()
            .parse()
            .map_err(ValidationClientError::ParseChainId)
    }

    pub fn account_id_prefix(&self) -> &str {
        &self.chain.account_id_prefix
    }

    pub fn token_denominator(&self) -> ClientResult<Denom> {
        self.chain
            .token_denominator
            .as_str()
            .parse()
            .map_err(ValidationClientError::ParseTokenDenominator)
    }

    pub fn tx_gas_limit(&self) -> u64 {
        self.chain.tx_gas_limit
    }

    pub fn tx_fee_amount(&self) -> u128 {
        self.chain.tx_fee_amount
    }
}

/// The configuration required for creating QueryClient
#[derive(Deserialize, Clone, Debug)]
pub struct QueryClientConfig {
    #[serde(flatten)]
    pub connection: ConnectionConfig,
}

impl QueryClientConfig {
    pub fn from_env() -> Self {
        from_env_or_fail()
    }
}

/// Connection parameters for the Validation Chain API
#[derive(Deserialize, Clone, Debug)]
pub struct ConnectionConfig {
    pub endpoint: String,
}

impl ConnectionConfig {
    pub fn from_env() -> Self {
        from_env_or_fail()
    }
}

/// Cosmos chain-specific configuration
#[derive(Deserialize, Clone, Debug)]
pub struct ChainConfig {
    #[serde(default = "ChainConfig::default_chain_id")]
    pub chain_id: String,
    #[serde(default = "ChainConfig::default_account_id_prefix")]
    pub account_id_prefix: String,
    #[serde(default = "ChainConfig::default_token_denominator")]
    pub token_denominator: String,
    #[serde(
        default = "ChainConfig::default_tx_gas_limit",
        deserialize_with = "u64_from_string"
    )]
    pub tx_gas_limit: u64,
    #[serde(
        default = "ChainConfig::default_tx_fee_amount",
        deserialize_with = "u128_from_string"
    )]
    pub tx_fee_amount: u128,
}

impl ChainConfig {
    fn default_chain_id() -> String {
        "validationchain".to_string()
    }

    fn default_account_id_prefix() -> String {
        "mamoru".to_string()
    }

    fn default_token_denominator() -> String {
        "token".to_string()
    }

    fn default_tx_gas_limit() -> u64 {
        100_000
    }

    fn default_tx_fee_amount() -> u128 {
        0
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain_id: Self::default_chain_id(),
            account_id_prefix: Self::default_account_id_prefix(),
            token_denominator: Self::default_token_denominator(),
            tx_gas_limit: Self::default_tx_gas_limit(),
            tx_fee_amount: Self::default_tx_fee_amount(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct AccountConfig {
    #[serde(deserialize_with = "key_from_base64_bytes")]
    pub private_key: Arc<secp256k1::SigningKey>,
}

impl AccountConfig {
    pub fn new(private_key: secp256k1::SigningKey) -> Self {
        Self {
            private_key: Arc::new(private_key),
        }
    }
}

fn key_from_base64_bytes<'de, D>(deserializer: D) -> Result<Arc<secp256k1::SigningKey>, D::Error>
where
    D: Deserializer<'de>,
{
    let base64_string: String = Deserialize::deserialize(deserializer)?;
    let bytes = base64::decode(base64_string).expect("Can not parse private key base64");
    let key = secp256k1::SigningKey::from_slice(&bytes).expect("Can not parse private key bytes");

    Ok(Arc::new(key))
}

fn u128_from_string<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let the_string: String = Deserialize::deserialize(deserializer)?;
    let val = the_string
        .parse()
        .map_err(|err| SerdeError::custom(format!("Failed to parse number: {}", &err)))?;

    Ok(val)
}

fn u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let the_string: String = Deserialize::deserialize(deserializer)?;
    let val = the_string
        .parse()
        .map_err(|err| SerdeError::custom(format!("Failed to parse number: {}", &err)))?;

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Clone)]
    struct TestConfig {
        #[serde(flatten)]
        nested: ChainConfig,
    }

    #[test]
    fn flatten_number_parsing_works() {
        let data = vec![
            (String::from("TX_FEE_AMOUNT"), String::from("100000")),
            (String::from("TX_GAS_LIMIT"), String::from("100000")),
        ];

        let cfg: TestConfig = envy::from_iter(data).unwrap();

        assert_eq!(cfg.nested.tx_gas_limit, 100_000);
        assert_eq!(cfg.nested.tx_fee_amount, 100_000);
    }
}
