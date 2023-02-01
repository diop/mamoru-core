use blockchain_data_macro::BlockchainData;
use datafusion::arrow::datatypes::DataType;

// Transaction is an Ethereum transaction.
#[derive(BlockchainData)]
#[schema(table_name = "transactions")]
pub struct Transaction {
    #[schema(type = "DataType::UInt32")]
    pub tx_index: u32,

    // tx_hash the transaction hash.
    #[schema(type = "DataType::Utf8")]
    pub tx_hash: String,

    // Type returns the transaction type.
    #[schema(type = "DataType::UInt8", rename = "type")]
    pub typ: u8,

    // Nonce returns the sender account nonce of the transaction.
    #[schema(type = "DataType::UInt64")]
    pub nonce: u64,

    //status code of a transaction. 0 if execution failed. 1 if execution succeeded.
    #[schema(type = "DataType::UInt64")]
    pub status: u64,

    #[schema(type = "DataType::UInt64")]
    pub block_index: u64,

    #[schema(type = "DataType::Utf8")]
    pub from: String,

    // To returns the recipient address of the transaction.
    // For contract-creation transactions, To returns nil.
    #[schema(type = "DataType::Utf8", nullable = true)]
    pub to: Option<String>,

    // Value returns the ether amount of the transaction.
    #[schema(type = "DataType::UInt64")]
    pub value: u64,

    // GasFeeCap returns the fee cap per gas of the transaction.
    #[schema(type = "DataType::UInt64")]
    pub fee: u64,

    // GasPrice returns the gas price of the transaction.
    #[schema(type = "DataType::UInt64")]
    pub gas_price: u64,

    // Gas returns the gas limit of the transaction.
    #[schema(type = "DataType::UInt64")]
    pub gas_limit: u64,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,

    // Data returns the input data of the transaction.
    #[schema(type = "DataType::Binary")]
    pub input: Vec<u8>,

    // Size returns the true RLP encoded storage size of the transaction, either by
    // encoding and returning it, or returning a previously cached value.
    #[schema(type = "DataType::Float64")]
    pub size: f64,
}
