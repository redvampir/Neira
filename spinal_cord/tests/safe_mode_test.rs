use backend::security::quarantine_cell::QuarantineCell;
use backend::security::safe_mode_controller::SafeModeController;

#[tokio::test]
async fn enters_safe_mode_on_quarantine() {
    let safe_mode = SafeModeController::new();
    let (_cell, tx, _dev_rx) = QuarantineCell::new(safe_mode.clone());
    tx.send("critical_module".to_string()).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    assert!(safe_mode.is_safe_mode());
}
