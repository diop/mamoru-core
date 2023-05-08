use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;
use serde::{Deserialize, Serialize};

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "events")]
pub struct Event {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub transaction_module: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8", rename = "type")]
    pub typ: String,

    #[schema(type = "DataType::Binary")]
    pub contents: Vec<u8>,
}
