use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;

#[derive(BlockchainData)]
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

    #[schema(type = "DataType::Binary")]
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

    #[schema(type = "DataType::Binary")]
    pub burnt_fees: Vec<u8>,

    #[schema(type = "DataType::UInt32")]
    pub pos_proposed_on_time: u32,

    #[schema(type = "DataType::UInt32")]
    pub pos_slot: u32,

    #[schema(type = "DataType::UInt32")]
    pub pos_epoch: u32,

    #[schema(type = "DataType::UInt32")]
    pub pos_proposer_index: u32,

    #[schema(type = "DataType::Binary")]
    pub pos_slot_root_hash: Vec<u8>,

    #[schema(type = "DataType::UInt32")]
    pub pos_beacon_chain_deposit_count: u32,

    #[schema(type = "DataType::Binary")]
    pub pos_slot_graffiti: Vec<u8>,

    #[schema(type = "DataType::Binary")]
    pub pos_block_randomness: Vec<u8>,

    #[schema(type = "DataType::Binary")]
    pub pos_random_reveal: Vec<u8>,
}
