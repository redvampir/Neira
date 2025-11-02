/* neira:meta
id: NEI-20250829-175425-init-config
intent: docs
scope: spinal_cord/security
summary: |
  Инициализирует переменные конфигурации, устанавливая INTEGRITY_ROOT.
env:
  - INTEGRITY_ROOT
*/

use std::sync::Arc;

use crate::action_cell::ActionCell;
use crate::memory_cell::MemoryCell;

/* neira:meta
id: NEI-20250505-000000-init-config-metrics
intent: feature
summary: |
  Добавлена метрика инициализации конфигурации.
*/

/// Cell responsible for initializing configuration variables.
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
        metrics::counter!("immune_actions_total", "action" => "init_config").increment(1);
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

impl Default for InitConfigCell {
    fn default() -> Self {
        Self::new()
    }
}

/* neira:meta
id: NEI-20240513-initconfig-default
intent: chore
summary: Добавлен Default для InitConfigCell во избежание lint new_without_default.
*/
