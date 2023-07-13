use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

/// The incident reported by a Daemon.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentV1 {
    pub severity: IncidentSeverity,
    pub message: String,

    #[serde(default)]
    pub tx_hash: String,

    #[serde(default)]
    pub address: String,

    #[serde_as(as = "Base64")]
    #[serde(default)]
    pub data: Vec<u8>,
}

impl From<IncidentV1> for crate::Incident {
    fn from(value: IncidentV1) -> Self {
        Self {
            severity: value.severity.into(),
            message: value.message,
            tx_hash: value.tx_hash,
            address: value.address,
            data: value.data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    Info,
    Warning,
    Error,
    Alert,
}

impl From<IncidentSeverity> for crate::IncidentSeverity {
    fn from(value: IncidentSeverity) -> Self {
        match value {
            IncidentSeverity::Info => Self::Info,
            IncidentSeverity::Warning => Self::Warning,
            IncidentSeverity::Error => Self::Error,
            IncidentSeverity::Alert => Self::Alert,
        }
    }
}
