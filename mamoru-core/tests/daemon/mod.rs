use mamoru_core::{Daemon, DataError};

mod assembly_script;
mod sql;
mod udf;

pub fn active_daemon(expression: impl AsRef<str>) -> Daemon {
    try_active_daemon(expression).expect("Failed to create daemon.")
}

pub fn try_active_daemon(expression: impl AsRef<str>) -> Result<Daemon, DataError> {
    Daemon::new_sql("dummy".to_string(), expression.as_ref())
}
