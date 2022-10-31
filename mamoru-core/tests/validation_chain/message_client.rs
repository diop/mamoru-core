use crate::init_logger;
use crate::validation_chain::TestAccount;
use mamoru_core::validation_chain::{
    AccountConfig, Block, ChainType, ConnectionConfig, IncidentSource, MessageClient,
    MessageClientConfig, Transaction,
};

#[tokio::test]
#[ignore]
async fn smoke() {
    init_logger();
    let mut client = message_client().await;
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
                block: Block {
                    block_id: "test_block_id".to_string(),
                    hash: "test_block_id".to_string(),
                },
                transaction: Transaction {
                    tx_id: "test_tx_id".to_string(),
                    hash: "test_tx_id".to_string(),
                },
            },
        )
        .await
        .expect("Report incident error");

    client
        .unregister_sniffer()
        .await
        .expect("Unregister sniffer error");
}

async fn message_client() -> MessageClient {
    let sender = TestAccount::new();
    sender.faucet().await.unwrap();

    let TestAccount { key } = sender;

    MessageClient::connect(MessageClientConfig {
        connection: ConnectionConfig::from_env(),
        chain: Default::default(),
        account: AccountConfig { private_key: key },
    })
    .await
    .expect("Connection error")
}
