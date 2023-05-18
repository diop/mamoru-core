use std::{ops::Add, sync::Arc, time::Duration};

use futures::TryStreamExt;
use serde::Deserialize;
use tokio::{
    sync::{
        mpsc::{error::TrySendError, Receiver, Sender},
        RwLock,
    },
    time::Instant,
};
use tracing::{debug, error, info, warn};

use mamoru_core::{BlockchainCtx, BlockchainData, Daemon, DataSource};

use crate::validation_chain::{BlockId, SourceType};
use crate::{
    errors::SnifferError,
    from_env,
    validation_chain::{
        ChainType, DaemonQueryResponseDto, IncidentReport, MessageClient, MessageClientConfig,
        QueryClient, QueryClientConfig, TransactionId,
    },
};

#[derive(Deserialize)]
pub struct SnifferConfig {
    #[serde(flatten)]
    pub message_config: MessageClientConfig,

    #[serde(flatten)]
    pub query_config: QueryClientConfig,

    pub chain_type: ChainType,

    #[serde(default = "SnifferConfig::default_incident_buffer_size")]
    pub incident_buffer_size: usize,

    #[serde(default = "SnifferConfig::default_rules_update_interval_secs")]
    pub rules_update_interval_secs: u64,
}

impl SnifferConfig {
    pub fn from_env() -> SnifferResult<Self> {
        let config = from_env()?;

        Ok(config)
    }

    pub fn default_incident_buffer_size() -> usize {
        256
    }

    pub fn default_rules_update_interval_secs() -> u64 {
        120
    }
}

type SnifferResult<T> = Result<T, SnifferError>;

/// Defines an API for Rule matching and incident reporting.
pub struct Sniffer {
    report_tx: Sender<IncidentReport>,
    rules: Arc<RwLock<Vec<Daemon>>>,
    chain_type: ChainType,
}

impl Sniffer {
    /// Bootstraps new [`Sniffer`] instance.
    /// Call [`SnifferConfig::from_env`] to create `config` parameter from environment variables.
    pub async fn new(config: SnifferConfig) -> SnifferResult<Self> {
        let rules = Arc::new(RwLock::new(vec![]));
        let (report_tx, report_rx) = tokio::sync::mpsc::channel(config.incident_buffer_size);

        let bg_task = SnifferBgTask::new(
            MessageClient::connect(config.message_config).await?,
            QueryClient::connect(config.query_config).await?,
            Arc::clone(&rules),
            config.chain_type,
            report_rx,
            Duration::from_secs(config.rules_update_interval_secs),
        )
        .await?;

        tokio::spawn(async move { bg_task.run().await });

        Ok(Self {
            report_tx,
            rules,
            chain_type: config.chain_type,
        })
    }

    /// Reports to Validation Chain if the provided transaction matches
    /// any rule from the internal storage.
    #[tracing::instrument(
        skip(ctx, self),
        fields(tx = ?ctx.tx(), block = ?ctx.block(), source = ?ctx.source(), level = "debug")
    )]
    pub async fn observe_data<T: BlockchainCtx>(&self, ctx: BlockchainData<T>) {
        let rules = self.rules.read().await;
        let futures: Vec<_> = rules
            .iter()
            .map(|daemon| async {
                let daemon_id = daemon.id();

                match daemon.verify(&ctx).await {
                    Ok(verify_ctx) => {
                        if !verify_ctx.matched {
                            debug!(%daemon_id, "Daemon is NOT matched");
                            return;
                        }

                        info!(%daemon_id, "Daemon is matched");

                        for incident in verify_ctx.incidents {
                            let sent = self.report_tx.try_send(IncidentReport {
                                daemon_id: daemon_id.clone(),
                                source: match ctx.source() {
                                    DataSource::Mempool => SourceType::Mempool,
                                    DataSource::Block => SourceType::Block,
                                },
                                tx: ctx.tx().map(|(tx_id, hash)| TransactionId { tx_id, hash }),
                                block: ctx.block().map(|(block_id, hash)| BlockId { block_id, hash }),
                                chain: self.chain_type,
                                incident,
                            });

                            match sent {
                                Ok(_) => {}
                                Err(TrySendError::Full(_)) => {
                                    error!("Reports channel is full. It may happen because of an event spike or incident reporting is stuck.");

                                    continue;
                                }

                                Err(TrySendError::Closed(_)) => {
                                    // This is non-recoverable error, we should panic here.
                                    panic!("Reports channel is closed.");
                                }
                            }
                        }
                    },
                    Err(err) => {
                        error!(?err, %daemon_id, "Failed to verify daemon, skipping...");
                    }
                }
            })
            .collect();

        futures::future::join_all(futures).await;
    }
}

