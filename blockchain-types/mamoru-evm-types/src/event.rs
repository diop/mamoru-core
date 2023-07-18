use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;
use serde::{Deserialize, Serialize};

// Event represents a contract log event. These events are generated by the LOG opcode and
// stored/indexed by the node.
#[derive(BlockchainData, Clone, Serialize, Deserialize)]
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

    #[schema(type = "DataType::LargeBinary")]
    pub topic0: Vec<u8>,

    #[schema(type = "DataType::LargeBinary")]
    pub topic1: Vec<u8>,

    #[schema(type = "DataType::LargeBinary")]
    pub topic2: Vec<u8>,

    #[schema(type = "DataType::LargeBinary")]
    pub topic3: Vec<u8>,

    #[schema(type = "DataType::LargeBinary")]
    pub topic4: Vec<u8>,

    // supplied by the contract, usually ABI-encoded
    #[schema(type = "DataType::LargeBinary")]
    pub data: Vec<u8>,
}
