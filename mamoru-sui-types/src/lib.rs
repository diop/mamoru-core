mod events;

pub use events::*;

use blockchain_data_derive::BlockchainData;
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

#[derive(BlockchainData)]
#[schema(table_name = "call_traces")]
pub struct CallTrace {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::UInt32")]
    pub depth: u32,

    #[schema(type = "DataType::UInt8")]
    pub call_type: u8,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,

    #[schema(type = "DataType::Utf8", nullable = true)]
    pub transaction_module: Option<String>,

    #[schema(type = "DataType::Utf8")]
    pub function: String,
}

#[derive(BlockchainData)]
#[schema(table_name = "call_trace_type_args")]
pub struct CallTraceTypeArg {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub call_trace_seq: u64,

    #[schema(type = "DataType::Binary")]
    pub arg: Vec<u8>,
}

#[derive(BlockchainData)]
#[schema(table_name = "call_trace_arg_types")]
pub struct CallTraceArgType {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub call_trace_seq: u64,

    #[schema(type = "DataType::Binary")]
    pub arg: Vec<u8>,
}

#[derive(BlockchainData)]
#[schema(table_name = "call_trace_arg_values")]
pub struct CallTraceArgValue {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub call_trace_seq: u64,

    #[schema(type = "DataType::Binary")]
    pub arg: Vec<u8>,
}
