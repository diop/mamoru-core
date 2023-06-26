use std::{collections::HashMap, fmt::Debug};

use crate::blockchain_data::BlockchainData;
use crate::{
    daemon::{assembly_script::AssemblyScriptExecutor, incident::Incident, sql::SqlExecutor},
    BlockchainCtx, DataError, IncidentData,
};

pub mod assembly_script;
pub mod incident;
pub mod sql;

#[derive(Debug)]
pub struct VerifyCtx {
    /// Do daemon found any incidents.
    pub matched: bool,

    /// Incidents generated by the daemon expression.
    pub incidents: Vec<Incident>,
}

/// The parameters that are passed to Daemon.
pub type DaemonParameters = HashMap<String, String>;

#[derive(Debug)]
pub enum Executor {
    Sql(SqlExecutor),
    AssemblyScript(AssemblyScriptExecutor),
}

/// The Daemon entity.
#[derive(Debug)]
pub struct Daemon {
    id: String,
    executor: Executor,
}

impl Daemon {
    pub fn new_sql(
        id: String,
        expression: &str,
        incident_data: IncidentData,
        parameters: DaemonParameters,
    ) -> Result<Self, DataError> {
        let executor = Executor::Sql(SqlExecutor::new(expression, incident_data, parameters)?);

        Ok(Self::new(id, executor))
    }

    pub fn new_assembly_script(
        id: String,
        wasm: impl AsRef<[u8]>,
        parameters: DaemonParameters,
    ) -> Result<Self, DataError> {
        let executor = Executor::AssemblyScript(AssemblyScriptExecutor::new(wasm, parameters)?);

        Ok(Self::new(id, executor))
    }

    pub fn new(id: String, executor: Executor) -> Self {
        Self { id, executor }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Executes the given daemon.
    #[tracing::instrument(skip_all)]
    pub async fn verify<T: BlockchainCtx>(
        &self,
        ctx: &BlockchainData<T>,
    ) -> Result<VerifyCtx, DataError> {
        let incidents = match &self.executor {
            Executor::Sql(sql) => sql.execute(ctx).await?,
            Executor::AssemblyScript(ass) => ass.execute(ctx).await?,
        };

        Ok(VerifyCtx {
            matched: !incidents.is_empty(),
            incidents,
        })
    }
}
