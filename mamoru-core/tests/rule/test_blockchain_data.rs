use blockchain_data_derive::BlockchainData;
use datafusion::arrow::datatypes::{DataType, TimeUnit};

#[derive(BlockchainData)]
#[schema(table_name = "transactions")]
pub struct TestTransaction {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub hash: String,

    #[schema(type = "DataType::Timestamp(TimeUnit::Second, None)")]
    pub time: i64,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,
}

#[derive(BlockchainData)]
#[schema(table_name = "call_traces")]
pub struct TestCallTrace {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub function: String,
}
