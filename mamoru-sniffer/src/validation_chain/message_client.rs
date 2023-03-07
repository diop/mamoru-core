pub use crate::validation_chain::proto::validation_chain::{
    chain::ChainType, Block as BlockId, DaemonMetadataType, IncidentSeverity,
    MsgCreateDaemonMetadataResponse, MsgRegisterDaemonResponse, MsgRegisterSnifferResponse,
    MsgReportIncidentResponse, MsgSubscribeDaemonsResponse, MsgUnregisterSnifferResponse,
    Transaction as TransactionId,
};

use crate::errors::ValidationClientError;
use crate::validation_chain::config::MessageClientConfig;
use crate::validation_chain::proto::cosmos::auth::v1beta1::query_client::QueryClient as CosmosQueryClient;
use crate::validation_chain::proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use crate::validation_chain::proto::cosmos::tx::v1beta1::service_client::ServiceClient as CosmosServiceClient;
use crate::validation_chain::proto::cosmos::tx::v1beta1::{BroadcastMode, BroadcastTxRequest};
use crate::validation_chain::proto::cosmos::TxMsgData;
use crate::validation_chain::proto::validation_chain::source::SourceType;
use crate::validation_chain::proto::validation_chain::{
    Chain, CreateDaemonMetadataCommandRequestDto,
    DaemonMetadataContent as ProtoDaemonMetadataContent,
    DaemonMetadataContentQuery as ProtoDaemonMetadataContentQuery, DaemonMetadataContentType,
    DaemonMetadataParemeter, DaemonRegisterCommandRequestDto, DaemonsSubscribeCommandRequestDto,
    IncidentReportCommandRequestDto, MsgCreateDaemonMetadata, MsgRegisterDaemon,
    MsgRegisterSniffer, MsgReportIncident, MsgSubscribeDaemons, MsgUnregisterSniffer,
    SnifferRegisterCommandRequestDto, SnifferUnregisterCommandRequestDto, Source,
};
use crate::validation_chain::{ClientResult, DaemonParameter, DaemonRelay};
use cosmrs::proto::traits::TypeUrl;
use cosmrs::tx::{
    AccountNumber, Body, BodyBuilder, Fee, MessageExt, SequenceNumber, SignDoc, SignerInfo,
};
use cosmrs::{AccountId, Coin};
use prost::Message;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::error;

const MAX_RETRIES: usize = 5;
const RETRY_SLEEP_TIME: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub struct IncidentReport {
    pub daemon_id: String,
    pub source: IncidentSource,
}

/// Safer wrapper over part of [`IncidentReportCommandRequestDto`]
#[derive(Debug)]
pub enum IncidentSource {
    Mempool,
    Transaction {
        // Sui doesn't have blocks at all
        block: Option<BlockId>,
        transaction: TransactionId,
    },
}

/// Safer wrapper over of [`CreateDaemonMetadataCommandRequestDto`]
#[derive(Default)]
pub struct RegisterDaemonMetadataRequest {
    pub kind: DaemonMetadataType,
    pub content: DaemonMetadataContent,
    pub logo_url: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub supported_chains: Vec<ChainType>,
    pub parameters: Vec<DaemonMetadataParemeter>,
}

#[derive(Default)]
pub enum DaemonMetadataContent {
    Sql {
        queries: Vec<DaemonMetadataContentQuery>,
    },
    Wasm {
        module: Vec<u8>,
    },
    #[default]
    Undefined,
}

#[derive(Default)]
pub struct DaemonMetadataContentQuery {
    pub query: String,
    pub incident_message: String,
    pub severity: IncidentSeverity,
}

/// Cache for account data, that is required to specify
/// during transaction signing.
#[derive(Clone, Copy)]
struct AccountDataCache {
    number: AccountNumber,
    sequence: SequenceNumber,
}

impl AccountDataCache {
    /// Loads [`Self`] via Cosmos API
    async fn fetch(
        mut query_client: CosmosQueryClient<tonic::transport::Channel>,
        address: AccountId,
    ) -> ClientResult<Self> {
        let response = query_client
            .account(QueryAccountRequest {
                address: address.to_string(),
            })
            .await?
            .into_inner();

        let account = BaseAccount::decode(&*response.account.expect("Always exists.").value)?;

        Ok(Self {
            number: account.account_number,
            sequence: account.sequence,
        })
    }
}

trait DeserializableMessage: MessageExt + Default + Sized + TypeUrl {}

impl<T: MessageExt + Default + Sized + TypeUrl> DeserializableMessage for T {}

/// High-level wrapper for incident-reporting to Validation Chain.
#[derive(Clone)]
pub struct MessageClient {
    account_data: Arc<Mutex<AccountDataCache>>,
    config: MessageClientConfig,
    service_client: CosmosServiceClient<tonic::transport::Channel>,
    query_client: CosmosQueryClient<tonic::transport::Channel>,
}

