use std::sync::Arc;

use backend::action::diagnostics_node::DiagnosticsNode;
use backend::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use backend::analysis_node::QualityMetrics;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn diagnostics_attempts_fix_success() {
    let (metrics, rx) = MetricsCollectorNode::channel();
    let (_node, mut dev_rx, _alert_rx) =
        DiagnosticsNode::new_with_fix(rx, 1, metrics.clone(), Arc::new(|| true));

    metrics.record(MetricsRecord {
        id: "m1".into(),
        metrics: QualityMetrics {
            credibility: Some(0.1),
            ..Default::default()
        },
    });

    sleep(Duration::from_millis(50)).await;
    assert!(dev_rx.try_recv().is_err(), "unexpected developer request");
}

#[tokio::test]
async fn diagnostics_emits_developer_request_on_failed_fix() {
    let (metrics, rx) = MetricsCollectorNode::channel();
    let (_node, mut dev_rx, _alert_rx) =
        DiagnosticsNode::new_with_fix(rx, 1, metrics.clone(), Arc::new(|| false));

    metrics.record(MetricsRecord {
        id: "m2".into(),
        metrics: QualityMetrics {
            credibility: Some(0.1),
            ..Default::default()
        },
    });

    let req = tokio::time::timeout(Duration::from_millis(100), dev_rx.recv())
        .await
        .expect("developer request expected")
        .expect("developer request");
    assert!(req.description.contains("credibility below threshold"));
}
