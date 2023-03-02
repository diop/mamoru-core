mod udf;
pub mod value;

use crate::DataError;
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
    fn to_record_batch(self: Box<Self>) -> Result<RecordBatch, ArrowError>;
}

pub struct BlockchainDataCtxBuilder {
    session: SessionContext,
}

impl Default for BlockchainDataCtxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub type TableDef = (&'static str, RecordBatch);

impl BlockchainDataCtxBuilder {
    pub fn new() -> Self {
        Self {
            session: setup_session(),
        }
    }

    pub fn empty<F>(self, all_tables: F) -> Result<BlockchainDataCtx, ArrowError>
    where
        F: FnOnce() -> Result<Vec<TableDef>, ArrowError>,
    {
        let session = self.session;

        for (table, empty_batch) in all_tables()? {
            session.register_batch(table, empty_batch)?;
        }

        Ok(BlockchainDataCtx {
            session,
            tx_id: "EMPTY_CTX".to_string(),
            tx_hash: "EMPTY_CTX".to_string(),
        })
    }

    pub fn add_data(&self, data: Box<dyn BlockchainData>) -> Result<(), DataError> {
        let table_name = data.table_name();
        let record_batch = data
            .to_record_batch()
            .map_err(DataError::CreateRecordBatch)?;

        self.session
            .register_batch(table_name, record_batch)
            .map_err(DataError::RegisterRecordBatch)?;

        Ok(())
    }

    pub fn finish(self, tx_id: String, tx_hash: String) -> BlockchainDataCtx {
        BlockchainDataCtx {
            session: self.session,
            tx_id,
            tx_hash,
        }
    }
}

/// Holds the blockchain data.
#[derive(Clone)]
pub struct BlockchainDataCtx {
    session: SessionContext,
    tx_id: String,
    tx_hash: String,
}

impl BlockchainDataCtx {
    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn tx_hash(&self) -> &str {
        &self.tx_hash
    }

    pub(crate) fn session(&self) -> &SessionContext {
        &self.session
    }
}

fn setup_session() -> SessionContext {
    let session = SessionContext::new();

    session.register_udf(udf::as_boolean());
    session.register_udf(udf::as_uint64());
    session.register_udf(udf::as_string());
    session.register_udf(udf::struct_field());
    session.register_udf(udf::bytes_to_hex());

    session
}
