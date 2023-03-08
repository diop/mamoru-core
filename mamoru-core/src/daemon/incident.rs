use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The incident reported by a Daemon.
#[derive(Debug, Serialize, Deserialize)]
pub struct Incident {
    pub severity: IncidentSeverity,
    pub message: String,

    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub data: IncidentDataStruct,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    Info,
    Warning,
    Error,
    Alert,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncidentDataStruct {
    #[serde(flatten)]
    fields: HashMap<String, IncidentDataValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum IncidentDataValue {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
    Struct(Box<IncidentDataStruct>),
    List(Vec<IncidentDataValue>),
}

impl IncidentDataStruct {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn insert(&mut self, field: String, value: IncidentDataValue) -> Option<IncidentDataValue> {
        self.fields.insert(field, value)
    }

    pub fn fields(self) -> HashMap<String, IncidentDataValue> {
        self.fields
    }
}

impl Default for IncidentDataStruct {
    fn default() -> Self {
        Self::new()
    }
}