/// An entity to perform slow IO-bound tasks
/// to avoid blocking transaction execution in a blockchain.
struct SnifferBgTask {
    message_client: MessageClient,
    query_client: QueryClient,
    daemons: Arc<RwLock<Vec<Daemon>>>,
    chain_type: ChainType,
    report_rx: Receiver<IncidentReport>,
    rules_update_interval: Duration,
}

impl SnifferBgTask {
    pub(crate) async fn new(
        message_client: MessageClient,
        query_client: QueryClient,
        daemons: Arc<RwLock<Vec<Daemon>>>,
        chain_type: ChainType,
        report_rx: Receiver<IncidentReport>,
        rules_update_interval: Duration,
    ) -> SnifferResult<Self> {
        message_client.register_sniffer(chain_type).await?;

        let task = Self {
            message_client,
            query_client,
            daemons,
            chain_type,
            report_rx,
            rules_update_interval,
        };

        task.update_daemons().await?;

        Ok(task)
    }

    /// This job:
    /// - updates rules list
    /// - reports incidents received via `report_rx`
    ///
    /// Stops when incidents channel `report_rx` is dropped.
    pub(crate) async fn run(mut self) {
        debug!(
            chain_type = self.chain_type.as_str_name(),
            "Starting sniffer background task"
        );

        let mut rules_interval = tokio::time::interval_at(
            Instant::now().add(self.rules_update_interval),
            self.rules_update_interval,
        );

        loop {
            tokio::select! {
                // it's time to update rules
                _ = rules_interval.tick() => {
                    if let Err(err) = self.update_daemons().await {
                        error!(error = ?err, "Failed to update rules.")
                    }
                }

                // received an update from `report_rx`
                message = self.report_rx.recv() => {
                    match message {
                        Some(report) => {
                            debug!(?report, "Reporting an incident...");

                            if let Err(err) = self.message_client.report_incidents(vec![report]).await {
                                error!(error = ?err, "Failed to report an incident")
                            }
                        }
                        None => {
                            warn!("Reports channel is closed. Stopping the job...");

                            if let Err(err) = self.message_client.unregister_sniffer().await {
                                 error!(error = ?err, "Failed to unregister sniffer")
                            }

                            return;
                        }
                    }

                }
            }
        }
    }

    /// Updates internal daemon storage with daemons from Validation Chain.
    /// Notifies Validation Chain that the sniffer is now subscribed to the new daemons.
    /// Must be called periodically to ensure the sniffer work on relevant daemons.
    /// Emits a log message if some rule is failed to parse.
    async fn update_daemons(&self) -> SnifferResult<()> {
        let daemon_response: Vec<DaemonQueryResponseDto> = self
            .query_client
            .list_daemons(self.chain_type)
            .try_collect()
            .await?;

        debug!(len = daemon_response.len(), "Received rules");

        let new_daemons: Vec<Daemon> = daemon_response
            .into_iter()
            .flat_map(|daemon_response_dto| {
                let daemons: Vec<Daemon> = daemon_response_dto.into();

                daemons
            })
            .collect();

        debug!(len = new_daemons.len(), "Parsed daemons");

        self.message_client
            .subscribe_daemons(new_daemons.iter().map(|rule| rule.id()).collect())
            .await?;

        {
            let mut daemons_guard = self.daemons.write().await;

            *daemons_guard = new_daemons;
        }

        Ok(())
    }
}
