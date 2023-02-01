use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;

// Event represents a contract log event. These events are generated by the LOG opcode and
// stored/indexed by the node.
#[derive(BlockchainData)]
#[schema(table_name = "events")]
pub struct Event {
    // index of the log in the block
    #[schema(type = "DataType::UInt32")]
    pub index: u32,

    // index of the transaction in the block
    #[schema(type = "DataType::UInt32")]
    pub tx_index: u32,

    // hash of the transaction
    #[schema(type = "DataType::Utf8")]
    pub tx_hash: String,

    // block in which the transaction was included
    #[schema(type = "DataType::UInt64")]
    pub block_number: u64,

    // hash of the block in which the transaction was included
    #[schema(type = "DataType::Utf8")]
    pub block_hash: String,

    // address of the contract that generated the event
    #[schema(type = "DataType::Utf8")]
    pub address: String,

    #[schema(type = "DataType::Binary")]
    pub topic0: Vec<u8>,

    #[schema(type = "DataType::Binary")]
    pub topic1: Vec<u8>,

    #[schema(type = "DataType::Binary")]
    pub topic2: Vec<u8>,

    #[schema(type = "DataType::Binary")]
    pub topic3: Vec<u8>,

    #[schema(type = "DataType::Binary")]
    pub topic4: Vec<u8>,

    // supplied by the contract, usually ABI-encoded
    #[schema(type = "DataType::Binary")]
    pub data: Vec<u8>,
}
