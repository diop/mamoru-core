use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;

#[derive(BlockchainData)]
#[schema(table_name = "transactions")]
pub struct Transaction {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub block_hash: String,

    #[schema(type = "DataType::Utf8")]
    pub hash: String,

    #[schema(type = "DataType::Utf8")]
    pub event_root_hash: String,

    #[schema(type = "DataType::Utf8")]
    pub state_change_hash: String,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,

    #[schema(type = "DataType::UInt64")]
    pub max_gas_amount: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_unit_price: u64,

    #[schema(type = "DataType::UInt64")]
    pub expiration_timestamp_secs: u64,

    #[schema(type = "DataType::UInt64")]
    pub status: u64,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::UInt64")]
    pub sequence_number: u64,
}
