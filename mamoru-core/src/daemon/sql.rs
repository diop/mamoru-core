use crate::daemon::{Executor, Incident};
use crate::{BlockchainDataCtx, DataError};
use async_trait::async_trait;
use datafusion::arrow::json::writer::record_batches_to_json_rows;
use datafusion::dataframe::DataFrame;
use datafusion::sql::parser::{DFParser, Statement};
use datafusion::sql::sqlparser;
use serde_json::{Map, Value};

pub(crate) type VerifyCtxOutputs = Vec<Map<String, Value>>;

/// Executes SQL queries.
#[derive(Debug)]
pub struct SqlExecutor {
    query: Statement,
}

impl SqlExecutor {
    pub fn new(expression: &str) -> Result<Self, DataError> {
        let query = Self::make_statement(expression)?;

        Ok(Self { query })
    }

    /// Executes the given query against the data context.
    /// Returns json-serializable rows.
    pub(crate) async fn query(
        &self,
        ctx: &BlockchainDataCtx,
    ) -> Result<VerifyCtxOutputs, DataError> {
        let state = ctx.session().state();

        let plan = state
            .statement_to_plan(self.query.clone())
            .await
            .map_err(DataError::PlanQuery)?;

        let data = DataFrame::new(state, plan)
            .collect()
            .await
            .map_err(DataError::ExecuteQuery)?;

        let list = record_batches_to_json_rows(&data[..]).map_err(DataError::RecordBatchToJson)?;

        Ok(list)
    }

    /// Extracts query statements only, as we don't want
    /// someone to call INSERT/UPDATE/CREATE TABLE etc., in the virtual db.
    fn make_statement(expression: &str) -> Result<Statement, DataError> {
        let mut statements = DFParser::parse_sql(expression).map_err(DataError::ParseSql)?;

        if statements.len() != 1 {
            return Err(DataError::WrongStatementsNumber);
        }

        let statement = statements
            .pop_front()
            .expect("A single statement exists, as checked before.");

        let is_query = match &statement {
            Statement::Statement(boxed_statement) => {
                matches!(**boxed_statement, sqlparser::ast::Statement::Query(_))
            }
            _ => false,
        };

        if is_query {
            Ok(statement)
        } else {
            Err(DataError::UnsupportedStatement)
        }
    }
}

#[async_trait]
impl Executor for SqlExecutor {
    async fn execute(&self, ctx: &BlockchainDataCtx) -> Result<Vec<Incident>, DataError> {
        let data = self.query(ctx).await?;

        let mut incidents = vec![];

        if !data.is_empty() {
            incidents.push(Incident);
        }

        Ok(incidents)
    }
}
