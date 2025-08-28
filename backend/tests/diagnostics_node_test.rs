use std::sync::Arc;

use backend::action::diagnostics_node::DiagnosticsNode;
use backend::action::metrics_collector_node::MetricsRecord;
use backend::analysis_node::QualityMetrics;
use tokio::sync::mpsc::unbounded_channel;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn diagnostics_attempts_fix_success() {
    let (tx, rx) = unbounded_channel();
    let (_node, mut dev_rx) = DiagnosticsNode::new_with_fix(rx, 1, Arc::new(|| true));

    tx.send(MetricsRecord {
        id: "m1".into(),
        metrics: QualityMetrics {
            credibility: Some(0.1),
            ..Default::default()
        },
    })
    .unwrap();

    sleep(Duration::from_millis(50)).await;
    assert!(dev_rx.try_recv().is_err(), "unexpected developer request");
}

#[tokio::test]
async fn diagnostics_emits_developer_request_on_failed_fix() {
    let (tx, rx) = unbounded_channel();
    let (_node, mut dev_rx) = DiagnosticsNode::new_with_fix(rx, 1, Arc::new(|| false));

    tx.send(MetricsRecord {
        id: "m2".into(),
        metrics: QualityMetrics {
            credibility: Some(0.1),
            ..Default::default()
        },
    })
    .unwrap();

    let req = tokio::time::timeout(Duration::from_millis(100), dev_rx.recv())
        .await
        .expect("developer request expected")
        .expect("developer request");
    assert!(req.description.contains("credibility below threshold"));
}

