use crate::validation_chain::TestAccount;
use mamoru_core::blockchain_data_types::Transaction;
use mamoru_core::validation_chain::{
    AccountConfig, ConnectionConfig, MessageClientConfig, QueryClientConfig,
};
use mamoru_core::{Sniffer, SnifferConfig};
use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use test_log::test;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let time = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut sniffer = sniffer().await;

    sniffer.register().await.expect("Failed to register");
    sniffer
        .update_rules()
        .await
        .expect("Failed to update rules");

    sniffer
        .observe_transaction(
            Transaction::new(42, 43, time, vec![], vec![], HashMap::new()),
            "hash".into(),
        )
        .await
        .expect("Failed to observe transaction");

    sniffer.unregister().await.expect("Failed to unergister");
}

async fn sniffer() -> Sniffer {
    let TestAccount { key } = TestAccount::with_faucet().await;

    Sniffer::new(SnifferConfig {
        message_config: MessageClientConfig {
            connection: ConnectionConfig::from_env(),
            chain: Default::default(),
            account: AccountConfig::new(key),
        },
        query_config: QueryClientConfig {
            connection: ConnectionConfig::from_env(),
        },
        chain_type: Default::default(),
    })
    .await
    .expect("Failed to create Sniffer")
}
