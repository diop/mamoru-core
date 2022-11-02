pub use crate::validation_chain::proto::cosmos::base::query::v1beta1::PageRequest;
pub use crate::validation_chain::proto::validation_chain::QueryListRulesResponse;

use crate::validation_chain::proto::validation_chain::{query_client, QueryListRulesRequest};
use crate::validation_chain::{ClientResult, QueryClientConfig};

/// High-level wrapper for querying Validation Chain data.
pub struct QueryClient {
    client: query_client::QueryClient<tonic::transport::Channel>,
}

impl QueryClient {
    /// Connect the the Validation Chain.
    /// Call [`QueryClientConfig::from_env`] to create `config` parameter from environment variables
    pub async fn connect(config: QueryClientConfig) -> ClientResult<Self> {
        Ok(Self {
            client: query_client::QueryClient::connect(config.connection.endpoint).await?,
        })
    }

    pub async fn list_rules(
        &mut self,
        pagination: PageRequest,
    ) -> ClientResult<QueryListRulesResponse> {
        let response = self
            .client
            .list_rules(QueryListRulesRequest {
                pagination: Some(pagination),
            })
            .await?;

        Ok(response.into_inner())
    }
}
