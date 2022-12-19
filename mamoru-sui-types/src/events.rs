use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;

#[derive(BlockchainData)]
#[schema(table_name = "move_events")]
pub struct MoveEvent {
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

#[derive(BlockchainData)]
#[schema(table_name = "publish_events")]
pub struct PublishEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,
}

#[derive(BlockchainData)]
#[schema(table_name = "coin_balance_change_events")]
pub struct CoinBalanceChangeEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub transaction_module: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8")]
    pub change_type: String,

    #[schema(type = "DataType::Utf8", nullable = true)]
    pub owner_address: Option<String>,

    #[schema(type = "DataType::Utf8")]
    pub coin_type: String,

    #[schema(type = "DataType::Utf8")]
    pub coin_object_id: String,

    #[schema(type = "DataType::UInt64")]
    pub version: u64,

    #[schema(type = "DataType::Binary")]
    pub amount: Vec<u8>,
}

#[derive(BlockchainData)]
#[schema(table_name = "epoch_change_events")]
pub struct EpochChangeEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub epoch_id: u64,
}

#[derive(BlockchainData)]
#[schema(table_name = "checkpoint_events")]
pub struct CheckpointEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub checkpoint_seq: u64,
}

#[derive(BlockchainData)]
#[schema(table_name = "transfer_object_events")]
pub struct TransferObjectEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub transaction_module: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8", nullable = true)]
    pub recipient_address: Option<String>,

    #[schema(type = "DataType::Utf8")]
    pub object_type: String,

    #[schema(type = "DataType::Utf8")]
    pub object_id: String,

    #[schema(type = "DataType::UInt64")]
    pub version: u64,
}

#[derive(BlockchainData)]
#[schema(table_name = "mutate_object_events")]
pub struct MutateObjectEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub transaction_module: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8")]
    pub object_type: String,

    #[schema(type = "DataType::Utf8")]
    pub object_id: String,

    #[schema(type = "DataType::UInt64")]
    pub version: u64,
}

#[derive(BlockchainData)]
#[schema(table_name = "delete_object_events")]
pub struct DeleteObjectEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub transaction_module: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8")]
    pub object_id: String,

    #[schema(type = "DataType::UInt64")]
    pub version: u64,
}

#[derive(BlockchainData)]
#[schema(table_name = "new_object_events")]
pub struct NewObjectEvent {
    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub package_id: String,

    #[schema(type = "DataType::Utf8")]
    pub transaction_module: String,

    #[schema(type = "DataType::Utf8")]
    pub sender: String,

    #[schema(type = "DataType::Utf8", nullable = true)]
    pub recipient_address: Option<String>,

    #[schema(type = "DataType::Utf8")]
    pub object_type: String,

    #[schema(type = "DataType::Utf8")]
    pub object_id: String,

    #[schema(type = "DataType::UInt64")]
    pub version: u64,
}
