use test_log::test;

use mamoru_core::{Incident, IncidentSeverity as MamoruIncidentSeverity};
use mamoru_sniffer::validation_chain::{
    BlockId, ChainType, DaemonMetadataContent, DaemonMetadataContentQuery, DaemonMetadataType,
    IncidentReport, IncidentSeverity, RegisterDaemonMetadataRequest, SourceType, StatisticsReport,
    TransactionId,
};

use crate::validation_chain_tests::message_client;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let client = message_client().await;

    client
        .register_sniffer(ChainType::SuiTestnet)
        .await
        .expect("Register sniffer error");

    let daemon_metadata_response = client
        .register_daemon_metadata(RegisterDaemonMetadataRequest {
            kind: DaemonMetadataType::Sole,
            supported_chains: vec![ChainType::SuiTestnet],
            title: "test".to_string(),
            description: "test".to_string(),
            logo_url: "https://example.com/logo.png".to_string(),
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
            ChainType::SuiTestnet,
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
            source: SourceType::Block,
            tx: Some(TransactionId {
                tx_id: "test_tx_id".to_string(),
                hash: "test_tx_id".to_string(),
            }),
            block: Some(BlockId {
                block_id: "test_block_id".to_string(),
                hash: "test_block_id".to_string(),
            }),
            chain: ChainType::SuiTestnet,
            incident: Incident {
                severity: MamoruIncidentSeverity::Info,
                message: "Test".to_string(),
                address: "".to_string(),
                tx_hash: "tx_hash_from_report".to_string(),
                data: Default::default(),
            },
        })
        .collect();

    client
        .report_incidents(incidents)
        .await
        .expect("Report incidents error");

    let statistic: Vec<_> = vec![StatisticsReport {
        source: SourceType::Mempool,
        blocks: 1,
        transactions: 3,
        events: 3,
        call_traces: 9,
    }];

    client
        .mark_sniffer_statistic(statistic)
        .await
        .expect("Mark sniffer statistic error");

    client
        .unregister_sniffer()
        .await
        .expect("Unregister sniffer error");
}
