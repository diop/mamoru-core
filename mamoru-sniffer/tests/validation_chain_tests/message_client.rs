use crate::validation_chain_tests::message_client;
use mamoru_core::{Incident, IncidentSeverity as MamoruIncidentSeverity};
use mamoru_sniffer::validation_chain::{
    BlockId, ChainType, DaemonMetadataContent, DaemonMetadataContentQuery, DaemonMetadataType,
    IncidentReport, IncidentSeverity, IncidentSource, RegisterDaemonMetadataRequest, TransactionId,
};
use test_log::test;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let client = message_client().await;

    client
        .register_sniffer(ChainType::SuiDevnet)
        .await
        .expect("Register sniffer error");

    let daemon_metadata_response = client
        .register_daemon_metadata(RegisterDaemonMetadataRequest {
            kind: DaemonMetadataType::Sole,
            supported_chains: vec![ChainType::SuiDevnet],
            content: DaemonMetadataContent::Sql {
                queries: vec![DaemonMetadataContentQuery {
                    query: "SELECT 1 FROM transactions".to_string(),
                    incident_message: "hello".to_string(),
                    severity: IncidentSeverity::SeverityAlert,
                }],
            },
            ..Default::default()
        })
        .await
        .expect("Register daemon metadata error");

    let register_daemon_response = client
        .register_daemon(
            daemon_metadata_response.daemon_metadata_id,
            ChainType::SuiDevnet,
            vec![],
            None,
        )
        .await
        .expect("Register daemon error.");

    let daemon_ids = vec![register_daemon_response.daemon_id];

    client
        .subscribe_daemons(daemon_ids.clone())
        .await
        .expect("Subscribe rules error");

    let incidents: Vec<_> = daemon_ids
        .into_iter()
        .map(|rule_id| IncidentReport {
            daemon_id: rule_id,
            source: IncidentSource::Transaction {
                transaction: TransactionId {
                    tx_id: "test_tx_id".to_string(),
                    hash: "test_tx_id".to_string(),
                },
                block: Some(BlockId {
                    block_id: "test_block_id".to_string(),
                    hash: "test_block_id".to_string(),
                }),
            },
            chain: ChainType::SuiDevnet,
            incident: Incident {
                severity: MamoruIncidentSeverity::Info,
                message: "Test".to_string(),
                address: "".to_string(),
                data: Default::default(),
            },
        })
        .collect();

    client
        .report_incidents(incidents)
        .await
        .expect("Report incidents error");

    client
        .unregister_sniffer()
        .await
        .expect("Unregister sniffer error");
}
