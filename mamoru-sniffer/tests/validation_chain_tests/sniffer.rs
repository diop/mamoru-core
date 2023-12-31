use std::future;

use futures::TryStreamExt;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use test_log::test;

use mamoru_core_test_utils::test_blockchain_data::data_ctx;
use mamoru_sniffer::validation_chain::{
    ChainType, DaemonMetadataContent, DaemonMetadataContentQuery, DaemonMetadataType,
    IncidentQueryResponseDto, IncidentSeverity, RegisterDaemonMetadataRequest,
};

use crate::validation_chain_tests::{message_client, query_client, retry, sniffer};

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let chain = ChainType::SuiTestnet;
    let tx_hash = random_string();

    let daemon_id = register_daemon(chain).await;

    let sniffer = sniffer(chain).await;

    let incidents = get_incidents(&daemon_id).await;
    assert!(incidents.is_empty());

    sniffer.observe_data(data_ctx(tx_hash.clone())).await;

    retry(|| async {
        let incidents = get_incidents(&daemon_id).await;

        if incidents.len() != 1 {
            return Err(format!("Wrong incidents len: {}", incidents.len()));
        }

        let incident = &incidents[0];

        assert_eq!(
            incident.block.as_ref().unwrap().hash,
            tx_hash,
            "Wrong incident block hash",
        );

        assert_eq!(
            incident.tx.as_ref().unwrap().hash,
            tx_hash,
            "Wrong incident transaction hash",
        );

        Ok(())
    })
    .await
    .expect("Failed to query incidents.");
}

async fn get_incidents(daemon_id: &str) -> Vec<IncidentQueryResponseDto> {
    query_client()
        .await
        .list_incidents()
        .try_filter(|i| future::ready(i.daemon_id == daemon_id))
        .try_collect::<Vec<_>>()
        .await
        .expect("List incidents error")
}

async fn register_daemon(chain: ChainType) -> String {
    let message_client = message_client().await;

    let daemon_metadata_response = message_client
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

    let register_daemon_response = message_client
        .register_daemon(
            daemon_metadata_response.daemon_metadata_id,
            chain,
            vec![],
            None,
        )
        .await
        .expect("Register daemon error");

    register_daemon_response.daemon_id
}

pub fn random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
