use std::collections::HashMap;

use typed_builder::TypedBuilder;

use mamoru_core::Version;
use mamoru_core::{Daemon, DaemonParameters, DataError, IncidentData, IncidentSeverity};

mod assembly_script;
mod evm_udf;
mod sql;
mod udf;

pub fn test_sql_daemon(expression: impl AsRef<str>) -> Daemon {
    TestDaemon::builder()
        .expression(expression.as_ref())
        .build()
        .into_daemon()
        .expect("Failed to create daemon.")
}

#[derive(TypedBuilder)]
pub struct TestDaemon {
    #[builder(setter(into))]
    expression: String,

    #[builder(default = "dummy".to_string())]
    id: String,

    #[builder(default = default_incident_data())]
    incident_data: IncidentData,

    #[builder(default = DaemonParameters::default())]
    parameters: DaemonParameters,

    #[builder(default = default_versions())]
    versions: HashMap<String, Version>,
}

fn default_incident_data() -> IncidentData {
    IncidentData {
        message: "Test".to_string(),
        severity: IncidentSeverity::Info,
    }
}

fn default_versions() -> HashMap<String, Version> {
    let mut versions = HashMap::new();
    versions.insert(
        Daemon::MAMORU_VERSION_KEY.to_string(),
        "0.0.0".parse().unwrap(),
    );

    versions
}

impl TestDaemon {
    pub fn set_core_version(mut self, version: Version) -> Self {
        self.versions
            .insert(Daemon::MAMORU_VERSION_KEY.to_string(), version);

        self
    }

    pub fn into_daemon(self) -> Result<Daemon, DataError> {
        Daemon::new_sql(
            self.id,
            &self.expression,
            self.incident_data,
            self.parameters,
            self.versions,
        )
    }
}
