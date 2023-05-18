use std::collections::HashMap;
use std::sync::Arc;

use datafusion::{
    arrow::{datatypes::Schema, error::ArrowError, record_batch::RecordBatch},
    prelude::SessionContext,
};

use crate::DataError;

pub mod serialize;
pub mod value;

mod udf;

/// Represents blockchain-specific data entity that is
/// going to be inserted into Apache Arrow and then queried.
///
/// The trait can be implemented manually or with `blockchain-data-derive` crate.
pub trait BlockchainTableItem {
    /// The table name visible in Arrow.
    fn table_name(&self) -> &'static str;

    /// The schema.
    fn schema(&self) -> Arc<Schema>;

    /// Converts the struct data to Arrow data type [`RecordBatch`].
    fn to_record_batch(self: Box<Self>) -> Result<RecordBatch, ArrowError>;
}

/// Represents blockchain-specific data context.
pub trait BlockchainCtx: Sync + Send + 'static {
    /// Creates an empty instance of the context.
    fn empty() -> Self;

    /// The module name that will be used as an env for imports.
    fn module() -> &'static str;

    /// Blockchain-specific imports that will be called from WASM.
    /// Put all getters here.
    fn imports() -> BlockchainSpecificImports<Self>
    where
        Self: Sized;

    /// All tables that will be inserted into Arrow.
    fn as_tables(&self) -> Vec<Box<dyn BlockchainTableItem>>;
}

/// A function that will be called from WASM to retrieve data.
///
/// The functions are expected to return data serialized with [`serialize::serialize_data`] function.
/// They could return `impl Serialize` instead, but it's not supported by Rust yet :(.
pub enum CtxImportFn<T> {
    /// Function that takes no arguments.
    NoArgs(fn(&T) -> Vec<u8>),

    /// Function that takes a single `u64` argument.
    /// Intended to be used for retrieving data by ID.
    ById(fn(&T, u64) -> Result<&[u8], CtxImportError>),
}

#[derive(thiserror::Error, Debug)]
pub enum CtxImportError {
    #[error("Failed to retrieve data: {0}")]
    ById(String),
}

pub type BlockchainSpecificImports<T> = HashMap<&'static str, CtxImportFn<T>>;

pub struct BlockchainDataBuilder<T> {
    data: T,
    source: DataSource,
    tx: Option<(Id, Hash)>,
    block: Option<(Id, Hash)>,
}

impl<T: BlockchainCtx> Default for BlockchainDataBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub type TableDef = (&'static str, RecordBatch);

pub type Id = String;
pub type Hash = String;

#[derive(Copy, Clone, Debug)]
pub enum DataSource {
    Mempool,
    Block,
}

impl<T: BlockchainCtx> BlockchainDataBuilder<T> {
    pub fn new() -> Self {
        Self {
            data: T::empty(),
            source: DataSource::Block,
            tx: None,
            block: None,
        }
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn set_mempool_source(&mut self) {
        self.source = DataSource::Mempool;
    }

    pub fn set_tx_data(&mut self, tx_id: impl Into<Id>, tx_hash: impl Into<Hash>) {
        self.tx = Some((tx_id.into(), tx_hash.into()));
    }

    pub fn set_block_data(&mut self, block_id: impl Into<Id>, block_hash: impl Into<Hash>) {
        self.block = Some((block_id.into(), block_hash.into()));
    }

    pub fn build(self) -> Result<BlockchainData<T>, DataError> {
        let session = setup_session();

        for table in self.data.as_tables() {
            self.add_to_session(table, &session)?;
        }

        Ok(BlockchainData {
            data: Arc::new(self.data),
            session,
            source: self.source,
            tx: self.tx,
            block: self.block,
        })
    }

    fn add_to_session(
        &self,
        data: Box<dyn BlockchainTableItem>,
        session: &SessionContext,
    ) -> Result<(), DataError> {
        let table_name = data.table_name();
        let record_batch = data
            .to_record_batch()
            .map_err(DataError::CreateRecordBatch)?;

        session
            .register_batch(table_name, record_batch)
            .map_err(DataError::RegisterRecordBatch)?;

        Ok(())
    }
}

/// Holds the blockchain data.
pub struct BlockchainData<T> {
    data: Arc<T>,
    session: SessionContext,
    source: DataSource,
    tx: Option<(Id, Hash)>,
    block: Option<(Id, Hash)>,
}

impl<T> Clone for BlockchainData<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            session: self.session.clone(),
            tx: self.tx.clone(),
            block: self.block.clone(),
            source: self.source,
        }
    }
}

impl<T> BlockchainData<T> {
    pub fn tx(&self) -> Option<(Id, Hash)> {
        self.tx.clone()
    }

    pub fn block(&self) -> Option<(Id, Hash)> {
        self.block.clone()
    }

    pub fn source(&self) -> DataSource {
        self.source
    }

    pub(crate) fn session(&self) -> &SessionContext {
        &self.session
    }

    pub(crate) fn data(&self) -> &T {
        &self.data
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
