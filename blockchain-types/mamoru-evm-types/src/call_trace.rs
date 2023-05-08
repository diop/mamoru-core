use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;
use serde::{Deserialize, Serialize};

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "call_traces")]
pub struct CallTrace {
    #[schema(type = "DataType::UInt32")]
    pub seq: u32,

    #[schema(type = "DataType::UInt32")]
    pub tx_index: u32,

    #[schema(type = "DataType::UInt64")]
    pub block_index: u64,

    #[schema(type = "DataType::UInt32")]
    pub depth: u32,

    #[schema(type = "DataType::Utf8", rename = "type")]
    pub typ: String,

    #[schema(type = "DataType::Utf8")]
    pub from: String,

    #[schema(type = "DataType::Utf8")]
    pub to: String,

    #[schema(type = "DataType::UInt64")]
    pub value: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_limit: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,

    #[schema(type = "DataType::Binary")]
    pub input: Vec<u8>,
}
