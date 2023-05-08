use datafusion::arrow::record_batch::RecordBatch;
use datafusion::execution::context::SessionState;
use datafusion::{
    arrow::json::writer::record_batches_to_json_rows,
    dataframe::DataFrame,
    sql::{
        parser::{DFParser, Statement},
        sqlparser,
    },
};
use serde_json::{Map, Value};

use crate::blockchain_data::BlockchainData;
use crate::{
    daemon::{
        incident::{IncidentDataStruct, IncidentSeverity},
        Incident,
    },
    BlockchainCtx, DataError,
};

pub(crate) type SqlQueryOutputs = Vec<Map<String, Value>>;

/// SQL daemon executor.
#[derive(Debug)]
pub struct SqlExecutor {
    query: SqlQuery,
    incident_data: IncidentData,
}

/// The data that is used for incident reporting.
#[derive(Debug)]
pub struct IncidentData {
    pub message: String,
    pub severity: IncidentSeverity,
}

impl SqlExecutor {
    pub fn new(expression: &str, incident_data: IncidentData) -> Result<Self, DataError> {
        let query = SqlQuery::new(expression)?;

        Ok(Self {
            query,
            incident_data,
        })
    }

    pub async fn execute<T: BlockchainCtx>(
        &self,
        ctx: &BlockchainData<T>,
    ) -> Result<Vec<Incident>, DataError> {
        let matches = self.query.matches(ctx.session().state()).await?;

        if matches {
            Ok(vec![Incident {
                severity: self.incident_data.severity.clone(),
                message: self.incident_data.message.clone(),
                address: "".to_string(),
                data: IncidentDataStruct::new(),
            }])
        } else {
            Ok(vec![])
        }
    }
}

#[derive(Debug)]
pub struct SqlQuery {
    statement: Statement,
}

impl SqlQuery {
    pub fn new(expression: &str) -> Result<Self, DataError> {
        let statement = Self::make_statement(expression)?;

        Ok(Self { statement })
    }

    /// Executes the given query against the data context.
    /// Returns json-serializable rows.
    pub(crate) async fn run(&self, state: SessionState) -> Result<SqlQueryOutputs, DataError> {
        let data = self.query(state).await?;
        let list = record_batches_to_json_rows(&data[..]).map_err(DataError::RecordBatchToJson)?;

        Ok(list)
    }

    pub(crate) async fn matches(&self, state: SessionState) -> Result<bool, DataError> {
        let data = self.query(state).await?;
        let matches = data.iter().any(|b| b.num_rows() > 0);

        Ok(matches)
    }

    async fn query(&self, state: SessionState) -> Result<Vec<RecordBatch>, DataError> {
        let plan = state
            .statement_to_plan(self.statement.clone())
            .await
            .map_err(DataError::PlanQuery)?;

        let data = DataFrame::new(state, plan)
            .collect()
            .await
            .map_err(DataError::ExecuteQuery)?;

        Ok(data)
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
