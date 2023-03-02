use crate::validation_chain_tests::{message_client, query_client, retry, sniffer};
use futures::TryStreamExt;
use mamoru_core::test_blockchain_data::data_ctx;
use mamoru_sniffer::validation_chain::{
    ChainType, DaemonParameter, DaemonQueryResponseDto, DaemonRelay, IncidentQueryResponseDto,
};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::future;
use test_log::test;
use uuid::Uuid;

#[test(tokio::test)]
#[ignore]
async fn smoke() {
    let daemon_metadata_id = Uuid::new_v4().to_string();
    let chain = ChainType::SuiDevnet;
    let tx_hash = random_string();

    let daemon_id = register_daemon(
        daemon_metadata_id,
        chain,
        vec![DaemonParameter {
            key: "key".to_string(),
            value: "value".to_string(),
        }],
        DaemonRelay {
            r#type: 0,
            call: "call".to_string(),
            address: "address".to_string(),
        },
    )
    .await;

    let sniffer = sniffer(chain).await;

    let incidents = get_incidents(&daemon_id).await;
    assert!(incidents.is_empty());

    sniffer.observe_data(data_ctx(tx_hash.clone())).await;

    retry(|| async {
        let incidents = get_incidents(&daemon_id).await;

        if incidents.len() == 1 {
            Ok(())
        } else {
            Err(format!("Wrong incidents len: {}", incidents.len()))
        }
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

async fn register_daemon(
    daemon_metadata_id: String,
    chain: ChainType,
    parameters: Vec<DaemonParameter>,
    relay: DaemonRelay,
) -> String {
    let message_client = message_client().await;
    let old_daemons = daemons(chain).await;

    message_client
        .register_daemon(daemon_metadata_id, chain, parameters, relay)
        .await
        .expect("Register rule error");

    retry(|| async {
        let mut new_daemons = daemons(chain).await;
        new_daemons.retain(|x| !old_daemons.contains(x));

        if let Some(daemon) = new_daemons.first() {
            Ok(daemon.daemon_id.clone())
        } else {
            Err("Failed to find newly created daemon.".to_string())
        }
    })
    .await
    .expect("Failed to query daemons.")
}

async fn daemons(chain: ChainType) -> Vec<DaemonQueryResponseDto> {
    let query_client = query_client().await;

    query_client
        .list_daemons(chain)
        .try_collect()
        .await
        .expect("List daemons error")
}

fn random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
