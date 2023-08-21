use crate::validation_chain::{MessageClient, StatisticsReport};
use serde::Deserialize;
use std::ops::Add;
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use tokio::time::Instant;
use tracing::{debug, error};

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct BgStatisticsConfig {
    #[serde(default = "BgStatisticsConfig::default_send_interval_sec")]
    pub send_interval_sec: Option<u64>,

    #[serde(default = "BgStatisticsConfig::default_buffer_size")]
    pub buffer_size: usize,
}

impl BgStatisticsConfig {
    pub fn default_send_interval_sec() -> Option<u64> {
        None
    }
    pub fn default_buffer_size() -> usize {
        256
    }
}

pub struct StatisticBgTask {
    message_client: MessageClient,
    statistic_rx: Receiver<StatisticsReport>,
    task_config: BgStatisticsConfig,
}

impl StatisticBgTask {
    pub async fn new(
        message_client: MessageClient,
        statistic_rx: Receiver<StatisticsReport>,
        config: BgStatisticsConfig,
    ) -> Self {
        Self {
            message_client,
            statistic_rx,
            task_config: config,
        }
    }

    pub async fn run(self) {
        match self.task_config.send_interval_sec {
            None => self.run_immediately().await,
            Some(interval_secs) => {
                let duration = Duration::from_secs(interval_secs);
                self.run_interval(duration).await;
            }
        };
    }

    async fn run_immediately(mut self) {
        while let Some(statistics) = self.statistic_rx.recv().await {
            debug!(?statistics, "Reporting statistics...");

            // Collecting all following reports to prevent reporting slower than we send to VC
            let mut reports = self.receive_statistic().await.unwrap_or_default();
            reports.push(statistics);

            if let Err(err) = self.message_client.mark_sniffer_statistic(reports).await {
                error!(error = ?err, "Failed to report statistic")
            }
        }
    }

    async fn run_interval(mut self, duration: Duration) {
        let mut interval = tokio::time::interval_at(Instant::now().add(duration), duration);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match self.receive_statistic().await {
                        Ok(statistics) => {
                            if statistics.is_empty() {
                                continue;
                            }
                            debug!(?statistics,len = statistics.len(), "Reporting statistic...");

                            if let Err(err) = self.message_client.mark_sniffer_statistic(statistics).await {
                                error!(error = ?err, "Failed to report statistic")
                            }
                        }
                        Err(err) => {
                            error!(error = ?err, "Unknown error while receiving statistic")
                        }
                    }
                }
            }
        }
    }

    /// Receive statistic from sniffer and send it to Validation Chain.
    async fn receive_statistic(&mut self) -> Result<Vec<StatisticsReport>, TryRecvError> {
        let mut items = Vec::with_capacity(self.task_config.buffer_size);
        loop {
            match self.statistic_rx.try_recv() {
                Ok(statistic) => items.push(statistic),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected),
            }
            if items.len() >= self.task_config.buffer_size {
                break;
            }
        }

        Ok(items)
    }
}
