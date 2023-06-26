use mamoru_core::{Daemon, DaemonParameters, DataError, IncidentData, IncidentSeverity};

mod assembly_script;
mod sql;
mod udf;

pub fn test_sql_daemon(expression: impl AsRef<str>) -> Daemon {
    try_test_sql_daemon(expression, DaemonParameters::default()).expect("Failed to create daemon.")
}

pub fn try_test_sql_daemon(
    expression: impl AsRef<str>,
    parameters: DaemonParameters,
) -> Result<Daemon, DataError> {
    Daemon::new_sql(
        "dummy".to_string(),
        expression.as_ref(),
        IncidentData {
            message: "Test".to_string(),
            severity: IncidentSeverity::Info,
        },
        parameters,
    )
}
