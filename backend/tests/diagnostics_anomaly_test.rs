use backend::action::diagnostics_node::{DiagnosticsNode, MAX_HISTORY};
use backend::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use backend::analysis_node::QualityMetrics;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn diagnostics_emits_alert_on_anomaly() {
    let (metrics, rx) = MetricsCollectorNode::channel();
    let (_node, _dev_rx, mut alert_rx) = DiagnosticsNode::new(rx, 5, metrics.clone());

    for i in 0..MAX_HISTORY {
        metrics.record(MetricsRecord {
            id: format!("m{}", i),
            metrics: QualityMetrics {
                credibility: Some(1.0),
                ..Default::default()
            },
        });
    }
    metrics.record(MetricsRecord {
        id: format!("m{}", MAX_HISTORY),
        metrics: QualityMetrics {
            credibility: Some(10.0),
            ..Default::default()
        },
    });

    let alert = timeout(Duration::from_millis(100), alert_rx.recv())
        .await
        .expect("alert expected")
        .expect("alert");
    assert!(
        alert.message.contains("deviates"),
        "unexpected alert message"
    );
}
