use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use strum::VariantNames;
use tracing::{error, warn};

use mamoru_core::{Daemon, IncidentData, Version};

use crate::validation_chain::proto::validation_chain::MetadataSdkVersion;
use crate::validation_chain::{
    proto::validation_chain::DaemonMetadataContentType, ChainType, DaemonParameter,
    DaemonQueryResponseDto, IncidentSeverity,
};

pub mod validation_chain {
    pub use super::includes::validationchain::validationchain::*;
}

pub mod cosmos {
    pub use cosmrs::proto::cosmos::*;
}

mod includes {
    // Disable Clippy for autogenerated code.
    #![allow(clippy::all)]

    tonic::include_proto!("includes");
}

impl From<DaemonQueryResponseDto> for Vec<Daemon> {
    fn from(value: DaemonQueryResponseDto) -> Self {
        let metadata = value.daemon_metadata.expect("BUG: Missing DaemonMetadata.");
        let content = metadata
            .content
            .expect("BUG: Missing DaemonMetadataContent.");
        let parameters = make_daemon_parameters(value.parameters);
        let sdk_versions = make_sdk_versions(metadata.sdk_versions);

        match content.r#type() {
            DaemonMetadataContentType::Sql => content
                .query
                .into_iter()
                .filter_map(|query| {
                    let incident_data = IncidentData {
                        message: query.incident_message.clone(),
                        severity: query.severity().into(),
                    };

                    match Daemon::new_sql(
                        value.daemon_id.clone(),
                        &query.query,
                        incident_data,
                        parameters.clone(),
                        sdk_versions.clone(),
                    ) {
                        Ok(daemon) => Some(daemon),
                        Err(err) => {
                            error!(?err, %value.daemon_id, "Failed to parse SQL daemon.");

                            None
                        }
                    }
                })
                .collect(),
            DaemonMetadataContentType::Wasm => {
                let wasm_bytes = match base64::decode(&content.wasm_module) {
                    Ok(wasm_bytes) => wasm_bytes,
                    Err(err) => {
                        error!(?err, %value.daemon_id, "Failed to decode WASM base64-encoded payload.");

                        return vec![];
                    }
                };

                match Daemon::new_assembly_script(
                    value.daemon_id.clone(),
                    wasm_bytes,
                    parameters,
                    sdk_versions,
                ) {
                    Ok(daemon) => vec![daemon],
                    Err(err) => {
                        error!(?err, %value.daemon_id, "Failed to parse WASM daemon.");

                        vec![]
                    }
                }
            }
        }
    }
}

impl From<IncidentSeverity> for mamoru_core::IncidentSeverity {
    fn from(value: IncidentSeverity) -> Self {
        match value {
            IncidentSeverity::SeverityInfo => Self::Info,
            IncidentSeverity::SeverityWarning => Self::Warning,
            IncidentSeverity::SeverityError => Self::Error,
            IncidentSeverity::SeverityAlert => Self::Alert,
        }
    }
}

fn make_sdk_versions(values: Vec<MetadataSdkVersion>) -> HashMap<String, Version> {
    let mut results = HashMap::new();

    for version in values {
        if results.get(&version.sdk).is_some() {
            warn!(sdk = %version.sdk, "Duplicate SDK version name");

            continue;
        }

        match Version::parse(&version.version) {
            Ok(semver) => {
                results.insert(version.sdk, semver);
            }
            Err(err) => {
                warn!(sdk = %version.sdk, ?err, "Failed to parse SDK version");

                continue;
            }
        }
    }

    results
}

fn make_daemon_parameters(values: Vec<DaemonParameter>) -> mamoru_core::DaemonParameters {
    let mut results = mamoru_core::DaemonParameters::new();

    for parameter_dto in values {
        if results.get(&parameter_dto.key).is_some() {
            warn!(key = %parameter_dto.key, "Duplicate daemon parameter key");
        } else {
            results.insert(parameter_dto.key, parameter_dto.value);
        }
    }

    results
}

impl<'de> Deserialize<'de> for ChainType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let chain_name = String::deserialize(deserializer)?;

        ChainType::from_str(&chain_name).map_err(|_| {
            serde::de::Error::custom(format!(
                "Failed to parse chain type. Given: {}, Available: [{}]",
                chain_name,
                ChainType::VARIANTS.join(", ")
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_type_from_str() {
        let chain = ChainType::from_str("SUI_DEVNET").unwrap();

        assert!(matches!(chain, ChainType::SuiDevnet))
    }

    #[derive(Deserialize)]
    struct TestObject {
        chain: ChainType,
    }

    #[test]
    fn chain_type_from_serde() {
        let object: TestObject = serde_json::from_str("{\"chain\":\"SUI_DEVNET\"}").unwrap();

        assert!(matches!(object.chain, ChainType::SuiDevnet))
    }
}
