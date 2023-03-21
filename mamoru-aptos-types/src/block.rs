use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;

#[derive(BlockchainData)]
#[schema(table_name = "blocks")]
pub struct Block {
    #[schema(type = "DataType::Utf8")]
    pub hash: String,

    #[schema(type = "DataType::UInt64")]
    pub epoch: u64,

    #[schema(type = "DataType::UInt64")]
    pub timestamp_usecs: u64,
}
