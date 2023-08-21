use std::{ops::Add, sync::Arc, time::Duration};

use futures::TryStreamExt;
use serde::Deserialize;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::{
    sync::{
        mpsc::{error::TrySendError, Receiver, Sender},
        RwLock,
    },
    time::Instant,
};
use tracing::{debug, error, info, warn};

use mamoru_core::{BlockchainCtx, BlockchainData, Daemon, DataSource};

use crate::statistics_bg_task::{BgStatisticsConfig, StatisticBgTask};
use crate::validation_chain::{BlockId, SourceType, StatisticsReport};
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

    #[serde(default = "SnifferConfig::default_daemons_update_interval_secs")]
    pub daemons_update_interval_secs: u64,

    #[serde(default = "SnifferConfig::default_incidents_send_interval_millis")]
    pub incidents_send_interval_millis: u64,

    #[serde(default = "SnifferConfig::default_max_incident_batch_size")]
    pub max_incident_batch_size: usize,

    #[serde(default = "SnifferConfig::default_statistics_send_interval_secs")]
    pub statistics_send_interval_secs: Option<u64>,

    #[serde(default = "SnifferConfig::default_statistics_buffer_size")]
    pub statistics_buffer_size: usize,
}

impl SnifferConfig {
    pub fn from_env() -> SnifferResult<Self> {
        let config = from_env()?;

        Ok(config)
    }

    pub fn default_incident_buffer_size() -> usize {
        256
    }

    pub fn default_daemons_update_interval_secs() -> u64 {
        120
    }

    pub fn default_incidents_send_interval_millis() -> u64 {
        100
    }

    pub fn default_max_incident_batch_size() -> usize {
        20
    }
    pub fn default_statistics_send_interval_secs() -> Option<u64> {
        None
    }
    pub fn default_statistics_buffer_size() -> usize {
        256
    }
}

pub type SnifferResult<T> = Result<T, SnifferError>;

/// Defines an API for Rule matching and incident reporting.
pub struct Sniffer {
    report_tx: Sender<IncidentReport>,
    rules: Arc<RwLock<Vec<Daemon>>>,
    chain_type: ChainType,

    statistic_tx: Sender<StatisticsReport>,
}

impl Sniffer {
    /// Bootstraps new [`Sniffer`] instance.
    /// Call [`SnifferConfig::from_env`] to create `config` parameter from environment variables.
    pub async fn new(config: SnifferConfig) -> SnifferResult<Self> {
        let rules = Arc::new(RwLock::new(vec![]));
        let (report_tx, report_rx) = tokio::sync::mpsc::channel(config.incident_buffer_size);

        let bg_task_config = BgTaskConfig {
            daemons_update_interval: Duration::from_secs(config.daemons_update_interval_secs),
            incident_send_interval: Duration::from_millis(config.incidents_send_interval_millis),
            max_incident_batch_size: config.max_incident_batch_size,
        };

        let message_client = MessageClient::connect(config.message_config.clone()).await?;

        let bg_task = SnifferBgTask::new(
            message_client.clone(),
            QueryClient::connect(config.query_config).await?,
            Arc::clone(&rules),
            config.chain_type,
            report_rx,
            bg_task_config,
        )
        .await?;

        tokio::spawn(async move { bg_task.run().await });

        let (statistic_tx, statistic_rx) =
            tokio::sync::mpsc::channel(config.statistics_buffer_size);

        let statistics_bg_config = BgStatisticsConfig {
            send_interval_sec: config.statistics_send_interval_secs,
            buffer_size: config.statistics_buffer_size,
        };

        let statistics_bg_task =
            StatisticBgTask::new(message_client, statistic_rx, statistics_bg_config).await;

        tokio::spawn(async move { statistics_bg_task.run().await });

        Ok(Self {
            report_tx,
            rules,
            chain_type: config.chain_type,
            statistic_tx,
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

        if let Some(statistics) = ctx.statistics() {
            self.send_statistic(StatisticsReport {
                source: match ctx.source() {
                    DataSource::Mempool => SourceType::Mempool,
                    DataSource::Block => SourceType::Block,
                },
                blocks: statistics.blocks,
                transactions: statistics.transactions,
                events: statistics.events,
                call_traces: statistics.call_traces,
            });
        }
    }

    fn send_statistic(&self, statistics: StatisticsReport) {
        match self.statistic_tx.try_send(statistics) {
            Ok(_) => {}
            Err(TrySendError::Full(_)) => {
                error!("Statistic channel is full. It may happen because of an event spike or statistic reporting is stuck.");
            }
            Err(TrySendError::Closed(_)) => {
                // This is non-recoverable error, we should panic here.
                panic!("Statistic channel is closed.");
            }
        }
    }
}

struct BgTaskConfig {
    daemons_update_interval: Duration,
    incident_send_interval: Duration,
    max_incident_batch_size: usize,
}

/// An entity to perform slow IO-bound tasks
/// to avoid blocking transaction execution in a blockchain.
struct SnifferBgTask {
    message_client: MessageClient,
    query_client: QueryClient,
    daemons: Arc<RwLock<Vec<Daemon>>>,
    chain_type: ChainType,
    report_rx: Receiver<IncidentReport>,
    task_config: BgTaskConfig,
}

impl SnifferBgTask {
    pub(crate) async fn new(
        message_client: MessageClient,
        query_client: QueryClient,
        daemons: Arc<RwLock<Vec<Daemon>>>,
        chain_type: ChainType,
        report_rx: Receiver<IncidentReport>,
        task_config: BgTaskConfig,
    ) -> SnifferResult<Self> {
        message_client.register_sniffer(chain_type).await?;

        let task = Self {
            message_client,
            query_client,
            daemons,
            chain_type,
            report_rx,
            task_config,
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

        let mut daemons_interval = tokio::time::interval_at(
            Instant::now().add(self.task_config.daemons_update_interval),
            self.task_config.daemons_update_interval,
        );

        let mut incidents_interval = tokio::time::interval_at(
            Instant::now().add(self.task_config.incident_send_interval),
            self.task_config.incident_send_interval,
        );

        loop {
            tokio::select! {
                // it's time to update rules
                _ = daemons_interval.tick() => {
                    if let Err(err) = self.update_daemons().await {
                        error!(error = ?err, "Failed to update rules.")
                    }
                }

                _ = incidents_interval.tick() => {
                    match self.receive_incidents() {
                        Ok(incidents) => {
                            if incidents.is_empty() {
                                continue;
                            }

                            debug!(?incidents, len = incidents.len(), "Reporting incidents...");

                            if let Err(err) = self.message_client.report_incidents(incidents).await {
                                error!(error = ?err, "Failed to report incidents")
                            }
                        }
                        Err(TryRecvError::Disconnected) => {
                            warn!("Reports channel is closed. Stopping the job...");

                            if let Err(err) = self.message_client.unregister_sniffer().await {
                                 error!(error = ?err, "Failed to unregister sniffer")
                            }

                            return;
                        }
                        Err(err) => {
                            error!(error = ?err, "Unknown error while receiving incidents")
                        }
                    }
                }
            }
        }
    }

    /// Receives incidents from the channel up to `max_incident_batch_size`.
    /// Returns `TryRecvError::Disconnected` if the channel is closed.
    fn receive_incidents(&mut self) -> Result<Vec<IncidentReport>, TryRecvError> {
        let mut items = Vec::with_capacity(self.task_config.max_incident_batch_size);

        loop {
            match self.report_rx.try_recv() {
                Ok(item) => items.push(item),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected),
            }

            if items.len() >= self.task_config.max_incident_batch_size {
                break;
            }
        }

        Ok(items)
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
