pub use crate::validation_chain::proto::cosmos::cosmos::base::query::v1beta1::PageRequest;
pub use crate::validation_chain::proto::validation_chain::QueryListRulesResponse;

use crate::validation_chain::proto::validation_chain::{query_client, QueryListRulesRequest};
use crate::validation_chain::ClientResult;
use tonic::codegen::StdError;
use tonic::Request;

pub struct QueryClient {
    client: query_client::QueryClient<tonic::transport::Channel>,
}

impl QueryClient {
    pub async fn connect<T>(endpoint: T) -> ClientResult<Self>
    where
        T: TryInto<tonic::transport::Endpoint>,
        T::Error: Into<StdError>,
    {
        Ok(Self {
            client: query_client::QueryClient::connect(endpoint).await?,
        })
    }

    pub async fn list_rules(
        &mut self,
        pagination: PageRequest,
    ) -> ClientResult<QueryListRulesResponse> {
        let response = self
            .client
            .list_rules(Request::new(QueryListRulesRequest {
                pagination: Some(pagination),
            }))
            .await?;

        Ok(response.into_inner())
    }
}
