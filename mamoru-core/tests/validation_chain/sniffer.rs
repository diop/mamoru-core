use crate::rule::data_ctx;
use crate::validation_chain::TestAccount;
use mamoru_core::validation_chain::{
    AccountConfig, ChainType, ConnectionConfig, MessageClientConfig, QueryClientConfig,
};
use mamoru_core::{Sniffer, SnifferConfig};
use test_log::test;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let mut sniffer = sniffer().await;

    sniffer.register().await.expect("Failed to register");
    sniffer
        .update_rules()
        .await
        .expect("Failed to update rules");

    sniffer
        .observe_data(data_ctx("HASH"))
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
        chain_type: ChainType::SuiDevnet,
    })
    .await
    .expect("Failed to create Sniffer")
}
