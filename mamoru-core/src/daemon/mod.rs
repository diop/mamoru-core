mod assembly_script;
mod sql;

use crate::daemon::assembly_script::AssemblyScriptExecutor;
use crate::daemon::sql::SqlExecutor;
use crate::{BlockchainDataCtx, DataError};
use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct VerifyCtx {
    /// Do daemon found any incidents.
    pub matched: bool,

    /// Incidents generated by the daemon expression.
    pub incidents: Vec<Incident>,
}

/// The incident found by a Daemon.
#[derive(Debug, Clone)]
pub struct Incident;

/// An entity that can search Incidents in the [`BlockchainDataCtx`].
#[async_trait]
pub trait Executor: Send + Sync + Debug {
    /// Executes the given daemon.
    async fn execute(&self, ctx: &BlockchainDataCtx) -> Result<Vec<Incident>, DataError>;
}

/// The Daemon entity.
#[derive(Debug)]
pub struct Daemon {
    id: String,
    executor: Box<dyn Executor>,
}

impl Daemon {
    pub fn new_sql(id: String, expression: &str) -> Result<Self, DataError> {
        let executor = Box::new(SqlExecutor::new(expression)?);

        Ok(Self::new(id, executor))
    }

    pub fn new_assembly_script(id: String, wasm: impl AsRef<[u8]>) -> Result<Self, DataError> {
        let executor = Box::new(AssemblyScriptExecutor::new(wasm)?);

        Ok(Self::new(id, executor))
    }

    pub fn new(id: String, executor: Box<dyn Executor>) -> Self {
        Self { id, executor }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Executes the given daemon.
    #[tracing::instrument(skip(ctx, self), fields(daemon_id = self.id(), tx_hash = ctx.tx_hash(), level = "trace"))]
    pub async fn verify(&self, ctx: &BlockchainDataCtx) -> Result<VerifyCtx, DataError> {
        let incidents = self.executor.execute(ctx).await?;

        Ok(VerifyCtx {
            matched: !incidents.is_empty(),
            incidents,
        })
    }
}
