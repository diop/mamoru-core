use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

/// The incident reported by a Daemon.
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Incident {
    pub severity: IncidentSeverity,
    pub message: String,

    #[serde(default)]
    pub address: String,

    #[serde_as(as = "Base64")]
    #[serde(default)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    Info,
    Warning,
    Error,
    Alert,
}
