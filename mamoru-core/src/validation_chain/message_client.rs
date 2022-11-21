pub use crate::validation_chain::proto::validation_chain::{
    chain::ChainType, Block as BlockId, Transaction as TransactionId,
};

use crate::errors::ValidationClientError;
use crate::validation_chain::config::MessageClientConfig;
use crate::validation_chain::proto::cosmos::auth::v1beta1::query_client::QueryClient as CosmosQueryClient;
use crate::validation_chain::proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use crate::validation_chain::proto::cosmos::tx::v1beta1::service_client::ServiceClient as CosmosServiceClient;
use crate::validation_chain::proto::cosmos::tx::v1beta1::{BroadcastMode, BroadcastTxRequest};
use crate::validation_chain::proto::validation_chain::source::SourceType;
use crate::validation_chain::proto::validation_chain::{
    Chain, IncidentReportCommandRequestDto, MsgRegisterSniffer, MsgReportIncident,
    MsgSubscribeRules, MsgUnregisterSniffer, RulesSubscribeCommandRequestDto,
    SnifferRegisterCommandRequestDto, SnifferUnregisterCommandRequestDto, Source,
};
use crate::validation_chain::ClientResult;
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

pub struct IncidentReport {
    pub rule_id: String,
    pub source: IncidentSource,
}

/// Safer wrapper over part of [`IncidentReportCommandRequestDto`]
pub enum IncidentSource {
    Mempool,
    Transaction {
        // Sui doesn't have blocks at all
        block: Option<BlockId>,
        transaction: TransactionId,
    },
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
        query_client: &mut CosmosQueryClient<tonic::transport::Channel>,
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

/// Stateful internal data for the high-level client
struct MessageClientInner {
    service_client: CosmosServiceClient<tonic::transport::Channel>,
    query_client: CosmosQueryClient<tonic::transport::Channel>,
    account_data: AccountDataCache,
}

/// High-level wrapper for incident-reporting to Validation Chain.
#[derive(Clone)]
pub struct MessageClient {
    inner: Arc<Mutex<MessageClientInner>>,
    config: MessageClientConfig,
}

impl MessageClient {
    /// Connect the the Validation Chain.
    /// Call [`MessageClientConfig::from_env`] to create `config` parameter from environment variables
    pub async fn connect(config: MessageClientConfig) -> ClientResult<Self> {
        let service_client =
            CosmosServiceClient::connect(config.connection.endpoint.clone()).await?;
        let mut query_client =
            CosmosQueryClient::connect(config.connection.endpoint.clone()).await?;
        let account_data = AccountDataCache::fetch(&mut query_client, config.address()).await?;

        Ok(Self {
            inner: Arc::new(Mutex::new(MessageClientInner {
                service_client,
                query_client,
                account_data,
            })),
            config,
        })
    }

    pub async fn register_sniffer(&self, chain: ChainType) -> ClientResult<()> {
        self.sign_and_broadcast_txs(vec![MsgRegisterSniffer {
            creator: self.config.address().to_string(),
            sniffer: Some(SnifferRegisterCommandRequestDto {
                chain: Some(Chain {
                    chain_type: chain.into(),
                }),
            }),
        }])
        .await?;

        Ok(())
    }

    pub async fn unregister_sniffer(&self) -> ClientResult<()> {
        self.sign_and_broadcast_txs(vec![MsgUnregisterSniffer {
            creator: self.config.address().to_string(),
            sniffer: Some(SnifferUnregisterCommandRequestDto {}),
        }])
        .await?;

        Ok(())
    }

    pub async fn subscribe_rules(&self, rule_ids: Vec<String>) -> ClientResult<()> {
        self.sign_and_broadcast_txs(vec![MsgSubscribeRules {
            creator: self.config.address().to_string(),
            rules: Some(RulesSubscribeCommandRequestDto { rule_ids }),
        }])
        .await?;

        Ok(())
    }

    pub async fn report_incidents(
        &self,
        reports: impl IntoIterator<Item = IncidentReport>,
    ) -> ClientResult<()> {
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
                    creator: self.config.address().to_string(),
                    incident: Some(IncidentReportCommandRequestDto {
                        source: Some(Source {
                            source_type: source.into(),
                        }),
                        rule_id: report.rule_id,
                        block,
                        tx,
                    }),
                }
            })
            .collect();

        self.sign_and_broadcast_txs(report_messages).await?;

        Ok(())
    }

    /// Unlike queries, commands in Cosmos must be signed and published as transactions.
    /// This method handles transaction signing and ordering ( like setting `sequence_number`).
    async fn sign_and_broadcast_txs<T: Message + TypeUrl>(
        &self,
        messages: impl IntoIterator<Item = T>,
    ) -> ClientResult<()> {
        let mut builder = BodyBuilder::new();

        for message in messages.into_iter() {
            builder.msg(message.to_any()?);
        }

        let tx_body = builder.finish();

        let mut data = self.inner.lock().await;

        for _ in 0..MAX_RETRIES {
            match self
                .sign_and_broadcast_tx_impl(
                    tx_body.clone(),
                    data.account_data,
                    &mut data.service_client,
                )
                .await
            {
                Ok(_) => break,
                Err(err) => {
                    if err.is_incorrect_account_sequence() {
                        data.account_data =
                            AccountDataCache::fetch(&mut data.query_client, self.config.address())
                                .await?;
                    } else {
                        error!("Got unknown error from validation chain: {}", err);

                        return Err(err);
                    }
                }
            }

            tokio::time::sleep(RETRY_SLEEP_TIME).await;
        }

        // Assume our app is a main account user
        // Intended to reduce API call rate
        data.account_data.sequence += 1;

        Ok(())
    }

    async fn sign_and_broadcast_tx_impl(
        &self,
        tx_body: Body,
        account_data: AccountDataCache,
        service_client: &mut CosmosServiceClient<tonic::transport::Channel>,
    ) -> ClientResult<()> {
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

        let response = service_client
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: tx_raw
                    .to_bytes()
                    .map_err(ValidationClientError::TransactionToBytes)?,
                mode: BroadcastMode::Sync.into(),
            })
            .await?
            .into_inner();

        let tx_response = response.tx_response.expect("Always exists.");

        match tx_response.try_into().ok() {
            // If code is an error code, return proper error
            Some(error) => Err(ValidationClientError::CosmosSdkError(error)),
            // Ok otherwise
            None => Ok(()),
        }
    }
}
