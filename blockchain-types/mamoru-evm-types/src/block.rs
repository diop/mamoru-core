use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;
use serde::{Deserialize, Serialize};

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "blocks")]
pub struct Block {
    #[schema(type = "DataType::UInt64")]
    pub block_index: u64,

    #[schema(type = "DataType::Utf8")]
    pub hash: String,

    #[schema(type = "DataType::Utf8")]
    pub parent_hash: String,

    #[schema(type = "DataType::Utf8")]
    pub state_root: String,

    // A nonce is a 64-bit hash which proves (combined with the
    // mix-hash) that a sufficient amount of computation has been carried
    // out on a block.
    #[schema(type = "DataType::UInt64")]
    pub nonce: u64,

    #[schema(type = "DataType::Utf8")]
    pub status: String,

    #[schema(type = "DataType::UInt64")]
    pub timestamp: u64,

    #[schema(type = "DataType::LargeBinary")]
    pub block_reward: Vec<u8>,

    #[schema(type = "DataType::Utf8")]
    pub fee_recipient: String,

    #[schema(type = "DataType::UInt64")]
    pub total_difficulty: u64,

    #[schema(type = "DataType::Float64")]
    pub size: f64,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_limit: u64,
}
