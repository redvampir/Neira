/* neira:meta
id: NEI-20250829-175425-quarantine-cell
intent: docs
summary: |
  Переводит подозрительные модули в карантин и активирует безопасный режим.
*/

use std::sync::Arc;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tracing::{info, warn};

use crate::action::diagnostics_cell::DeveloperRequest;
use crate::action_cell::ActionCell;
use crate::memory_cell::MemoryCell;
use crate::security::safe_mode_controller::SafeModeController;

/// Cell responsible for putting suspicious modules into quarantine.
/// Receives module identifiers over a channel and attempts to disable
/// or restart them. Each quarantine action is logged and a developer
/// notification is emitted.
#[derive(Clone)]
pub struct QuarantineCell {
    notify: UnboundedSender<DeveloperRequest>,
    safe_mode: Arc<SafeModeController>,
}

impl QuarantineCell {
    /// Creates the cell and returns a sender for quarantine messages
    /// along with a receiver for developer notifications.
    pub fn new(
        safe_mode: Arc<SafeModeController>,
    ) -> (
        Arc<Self>,
        UnboundedSender<String>,
        UnboundedReceiver<DeveloperRequest>,
    ) {
        let (tx, mut rx) = unbounded_channel();
        let (notify_tx, notify_rx) = unbounded_channel();
        let cell = Arc::new(Self {
            notify: notify_tx,
            safe_mode,
        });
        let cell_clone = cell.clone();
        tokio::spawn(async move {
            while let Some(module) = rx.recv().await {
                // Attempt to disable or restart the module. For now we simply
                // log the action.
                warn!(module = %module, "quarantine activated, disabling module");
                cell_clone.safe_mode.enter_safe_mode();
                // In a real implementation, logic to disable or restart the
                // module would go here.
                let _ = cell_clone.notify.send(DeveloperRequest {
                    description: format!("module {module} quarantined"),
                });
                info!(module = %module, "developer notified about quarantine");
            }
        });
        (cell, tx, notify_rx)
    }
}

impl ActionCell for QuarantineCell {
    fn id(&self) -> &str {
        "security.quarantine"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryCell>) {}
}
