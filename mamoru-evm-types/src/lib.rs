mod block;
mod call_trace;
mod event;
mod transaction;

pub use block::*;
pub use call_trace::*;
pub use event::*;
pub use transaction::*;

use blockchain_data_macro::tables;
use datafusion::arrow::error::ArrowError;
use mamoru_core::{BlockchainData, TableDef};

pub fn all_tables() -> Result<Vec<TableDef>, ArrowError> {
    tables![
        Transaction,
        TransactionArg,
        CallTrace,
        CallTraceArg,
        Block,
        Event,
        EventTopic,
    ]
}