impl MessageClient {
    /// Connects to the Validation Chain.
    /// Call [`MessageClientConfig::from_env`] to create `config` parameter from environment variables
    pub async fn connect(config: MessageClientConfig) -> ClientResult<Self> {
        let service_client =
            CosmosServiceClient::connect(config.connection.endpoint.clone()).await?;
        let query_client = CosmosQueryClient::connect(config.connection.endpoint.clone()).await?;
        let account_data = AccountDataCache::fetch(query_client.clone(), config.address()).await?;

        Ok(Self {
            account_data: Arc::new(Mutex::new(account_data)),
            config,
            service_client,
            query_client,
        })
    }

    pub async fn register_sniffer(
        &self,
        chain: ChainType,
    ) -> ClientResult<MsgRegisterSnifferResponse> {
        let sniffer = self.config.address().to_string();
        let chain_vec = vec![Chain {
            chain_type: chain.into(),
        }];

        let mut result = self
            .sign_and_broadcast_txs(vec![MsgRegisterSniffer {
                creator: sniffer.clone(),
                sniffer: Some(SnifferRegisterCommandRequestDto {
                    sniffer,
                    chains: chain_vec,
                }),
            }])
            .await?;

        Ok(result.remove(0))
    }

    pub async fn unregister_sniffer(&self) -> ClientResult<MsgUnregisterSnifferResponse> {
        let sniffer = self.config.address().to_string();

        let mut result = self
            .sign_and_broadcast_txs(vec![MsgUnregisterSniffer {
                creator: sniffer.clone(),
                sniffer: Some(SnifferUnregisterCommandRequestDto { sniffer }),
            }])
            .await?;

        Ok(result.remove(0))
    }

    pub async fn subscribe_daemons(
        &self,
        daemons_ids: Vec<String>,
    ) -> ClientResult<MsgSubscribeDaemonsResponse> {
        let sniffer = self.config.address().to_string();

        let mut result = self
            .sign_and_broadcast_txs(vec![MsgSubscribeDaemons {
                creator: sniffer.clone(),
                daemons: Some(DaemonsSubscribeCommandRequestDto {
                    daemons_ids,
                    sniffer,
                }),
            }])
            .await?;

        Ok(result.remove(0))
    }

    pub async fn report_incidents(
        &self,
        reports: impl IntoIterator<Item = IncidentReport>,
    ) -> ClientResult<Vec<MsgReportIncidentResponse>> {
        let sniffer = self.config.address().to_string();

        let report_messages: Vec<_> = reports
            .into_iter()
            .map(|report| {
                let (source, block, tx) = match report.source {
                    IncidentSource::Mempool => (SourceType::Mempool, None, None),
                    IncidentSource::Transaction { block, transaction } => {
                        (SourceType::Block, block, Some(transaction))
                    }
                };

                MsgReportIncident {
                    creator: sniffer.clone(),
                    incident: Some(IncidentReportCommandRequestDto {
                        daemon_id: report.daemon_id,
                        sniffer: sniffer.clone(),
                        source: Some(Source {
                            source_type: source.into(),
                        }),
                        block,
                        tx,
                        // @TODO: Fill this fields from SQL
                        severity: Default::default(),
                        message: Default::default(),
                        address: Default::default(),
                        data: Default::default(),
                        chain: Default::default(),
                    }),
                }
            })
            .collect();

        let result = self.sign_and_broadcast_txs(report_messages).await?;

        Ok(result)
    }

    pub async fn register_daemon(
        &self,
        daemon_metadata_id: String,
        chain: ChainType,
        parameters: Vec<DaemonParameter>,
        relay: Option<DaemonRelay>,
    ) -> ClientResult<MsgRegisterDaemonResponse> {
        let sniffer = self.config.address().to_string();

        let mut result = self
            .sign_and_broadcast_txs(vec![MsgRegisterDaemon {
                creator: sniffer,
                daemon: Some(DaemonRegisterCommandRequestDto {
                    chain: Some(Chain {
                        chain_type: chain.into(),
                    }),
                    daemon_metadata_id,
                    parameters,
                    relay: Some(relay.unwrap_or(DaemonRelay {
                        r#type: 0,
                        address: "".to_string(),
                        call: "".to_string(),
                    })),
                }),
            }])
            .await?;

        Ok(result.remove(0))
    }

