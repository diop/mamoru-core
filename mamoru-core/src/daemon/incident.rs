use serde::{Deserialize, Serialize};

/// The incident reported by a Daemon.
#[derive(Debug, Serialize, Deserialize)]
pub struct Incident {
    pub severity: IncidentSeverity,
    pub message: String,
    pub tx_hash: String,
    pub address: String,
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

impl IncidentSeverity {
    pub fn new_from_str(s: &str) -> Option<Self> {
        match s {
            "info" => Some(Self::Info),
            "warning" => Some(Self::Warning),
            "error" => Some(Self::Error),
            "alert" => Some(Self::Alert),
            _ => None,
        }
    }
}
