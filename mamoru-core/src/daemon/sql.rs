use datafusion::arrow::array::{Array, AsArray};
use datafusion::arrow::datatypes::DataType;
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
use handlebars::Handlebars;
use lazy_static::lazy_static;
use semver::{Version, VersionReq};
use serde_json::{Map, Value};
use tracing::warn;

use crate::blockchain_data::BlockchainData;
use crate::{
    daemon::{incident::IncidentSeverity, Incident},
    deserialize_data, BlockchainCtx, DaemonParameters, DataError,
};

pub(crate) type SqlQueryOutputs = Vec<Map<String, Value>>;

lazy_static! {
    static ref FEAT_SELECT_REPORTS: VersionReq =
        VersionReq::parse(">=0.1.0").expect("BUG: Failed to parse FEAT_SELECT_REPORTS version");
}

/// SQL daemon executor.
#[derive(Debug)]
pub struct SqlExecutor {
    /// The parsed SQL query.
    query: SqlQuery,
    /// The data that is used for incident reporting for old daemons.
    incident_data: IncidentData,
    /// The mamoru version daemon requires.
    version: Version,
}

/// The data that is used for incident reporting.
#[derive(Debug)]
pub struct IncidentData {
    pub message: String,
    pub severity: IncidentSeverity,
}

impl SqlExecutor {
    pub fn new(
        expression: &str,
        incident_data: IncidentData,
        params: DaemonParameters,
        version: Version,
    ) -> Result<Self, DataError> {
        let expression = substitute_parameters(expression, params)?;

        let query = SqlQuery::new(&expression)?;

        Ok(Self {
            query,
            incident_data,
            version,
        })
    }

    pub async fn execute<T: BlockchainCtx>(
        &self,
        ctx: &BlockchainData<T>,
    ) -> Result<Vec<Incident>, DataError> {
        if FEAT_SELECT_REPORTS.matches(&self.version) {
            let batches = self.query.query(ctx.session().state()).await?;
            let incidents = extract_incidents(batches);

            return Ok(incidents);
        }

        let matches = self.query.matches(ctx.session().state()).await?;

        if matches {
            Ok(vec![Incident {
                severity: self.incident_data.severity.clone(),
                message: self.incident_data.message.clone(),
                address: Default::default(),
                tx_hash: Default::default(),
                data: vec![],
            }])
        } else {
            Ok(vec![])
        }
    }
}

fn substitute_parameters(expression: &str, params: DaemonParameters) -> Result<String, DataError> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    let expression = handlebars
        .render_template(expression, &params)
        .map_err(DataError::RenderSql)?;

    Ok(expression)
}

fn extract_incidents(batches: Vec<RecordBatch>) -> Vec<Incident> {
    batches
        .iter()
        .filter_map(|b| {
            let columns = b.columns();

            if b.num_rows() == 0 || columns.is_empty() {
                return None;
            }

            // we expect reports to be in the first column
            let maybe_reports = &b.columns()[0];

            match maybe_reports.data_type() {
                DataType::Binary => {
                    let incidents = maybe_reports
                        .as_binary::<i32>()
                        .iter()
                        .filter_map(|x| match x {
                            Some(x) => deserialize_data(x).ok(),
                            None => None,
                        })
                        .collect::<Vec<_>>();

                    Some(incidents)
                }
                _ => {
                    warn!("Incident returns something that is not incidents");

                    None
                }
            }
        })
        .flatten()
        .collect()
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
    pub(crate) async fn query_serialize(
        &self,
        state: SessionState,
    ) -> Result<SqlQueryOutputs, DataError> {
        let data = self.query(state).await?;
        let list = record_batches_to_json_rows(&data[..]).map_err(DataError::RecordBatchToJson)?;

        Ok(list)
    }

    /// Returns true if query returns any rows.
    pub(crate) async fn matches(&self, state: SessionState) -> Result<bool, DataError> {
        let data = self.query(state).await?;
        let matches = data.iter().any(|b| b.num_rows() > 0);

        Ok(matches)
    }

    /// Executes the given query against the data context.
    pub(crate) async fn query(&self, state: SessionState) -> Result<Vec<RecordBatch>, DataError> {
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
