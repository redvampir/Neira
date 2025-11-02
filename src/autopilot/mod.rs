use crate::training::metrics::LearningMetrics;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug)]
pub struct AutoPilot {
    metrics: Arc<LearningMetrics>,
    state: RwLock<AutoPilotState>,
}

#[derive(Debug, Default)]
struct AutoPilotState {
    enabled: bool,
}

#[derive(Debug, Clone)]
pub struct AutoPilotConfig {
    pub check_interval_ms: u64,
    pub min_samples: usize,
}

impl Default for AutoPilotConfig {
    fn default() -> Self {
        Self {
            check_interval_ms: 5000,
            min_samples: 10,
        }
    }
}

impl AutoPilot {
    pub fn new(metrics: Arc<LearningMetrics>) -> Self {
        Self {
            metrics,
            state: RwLock::new(AutoPilotState::default()),
        }
    }

    pub async fn start(&self) {
        let mut state = self.state.write().await;
        state.enabled = true;
        drop(state);

        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                let success_rate = metrics.get_success_rate_metric();
                let current_level = metrics.get_current_level().await;

                info!(
                    success_rate = success_rate,
                    difficulty = current_level,
                    "Learning progress monitored"
                );
            }
        });

        info!("AutoPilot activated");
    }
}
