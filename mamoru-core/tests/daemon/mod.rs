use mamoru_core::{Daemon, DataError};

mod assembly_script;
mod sql;
mod udf;

pub fn active_daemon(expression: impl AsRef<str>) -> Daemon {
    try_active_daemon(expression).expect("Failed to create daemon.")
}

pub fn try_active_daemon(expression: impl AsRef<str>) -> Result<Daemon, DataError> {
    Daemon::new_sql("dummy".to_string(), 0, i64::MAX, expression.as_ref())
}

pub fn inactive_daemon(expression: impl AsRef<str>) -> Daemon {
    Daemon::new_sql("dummy".to_string(), 0, 0, expression.as_ref())
        .expect("Failed to create daemon.")
}
