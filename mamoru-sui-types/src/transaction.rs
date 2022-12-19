use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::{DataType, TimeUnit};

#[derive(BlockchainData)]
#[schema(table_name = "transactions")]
pub struct Transaction {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub digest: String,

    #[schema(type = "DataType::Timestamp(TimeUnit::Second, None)")]
    pub time: i64,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_computation_cost: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_storage_cost: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_budget: u64,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8")]
    pub kind: String,
}
