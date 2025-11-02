use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HealthMonitor {
    connection_stats: RwLock<HashMap<String, ConnectionHealth>>,
    alerts: Arc<AlertSystem>,
}

impl HealthMonitor {
    pub async fn check_connections(&self) -> Vec<HealthIssue> {
        let mut issues = Vec::new();
        let stats = self.connection_stats.read().await;

        for (id, health) in stats.iter() {
            if health.error_rate > 0.1 {
                issues.push(HealthIssue::HighErrorRate(id.clone()));
            }
            if health.latency > Duration::from_millis(100) {
                issues.push(HealthIssue::HighLatency(id.clone()));
            }
        }

        // Уведомляем иммунную систему о проблемах
        if !issues.is_empty() {
            self.alerts.notify(AlertLevel::Warning, &issues).await;
        }

        issues
    }
}
