use std::time::Duration;
use backend::action::diagnostics_node::DiagnosticsNode;
use backend::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use backend::analysis_node::QualityMetrics;
use tokio::time::sleep;

#[tokio::test]
async fn diagnostics_switches_collector_interval() {
    std::env::set_var("METRICS_LOW_INTERVAL_MS", "100");
    std::env::set_var("METRICS_NORMAL_INTERVAL_MS", "10");

    let (metrics, rx) = MetricsCollectorNode::channel();
    let (_diag, _dev_rx, _alert_rx) = DiagnosticsNode::new(rx, 1, metrics.clone());

    assert_eq!(metrics.get_interval_ms(), 10);

    metrics.record(MetricsRecord {
        id: "ok".into(),
        metrics: QualityMetrics {
            credibility: Some(1.0),
            recency_days: None,
            demand: None,
        },
    });
    sleep(Duration::from_millis(20)).await;
    assert_eq!(metrics.get_interval_ms(), 100);

    metrics.record(MetricsRecord {
        id: "bad".into(),
        metrics: QualityMetrics {
            credibility: Some(0.0),
            recency_days: None,
            demand: None,
        },
    });
    sleep(Duration::from_millis(20)).await;
    assert_eq!(metrics.get_interval_ms(), 10);

    metrics.record(MetricsRecord {
        id: "ok2".into(),
        metrics: QualityMetrics {
            credibility: Some(1.0),
            recency_days: None,
            demand: None,
        },
    });
    sleep(Duration::from_millis(20)).await;
    assert_eq!(metrics.get_interval_ms(), 100);

    std::env::remove_var("METRICS_LOW_INTERVAL_MS");
    std::env::remove_var("METRICS_NORMAL_INTERVAL_MS");
}
