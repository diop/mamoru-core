pub use crate::validation_chain::proto::validation_chain::{chain::ChainType, Block, Transaction};

use crate::errors::ValidationClientError;
use crate::validation_chain::config::MessageClientConfig;
use crate::validation_chain::proto::cosmos::auth::v1beta1::query_client::QueryClient;
use crate::validation_chain::proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use crate::validation_chain::proto::cosmos::tx::v1beta1::service_client::ServiceClient;
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
use std::time::Duration;

const MAX_RETRIES: usize = 5;
const RETRY_SLEEP_TIME: Duration = Duration::from_millis(100);

/// Safer wrapper over part of [`IncidentReportCommandRequestDto`]
pub enum IncidentSource {
    Mempool,
    Transaction {
        block: Block,
        transaction: Transaction,
    },
}

/// Cache for account data, that is required to specify
/// during transaction signing.
struct AccountDataCache {
    number: AccountNumber,
    sequence: SequenceNumber,
}

impl AccountDataCache {
    /// Loads [`Self`] from Cosmos gRPC API
    async fn fetch(
        query_client: &mut QueryClient<tonic::transport::Channel>,
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

/// High-level wrapper for incident-reporting to Validation Chain.
pub struct MessageClient {
    service_client: ServiceClient<tonic::transport::Channel>,
    query_client: QueryClient<tonic::transport::Channel>,
    config: MessageClientConfig,
    account_data: AccountDataCache,
}

impl MessageClient {
    /// Connect the the Validation Chain.
    /// Call [`MessageClientConfig::from_env`] to create `config` parameter from environment variables
    pub async fn connect(config: MessageClientConfig) -> ClientResult<Self> {
        let service_client = ServiceClient::connect(config.connection.endpoint.clone()).await?;
        let mut query_client = QueryClient::connect(config.connection.endpoint.clone()).await?;
        let account_data = AccountDataCache::fetch(&mut query_client, config.address()).await?;

        Ok(Self {
            service_client,
            query_client,
            config,
            account_data,
        })
    }

    pub async fn register_sniffer(&mut self, chain: ChainType) -> ClientResult<()> {
        self.sign_and_broadcast_tx(MsgRegisterSniffer {
            creator: self.config.address().to_string(),
            sniffer: Some(SnifferRegisterCommandRequestDto {
                chain: Some(Chain {
                    chain_type: chain.into(),
                }),
            }),
        })
        .await?;

        Ok(())
    }

    pub async fn unregister_sniffer(&mut self) -> ClientResult<()> {
        self.sign_and_broadcast_tx(MsgUnregisterSniffer {
            creator: self.config.address().to_string(),
            sniffer: Some(SnifferUnregisterCommandRequestDto {}),
        })
        .await?;

        Ok(())
    }

    pub async fn subscribe_rules(&mut self, rule_ids: Vec<String>) -> ClientResult<()> {
        self.sign_and_broadcast_tx(MsgSubscribeRules {
            creator: self.config.address().to_string(),
            rules: Some(RulesSubscribeCommandRequestDto { rule_ids }),
        })
        .await?;

        Ok(())
    }

    pub async fn report_incident(
        &mut self,
        rule_id: String,
        source: IncidentSource,
    ) -> ClientResult<()> {
        let (source, block, tx) = match source {
            IncidentSource::Mempool => (SourceType::Mempool, None, None),
            IncidentSource::Transaction { block, transaction } => {
                (SourceType::Block, Some(block), Some(transaction))
            }
        };

        self.sign_and_broadcast_tx(MsgReportIncident {
            creator: self.config.address().to_string(),
            incident: Some(IncidentReportCommandRequestDto {
                source: Some(Source {
                    source_type: source.into(),
                }),
                rule_id,
                block,
                tx,
            }),
        })
        .await?;

        Ok(())
    }

    /// Unlike queries, commands in Cosmos must be signed and published as transactions.
    /// This method handles transaction signing and ordering ( like setting `sequence_number`).
    async fn sign_and_broadcast_tx<T: Message + TypeUrl>(
        &mut self,
        message: T,
    ) -> ClientResult<()> {
        let tx_body = BodyBuilder::new().msg(message.to_any()?).finish();

        for _ in 0..MAX_RETRIES {
            match self.sign_and_broadcast_tx_impl(tx_body.clone()).await {
                Ok(_) => break,
                Err(err) => {
                    if err.is_incorrect_account_sequence() {
                        self.account_data =
                            AccountDataCache::fetch(&mut self.query_client, self.config.address())
                                .await?;
                    } else {
                        log::error!("Got unknown error code from validation chain: {}", err);

                        return Err(err);
                    }
                }
            }

            tokio::time::sleep(RETRY_SLEEP_TIME).await;
        }

        // Assume our app is a main account user
        // Intended to reduce API call rate
        self.account_data.sequence += 1;

        Ok(())
    }

    async fn sign_and_broadcast_tx_impl(&mut self, tx_body: Body) -> ClientResult<()> {
        let AccountDataCache { number, sequence } = self.account_data;
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
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: tx_raw
                    .to_bytes()
                    .map_err(ValidationClientError::TransactionToBytes)?,
                mode: BroadcastMode::Sync.into(),
            })
            .await?
            .into_inner();

        let tx_response = response.tx_response.expect("Always exists.");

        log::debug!("{:?}", tx_response);

        match tx_response.code.try_into().ok() {
            // If code is an error code, return proper error
            Some(error) => Err(ValidationClientError::CosmosSdkError(error)),
            // Ok otherwise
            None => Ok(()),
        }
    }
}
