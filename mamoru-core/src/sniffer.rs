use crate::blockchain_data_types::Transaction;
use crate::errors::SnifferError;
use crate::from_env;
use crate::rule::Rule;
use crate::validation_chain::{
    ChainType, IncidentSource, MessageClient, MessageClientConfig, QueryClient, QueryClientConfig,
    RuleQueryResponseDto, TransactionId,
};
use futures::future::join_all;
use futures::TryStreamExt;
use rayon::prelude::*;
use serde::Deserialize;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tracing::{debug, error, span, Level};

#[derive(Deserialize)]
pub struct SnifferConfig {
    #[serde(flatten)]
    pub message_config: MessageClientConfig,

    #[serde(flatten)]
    pub query_config: QueryClientConfig,

    pub chain_type: ChainType,
}

impl SnifferConfig {
    pub fn from_env() -> Self {
        from_env()
    }
}

type SnifferResult<T> = Result<T, SnifferError>;

impl Debug for Sniffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Sniffer {{chain_type = {}}}",
            self.chain_type.as_str_name()
        ))?;

        Ok(())
    }
}

/// Defines an API for Rule matching and incident reporting.
pub struct Sniffer {
    message_client: MessageClient,
    query_client: QueryClient,
    chain_type: ChainType,
    rules: Arc<Vec<Rule>>,
}

impl Sniffer {
    /// Bootstrap new [`Sniffer`] instance.
    /// Call [`SnifferConfig::from_env`] to create `config` parameter from environment variables.
    pub async fn new(config: SnifferConfig) -> SnifferResult<Self> {
        let message_client = MessageClient::connect(config.message_config).await?;
        let query_client = QueryClient::connect(config.query_config).await?;
        let chain_type = config.chain_type;

        Ok(Self {
            message_client,
            query_client,
            chain_type,
            rules: Arc::new(vec![]),
        })
    }

    /// Registers the sniffer on Validation Chain.
    /// Must be called before posting any other data.
    #[tracing::instrument(err)]
    pub async fn register(&mut self) -> SnifferResult<()> {
        debug!(chain_type = self.chain_type.as_str_name());

        self.message_client
            .register_sniffer(self.chain_type)
            .await?;

        Ok(())
    }

    /// Unregisters the sniffer on Validation Chain.
    /// Must be called before shutting down the sniffer node.
    #[tracing::instrument(err)]
    pub async fn unregister(&self) -> SnifferResult<()> {
        self.message_client.unregister_sniffer().await?;

        Ok(())
    }

    /// Updates internal rule storage with rules from Validation Chain.
    /// Notifies Validation Chain that the sniffer is now subscribed to the new rules.
    /// Must be called periodically to ensure the sniffer work on relevant rules.
    /// Emits a log message if some rule is failed to parse.
    #[tracing::instrument(err)]
    pub async fn update_rules(&mut self) -> SnifferResult<()> {
        let rule_response: Vec<RuleQueryResponseDto> =
            self.query_client.list_rules().try_collect().await?;

        debug!(len = rule_response.len(), "Received rules");

        let rules: Vec<Rule> = rule_response
            .into_iter()
            .filter_map(|rule| {
                let rule_id = rule.rule_id.clone();

                match rule.try_into() {
                    Ok(rule) => Some(rule),
                    Err(err) => {
                        error!(?err, %rule_id, "Failed to parse rule, skipping...");

                        None
                    }
                }
            })
            .collect();

        debug!(len = rules.len(), "Parsed rules");

        self.message_client
            .subscribe_rules(rules.iter().map(|rule| rule.id()).collect())
            .await?;

        self.rules = Arc::new(rules);

        Ok(())
    }

    /// Reports to Validation Chain if the provided transaction matches
    /// any rule from the internal storage.
    #[tracing::instrument]
    pub async fn observe_transaction(&self, tx: Transaction, tx_hash: String) {
        let tx = Arc::new(tx);
        let verify_results = self.check_incidents(tx.clone()).await;

        let report_futures: Vec<_> = verify_results
            .into_iter()
            .map(|rule_id| async {
                let result = self
                    .message_client
                    .report_incident(
                        rule_id,
                        IncidentSource::Transaction {
                            block: None,
                            transaction: TransactionId {
                                tx_id: tx.tx_index_string(),
                                hash: tx_hash.clone(),
                            },
                        },
                    )
                    .await;

                result.map_err(|err| {
                    error!(?err, "Failed to report an incident");
                })
            })
            .collect();

        join_all(report_futures).await;
    }

    /// Checks for matches for each of rules available.
    /// Returns a list of Rule ids.
    /// This method doesn't fail as we don't want to break all of our pipeline
    /// because of a single invalid rule.
    async fn check_incidents(&self, tx: Arc<Transaction>) -> Vec<String> {
        let rules = self.rules.clone();

        tokio_rayon::spawn(move || {
            rules
                .par_iter()
                .filter_map(|rule| {
                    let span = span!(
                        Level::INFO,
                        "check_incidents",
                        rule_id = rule.id().as_str(),
                        tx_id = tx.tx_index_string()
                    );
                    let _guard = span.enter();

                    match rule.verify(&tx, None) {
                        Ok(ctx) => {
                            debug!(?rule, "Rule is verified");

                            if ctx.matched() {
                                debug!("Rule is matched");
                                Some(rule.id())
                            } else {
                                debug!("Rule is NOT matched");
                                None
                            }
                        }
                        Err(err) => {
                            error!(?err, "Failed to verify rule, skipping...");
                            None
                        }
                    }
                })
                .collect()
        })
        .await
    }
}