    pub async fn register_daemon_metadata(
        &self,
        request: RegisterDaemonMetadataRequest,
    ) -> ClientResult<MsgCreateDaemonMetadataResponse> {
        let sniffer = self.config.address().to_string();

        let mut result = self
            .sign_and_broadcast_txs(vec![MsgCreateDaemonMetadata {
                creator: sniffer,
                daemon_metadata: Some(CreateDaemonMetadataCommandRequestDto {
                    logo_url: request.logo_url,
                    metadata_type: request.kind as i32,
                    title: request.title,
                    description: request.description,
                    tags: request.tags,
                    supported_chains: request
                        .supported_chains
                        .into_iter()
                        .map(|chain_type| Chain {
                            chain_type: chain_type as i32,
                        })
                        .collect(),
                    parameters: request.parameters,
                    content: Some(match request.content {
                        DaemonMetadataContent::Sql { queries } => ProtoDaemonMetadataContent {
                            r#type: DaemonMetadataContentType::Sql as i32,
                            query: queries
                                .into_iter()
                                .map(|query| ProtoDaemonMetadataContentQuery {
                                    query: query.query,
                                    incident_message: query.incident_message,
                                    severity: query.severity as i32,
                                })
                                .collect(),
                            wasm_module: "".to_string(),
                        },
                        DaemonMetadataContent::Wasm { module } => ProtoDaemonMetadataContent {
                            r#type: DaemonMetadataContentType::Wasm as i32,
                            query: vec![],
                            wasm_module: base64::encode(module),
                        },
                        DaemonMetadataContent::Undefined => {
                            unreachable!("DaemonMetadataContent::Undefined is invalid.");
                        }
                    }),
                }),
            }])
            .await?;

        Ok(result.remove(0))
    }

    /// Unlike queries, commands in Cosmos must be signed and published as transactions.
    /// This method handles transaction signing and ordering ( like setting `sequence_number`).
    async fn sign_and_broadcast_txs<T, R>(
        &self,
        messages: impl IntoIterator<Item = T>,
    ) -> ClientResult<Vec<R>>
    where
        T: Message + TypeUrl,
        R: DeserializableMessage,
    {
        let mut builder = BodyBuilder::new();

        for message in messages.into_iter() {
            builder.msg(message.to_any()?);
        }

        let tx_body = builder.finish();

        let mut account_data = self.account_data.lock().await;
        let mut tx_response_objects = vec![];

        for _ in 0..MAX_RETRIES {
            match self
                .sign_and_broadcast_tx_impl(tx_body.clone(), *account_data)
                .await
            {
                Ok(tx_responses) => {
                    tx_response_objects = tx_responses;

                    break;
                }
                Err(err) => {
                    if err.is_incorrect_account_sequence() {
                        *account_data = AccountDataCache::fetch(
                            self.query_client.clone(),
                            self.config.address(),
                        )
                        .await?;
                    } else {
                        error!("Got unknown error from validation chain: {:?}", err);

                        return Err(err);
                    }
                }
            }

            tokio::time::sleep(RETRY_SLEEP_TIME).await;
        }

        // Assume our app is the only account user
        // Intended to reduce API call rate
        account_data.sequence += 1;

        Ok(tx_response_objects)
    }

    async fn sign_and_broadcast_tx_impl<R>(
        &self,
        tx_body: Body,
        account_data: AccountDataCache,
    ) -> ClientResult<Vec<R>>
    where
        R: DeserializableMessage,
    {
        let AccountDataCache { number, sequence } = account_data;
        let auth_info = SignerInfo::single_direct(Some(self.config.public_key()), sequence)
            .auth_info(Fee::from_amount_and_gas(
                Coin {
                    denom: self.config.token_denominator()?,
                    amount: self.config.tx_fee_amount(),
                },
                self.config.tx_gas_limit(),
            ));

        let chain_id = self.config.chain_id()?;

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, number)
            .map_err(ValidationClientError::CreateSignDoc)?;

        let tx_raw = sign_doc
            .sign(self.config.private_key())
            .map_err(ValidationClientError::SignTransaction)?;

        let response = self
            .service_client
            .clone()
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: tx_raw
                    .to_bytes()
                    .map_err(ValidationClientError::TransactionToBytes)?,
                mode: BroadcastMode::Block.into(),
            })
            .await?
            .into_inner();

        let tx_response = response.tx_response.expect("Always exists.");
        let tx_response_objects = make_responses(&tx_response.data);

        match tx_response.try_into().ok() {
            // If code is an error code, return proper error
            Some(error) => Err(ValidationClientError::CosmosSdkError(error)),
            // Ok otherwise
            None => Ok(tx_response_objects),
        }
    }
}

fn make_responses<R: DeserializableMessage>(data: &str) -> Vec<R> {
    let bytes =
        hex::decode(data).expect("BUG: Cosmos SDK returned invalid non-hex TxResponse::data.");
    let res = TxMsgData::decode(bytes.as_slice())
        .expect("BUG: Cosmos SDK returned non `TxMsgData` in TxResponse::data.");

    res.msg_responses
        .into_iter()
        .map(|resp| R::from_any(&resp).expect("BUG: incompatible type conversion."))
        .collect()
}
