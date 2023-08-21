use futures::TryStreamExt;
use test_log::test;

use mamoru_core_test_utils::test_blockchain_data::data_ctx;
use mamoru_sniffer::validation_chain::{ChainType, SnifferStatistic};

use crate::validation_chain_tests::sniffer::random_string;
use crate::validation_chain_tests::{query_client, retry, sniffer, sniffer_with_interval};

#[test(tokio::test)]
#[ignore]
async fn smoke_with_interval() {
    let chain = ChainType::EthTestnet;
    let tx_hash = random_string();

    let sniffer = sniffer_with_interval(chain).await;

    let builder = data_ctx(tx_hash.clone());
    let expected_statistics = &builder.statistics().unwrap();

    sniffer.observe_data(builder).await;

    retry(|| async {
        let statistics = get_statistics().await;

        if statistics.is_empty() {
            return Err(format!("Wrong statistic len: {}", statistics.len()));
        }

        let got_statistics = &statistics[0];

        assert_eq!(
            expected_statistics.blocks, got_statistics.blocks,
            "Wrong statistics blocks number"
        );
        assert_eq!(
            expected_statistics.transactions, got_statistics.transactions,
            "Wrong statistics transactions number"
        );
        assert_eq!(
            expected_statistics.events, got_statistics.events,
            "Wrong statistics events number"
        );
        assert_eq!(
            expected_statistics.call_traces, got_statistics.call_traces,
            "Wrong statistics call_traces number"
        );

        Ok(())
    })
    .await
    .expect("Failed to query statistics.");
}

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let chain = ChainType::EthTestnet;
    let tx_hash = random_string();

    let sniffer = sniffer(chain).await;

    let builder = data_ctx(tx_hash.clone());
    let expected_statistics = &builder.statistics().unwrap();

    sniffer.observe_data(builder).await;

    retry(|| async {
        let statistics = get_statistics().await;

        if statistics.is_empty() {
            return Err(format!("Wrong statistic len: {}", statistics.len()));
        }

        let got_statistics = &statistics[0];

        assert_eq!(
            expected_statistics.blocks, got_statistics.blocks,
            "Wrong statistics blocks number"
        );
        assert_eq!(
            expected_statistics.transactions, got_statistics.transactions,
            "Wrong statistics transactions number"
        );
        assert_eq!(
            expected_statistics.events, got_statistics.events,
            "Wrong statistics events number"
        );
        assert_eq!(
            expected_statistics.call_traces, got_statistics.call_traces,
            "Wrong statistics call_traces number"
        );

        Ok(())
    })
    .await
    .expect("Failed to query statistics.");
}
async fn get_statistics() -> Vec<SnifferStatistic> {
    query_client()
        .await
        .list_statistics()
        .try_collect::<Vec<_>>()
        .await
        .expect("List statistics error")
}
