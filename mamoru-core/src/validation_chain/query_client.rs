pub use crate::validation_chain::proto::cosmos::base::query::v1beta1::PageRequest;
pub use crate::validation_chain::proto::validation_chain::{
    QueryListRulesResponse, RuleQueryResponseDto,
};
use async_stream::try_stream;
use std::sync::Arc;

use crate::validation_chain::proto::validation_chain::query_client::QueryClient as GeneratedQueryClient;
use crate::validation_chain::proto::validation_chain::QueryListRulesRequest;
use crate::validation_chain::{ClientResult, QueryClientConfig};
use futures::Stream;
use tokio::sync::Mutex;
use tracing::warn;

const PAGE_SIZE: u64 = 50;

/// High-level wrapper for querying Validation Chain data.
#[derive(Clone)]
pub struct QueryClient {
    client: Arc<Mutex<GeneratedQueryClient<tonic::transport::Channel>>>,
}

impl QueryClient {
    /// Connect the the Validation Chain.
    /// Call [`QueryClientConfig::from_env`] to create `config` parameter from environment variables
    pub async fn connect(config: QueryClientConfig) -> ClientResult<Self> {
        Ok(Self {
            client: Arc::new(Mutex::new(
                GeneratedQueryClient::connect(config.connection.endpoint).await?,
            )),
        })
    }

    pub fn list_rules(&self) -> impl Stream<Item = ClientResult<RuleQueryResponseDto>> + '_ {
        try_stream! {
            let mut key = vec![];

            loop {
                let response = self
                    .list_rules_paginated(PageRequest {
                        key: key.clone(),
                        limit: PAGE_SIZE,
                        ..Default::default()
                    })
                    .await?;

                for rule in response.rules {
                    yield rule;
                }

                let next_key = response.pagination.map(|p| p.next_key).unwrap_or_else(|| {
                    warn!("Missing pagination object for `list_rules`");

                    vec![]
                });

                if next_key.is_empty() {
                    break;
                } else {
                    key = next_key
                }
            }
        }
    }

    async fn list_rules_paginated(
        &self,
        pagination: PageRequest,
    ) -> ClientResult<QueryListRulesResponse> {
        let mut client = self.client.lock().await;

        let response = client
            .list_rules(QueryListRulesRequest {
                pagination: Some(pagination),
            })
            .await?;

        Ok(response.into_inner())
    }
}
