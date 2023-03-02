use crate::validation_chain_tests::{message_client, query_client, retry};
use futures::TryStreamExt;
use mamoru_sniffer::validation_chain::{
    BlockId, ChainType, DaemonRelay, IncidentReport, IncidentSource, TransactionId,
};
use test_log::test;
use uuid::Uuid;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let client = message_client().await;
    let query = query_client().await;
    let daemon_metadata_id = Uuid::new_v4().to_string();

    client
        .register_sniffer(ChainType::SuiDevnet)
        .await
        .expect("Register sniffer error");

    client
        .register_daemon(
            daemon_metadata_id,
            ChainType::SuiDevnet,
            vec![],
            DaemonRelay {
                r#type: 0,
                call: "call".to_string(),
                address: "address".to_string(),
            },
        )
        .await
        .expect("Register rule error.");

    let daemon_ids: Vec<_> = retry(|| async {
        let daemons = query
            .list_daemons(ChainType::SuiDevnet)
            .try_collect::<Vec<_>>()
            .await
            .expect("List rules error");

        let daemon_ids: Vec<_> = daemons.into_iter().map(|d| d.daemon_id).collect();

        if !daemon_ids.is_empty() {
            Ok(daemon_ids)
        } else {
            Err("Daemons list is empty".to_string())
        }
    })
    .await
    .expect("Failed to query daemons.");

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
