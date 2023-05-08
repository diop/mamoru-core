use std::error::Error;
use thiserror::Error;
use wasmer::{CompileError, ExportError, InstantiationError, RuntimeError};

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Failed to create RecordBatch for the internal database.")]
    CreateRecordBatch(datafusion::arrow::error::ArrowError),

    #[error("Failed to covert RecordBatch to json data.")]
    RecordBatchToJson(datafusion::arrow::error::ArrowError),

    #[error("Failed to register RecordBatch in the internal database.")]
    RegisterRecordBatch(datafusion::error::DataFusionError),

    #[error("Failed to parse SQL: {0}")]
    ParseSql(datafusion::sql::sqlparser::parser::ParserError),

    #[error("Failed to plan a query: {0}")]
    PlanQuery(datafusion::error::DataFusionError),

    #[error("Failed to execute query: {0}")]
    ExecuteQuery(datafusion::error::DataFusionError),

    #[error("We only support a single SQL statement.")]
    WrongStatementsNumber,

    #[error("We only support SELECT statements. Queries like INSERT, CREATE TABLE, etc are not allowed.")]
    UnsupportedStatement,

    #[error("Failed to compile WASM module: {0}")]
    WasmCompile(CompileError),

    #[error("Failed to initialize WASM module: {0}")]
    WasmInit(Box<InstantiationError>),

    #[error("Failed to get WASM module export \"{export}\": {source}")]
    WasmExport { source: ExportError, export: String },

    #[error("Failed to execute WASM module: {0}")]
    WasmRuntime(RuntimeError),
}

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("Failed to serialize the value.")]
    Serialize(Box<dyn Error>),

    #[error("Failed to deserialize the value.")]
    Deserialize(Box<dyn Error>),
}
