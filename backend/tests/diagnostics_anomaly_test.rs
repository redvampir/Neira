use backend::action::diagnostics_node::detect_anomaly;

#[test]
fn detect_anomaly_triggers_alert() {
    let data = vec![1.0, 1.0, 1.0, 10.0];
    let alert = detect_anomaly(&data);
    assert!(alert.is_some(), "expected alert for anomaly");
}
