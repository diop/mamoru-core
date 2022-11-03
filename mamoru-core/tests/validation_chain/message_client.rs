use crate::validation_chain::message_client;
use mamoru_core::validation_chain::{BlockId, ChainType, IncidentSource, TransactionId};
use test_log::test;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let client = message_client().await;
    let rule_id = "test_rule_id".to_string();

    client
        .register_sniffer(ChainType::AptosMainnet)
        .await
        .expect("Register sniffer error");

    client
        .subscribe_rules(vec![rule_id.clone()])
        .await
        .expect("Subscribe rules error");

    client
        .report_incident(
            rule_id,
            IncidentSource::Transaction {
                transaction: TransactionId {
                    tx_id: "test_tx_id".to_string(),
                    hash: "test_tx_id".to_string(),
                },
                block: Some(BlockId {
                    block_id: "test_block_id".to_string(),
                    hash: "test_block_id".to_string(),
                }),
            },
        )
        .await
        .expect("Report incident error");

    client
        .unregister_sniffer()
        .await
        .expect("Unregister sniffer error");
}
