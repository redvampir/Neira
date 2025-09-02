/* neira:meta
id: NEI-20250829-175425-action-cell
intent: docs
summary: |
  Базовый интерфейс клеток действий и стандартная реализация предзагрузки.
*/

use std::sync::Arc;

use crate::memory_cell::MemoryCell;

pub trait ActionCell: Send + Sync {
    fn id(&self) -> &str;
    fn preload(&self, triggers: &[String], memory: &Arc<MemoryCell>);
}

pub struct PreloadAction;

impl ActionCell for PreloadAction {
    fn id(&self) -> &str {
        "preload.action"
    }

    fn preload(&self, triggers: &[String], memory: &Arc<MemoryCell>) {
        let matched = memory.preload_by_trigger(triggers);
        for rec in matched {
            let mem = Arc::clone(memory);
            mem.recalc_priority_async(rec.id.clone());
        }
    }
}

impl Default for PreloadAction {
    fn default() -> Self {
        Self
    }
}
