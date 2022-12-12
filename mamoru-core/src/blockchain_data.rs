use crate::DataError;
use chrono::NaiveDateTime;
use datafusion::arrow::datatypes::Schema;
use datafusion::arrow::error::ArrowError;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::prelude::SessionContext;
use std::sync::Arc;

/// Represents blockchain-agnostic data that is
/// going to be inserted into Apache Arrow and then queried.
///
/// The trait can be implemented manually or with `blockchain-data-derive` crate.
pub trait BlockchainData {
    /// The table name visible in Arrow.
    fn table_name(&self) -> &'static str;

    /// The schema.
    fn schema(&self) -> Arc<Schema>;

    /// Converts the struct data to Arrow data type [`RecordBatch`].
    fn to_record_batch(self) -> Result<RecordBatch, ArrowError>;
}

pub struct BlockchainDataCtxBuilder {
    session: SessionContext,
}

impl Default for BlockchainDataCtxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainDataCtxBuilder {
    pub fn new() -> Self {
        Self {
            session: SessionContext::new(),
        }
    }

    pub fn add_data(&self, data: impl BlockchainData) -> Result<(), DataError> {
        let table_name = data.table_name();
        let record_batch = data
            .to_record_batch()
            .map_err(DataError::CreateRecordBatch)?;

        self.session
            .register_batch(table_name, record_batch)
            .map_err(DataError::RegisterRecordBatch)?;

        Ok(())
    }

    pub fn finish(self, tx_id: String, tx_hash: String, now: NaiveDateTime) -> BlockchainDataCtx {
        BlockchainDataCtx {
            session: self.session,
            tx_id,
            tx_hash,
            now,
        }
    }
}

/// Holds the blockchain data.
#[derive(Clone)]
pub struct BlockchainDataCtx {
    session: SessionContext,
    tx_id: String,
    tx_hash: String,
    now: NaiveDateTime,
}

impl BlockchainDataCtx {
    pub(crate) fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub(crate) fn tx_hash(&self) -> &str {
        &self.tx_hash
    }

    pub(crate) fn time(&self) -> NaiveDateTime {
        self.now
    }

    pub(crate) fn session(&self) -> &SessionContext {
        &self.session
    }
}
