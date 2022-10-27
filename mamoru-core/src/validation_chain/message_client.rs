pub use crate::validation_chain::proto::validation_chain::{chain::ChainType, Block, Transaction};

use crate::validation_chain::proto::validation_chain::msg_client::MsgClient;
use crate::validation_chain::proto::validation_chain::source::SourceType;
use crate::validation_chain::proto::validation_chain::{
    Chain, IncidentReportCommandRequestDto, MsgRegisterSniffer, MsgReportIncident,
    MsgSubscribeRules, MsgUnregisterSniffer, RulesSubscribeCommandRequestDto,
    SnifferRegisterCommandRequestDto, SnifferUnregisterCommandRequestDto, Source,
};
use crate::validation_chain::ClientResult;
use tonic::codegen::StdError;
use tonic::Request;

pub enum IncidentSource {
    Mempool,
    Transaction {
        block: Block,
        transaction: Transaction,
    },
}

pub struct MessageClient {
    client: MsgClient<tonic::transport::Channel>,
    creator: String,
}

impl MessageClient {
    pub async fn connect<T>(endpoint: T, creator: impl Into<String>) -> ClientResult<Self>
    where
        T: TryInto<tonic::transport::Endpoint>,
        T::Error: Into<StdError>,
    {
        Ok(Self {
            client: MsgClient::connect(endpoint).await?,
            creator: creator.into(),
        })
    }

    pub async fn register_sniffer(&mut self, chain: ChainType) -> ClientResult<()> {
        self.client
            .register_sniffer(Request::new(MsgRegisterSniffer {
                creator: self.creator.clone(),
                sniffer: Some(SnifferRegisterCommandRequestDto {
                    chain: Some(Chain {
                        chain_type: chain.into(),
                    }),
                }),
            }))
            .await?;

        Ok(())
    }

    pub async fn unregister_sniffer(&mut self) -> ClientResult<()> {
        self.client
            .unregister_sniffer(Request::new(MsgUnregisterSniffer {
                creator: self.creator.clone(),
                sniffer: Some(SnifferUnregisterCommandRequestDto {}),
            }))
            .await?;

        Ok(())
    }

    pub async fn subscribe_rules(&mut self, rule_ids: Vec<String>) -> ClientResult<()> {
        self.client
            .subscribe_rules(Request::new(MsgSubscribeRules {
                creator: self.creator.clone(),
                rules: Some(RulesSubscribeCommandRequestDto { rule_ids }),
            }))
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

        self.client
            .report_incident(Request::new(MsgReportIncident {
                creator: self.creator.clone(),
                incident: Some(IncidentReportCommandRequestDto {
                    source: Some(Source {
                        source_type: source.into(),
                    }),
                    rule_id,
                    block,
                    tx,
                }),
            }))
            .await?;

        Ok(())
    }
}
