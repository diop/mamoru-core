use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;
use serde::{Deserialize, Serialize};

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "events")]
pub struct Event {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub key: String,

    #[schema(type = "DataType::UInt64")]
    pub sequence_number: u64,

    #[schema(type = "DataType::Utf8", rename = "type")]
    pub typ: String,

    #[schema(type = "DataType::Binary")]
    pub data: Vec<u8>,
}
