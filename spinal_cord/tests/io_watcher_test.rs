/* neira:meta
id: NEI-20240607-io-watcher-test-update
intent: test
summary: Обновлены вызовы IoWatcher::new с токеном остановки.
*/
use std::sync::Arc;
use std::time::Duration;

use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::nervous_system::io_watcher::IoWatcher;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn io_watcher_triggers_diagnostics_on_delay() {
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (_diag, mut dev_rx, _alert_rx) =
        DiagnosticsCell::new_with_fix(rx, 1, metrics.clone(), Arc::new(|| false));

    let watcher = IoWatcher::new(metrics, 1, CancellationToken::new());
    watcher.record_keyboard_latency(Duration::from_millis(5));

    let req = tokio::time::timeout(Duration::from_millis(100), dev_rx.recv())
        .await
        .expect("developer request expected")
        .expect("developer request");
    assert!(req.description.contains("credibility below threshold"));
}

#[tokio::test]
async fn io_watcher_ignores_small_latency() {
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (_diag, mut dev_rx, _alert_rx) =
        DiagnosticsCell::new_with_fix(rx, 1, metrics.clone(), Arc::new(|| false));

    let watcher = IoWatcher::new(metrics, 100, CancellationToken::new());
    watcher.record_keyboard_latency(Duration::from_millis(10));

    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(dev_rx.try_recv().is_err());
}
