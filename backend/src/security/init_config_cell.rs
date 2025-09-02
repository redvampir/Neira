/* neira:meta
id: NEI-20250829-175425-init-config
intent: docs
scope: backend/security
summary: |
  Инициализирует переменные конфигурации, устанавливая INTEGRITY_ROOT.
env:
  - INTEGRITY_ROOT
*/

use std::sync::Arc;

use crate::action_cell::ActionCell;
use crate::memory_cell::MemoryCell;

/// Node responsible for initializing configuration variables.
/// Ensures `INTEGRITY_ROOT` is set based on `MemoryCell` base path.
pub struct InitConfigCell;

impl InitConfigCell {
    pub fn new() -> Self {
        Self
    }

    fn ensure_integrity_root(&self, memory: &Arc<MemoryCell>) {
        if std::env::var("INTEGRITY_ROOT").is_err() {
            let base = memory.base_path();
            std::env::set_var("INTEGRITY_ROOT", base);
        }
    }
}

impl ActionCell for InitConfigCell {
    fn id(&self) -> &str {
        "system.init_config"
    }

    fn preload(&self, _triggers: &[String], memory: &Arc<MemoryCell>) {
        self.ensure_integrity_root(memory);
    }
}
