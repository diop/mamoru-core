use crate::init_logger;
use mamoru_core::validation_chain::{
    ConnectionConfig, PageRequest, QueryClient, QueryClientConfig,
};

#[tokio::test]
#[ignore]
async fn list_rules() {
    init_logger();
    let mut client = query_client().await;

    let response = client
        .list_rules(PageRequest {
            ..Default::default()
        })
        .await
        .expect("List rules error");

    assert_eq!(response.rules.len(), 0)
}

async fn query_client() -> QueryClient {
    QueryClient::connect(QueryClientConfig {
        connection: ConnectionConfig::from_env(),
    })
    .await
    .expect("Connection error")
}
