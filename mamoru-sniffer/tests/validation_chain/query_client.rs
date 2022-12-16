use crate::validation_chain::query_client;
use futures::TryStreamExt;
use mamoru_sniffer::validation_chain::RuleQueryResponseDto;
use test_log::test;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let client = query_client().await;

    let response = client
        .list_rules()
        .try_collect::<Vec<RuleQueryResponseDto>>()
        .await
        .expect("List rules error");

    assert_eq!(response.len(), 0)
}
