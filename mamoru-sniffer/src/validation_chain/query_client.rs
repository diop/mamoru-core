pub use crate::validation_chain::{
    proto::{
        cosmos::base::query::v1beta1::PageRequest,
        validation_chain::{
            DaemonMetadata, DaemonParameter, DaemonQueryResponseDto, DaemonRelay,
            IncidentQueryResponseDto, QueryListDaemonsResponse, SnifferQueryResponseDto,
            SnifferStatistic,
        },
    },
    ChainType,
};

use crate::validation_chain::{
    proto::validation_chain::{
        query_client::QueryClient as GeneratedQueryClient, Chain, QueryListDaemonsRequest,
        QueryListIncidentsRequest, QueryListIncidentsResponse, QueryListSnifferStatisticsRequest,
        QueryListSnifferStatisticsResponse, QueryListSniffersRequest, QueryListSniffersResponse,
    },
    ClientResult, QueryClientConfig,
};
use async_stream::try_stream;
use futures::Stream;
use tracing::warn;

const PAGE_SIZE: u64 = 50;

macro_rules! as_stream {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident($($arg:ident: $arg_ty:ty)*) -> impl Stream<Item = ClientResult<$response_type:ty>> { $request_fn:ident, $response_field:ident }
    ) => {
        $(#[$meta])*
        $vis fn $name(&self $(,$arg: $arg_ty)*) -> impl Stream<Item = ClientResult<$response_type>> + '_ {
            try_stream! {
                let mut key = vec![];

                loop {
                    let response = self
                        .$request_fn($($arg,)* PageRequest {
                            key: key.clone(),
                            limit: PAGE_SIZE,
                            ..Default::default()
                        })
                        .await?;

                    for item in response.$response_field {
                        yield item;
                    }

                    let next_key = response.pagination.map(|p| p.next_key).unwrap_or_else(|| {
                        warn!("Missing pagination object for `{}`", stringify!($name));
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
    }
}

/// High-level wrapper for querying Validation Chain data.
#[derive(Clone)]
pub struct QueryClient {
    client: GeneratedQueryClient<tonic::transport::Channel>,
}

impl QueryClient {
    /// Connects to the Validation Chain.
    /// Call [`QueryClientConfig::from_env`] to create `config` parameter from environment variables
    pub async fn connect(config: QueryClientConfig) -> ClientResult<Self> {
        Ok(Self {
            client: GeneratedQueryClient::connect(config.connection.endpoint).await?,
        })
    }

    as_stream! {
        pub fn list_daemons(chain: ChainType) -> impl Stream<Item = ClientResult<DaemonQueryResponseDto>> {
            list_daemons_paginated, daemons
        }
    }

    as_stream! {
        pub fn list_sniffers() -> impl Stream<Item = ClientResult<SnifferQueryResponseDto>> {
            list_sniffers_paginated, sniffers
        }
    }

    as_stream! {
        pub fn list_incidents() -> impl Stream<Item = ClientResult<IncidentQueryResponseDto>> {
            list_incidents_paginated, incidents
        }
    }

    as_stream! {
        pub fn list_statistics() -> impl Stream<Item = ClientResult<SnifferStatistic>> {
            list_statistics_paginated, statistics
        }
    }

    async fn list_daemons_paginated(
        &self,
        chain: ChainType,
        pagination: PageRequest,
    ) -> ClientResult<QueryListDaemonsResponse> {
        let mut client = self.client.clone();

        let response = client
            .list_daemons(QueryListDaemonsRequest {
                pagination: Some(pagination),
                chain: Some(Chain {
                    chain_type: chain.into(),
                }),
            })
            .await?;

        Ok(response.into_inner())
    }

    async fn list_sniffers_paginated(
        &self,
        pagination: PageRequest,
    ) -> ClientResult<QueryListSniffersResponse> {
        let mut client = self.client.clone();

        let response = client
            .list_sniffers(QueryListSniffersRequest {
                pagination: Some(pagination),
            })
            .await?;

        Ok(response.into_inner())
    }

    async fn list_incidents_paginated(
        &self,
        pagination: PageRequest,
    ) -> ClientResult<QueryListIncidentsResponse> {
        let mut client = self.client.clone();

        let response = client
            .list_incidents(QueryListIncidentsRequest {
                pagination: Some(pagination),
            })
            .await?;

        Ok(response.into_inner())
    }

    async fn list_statistics_paginated(
        &self,
        pagination: PageRequest,
    ) -> ClientResult<QueryListSnifferStatisticsResponse> {
        let mut client = self.client.clone();

        let response = client
            .list_sniffer_statistics(QueryListSnifferStatisticsRequest {
                pagination: Some(pagination),
            })
            .await?;

        Ok(response.into_inner())
    }
}
