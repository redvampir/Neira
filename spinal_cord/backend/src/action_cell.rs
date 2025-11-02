/* neira:meta
id: NEI-20250829-175425-action-cell
intent: docs
summary: |
  Базовый интерфейс клеток действий и стандартная реализация предзагрузки.
*/
/* neira:meta
id: NEI-20270520-action-cell-engine
intent: feature
summary: |
  Добавлена поддержка ActionEngine для выполнения команд клетками.
*/

use std::sync::Arc;

use crate::action_engine::{ActionCommand, ActionEngine, ActionError};
use crate::memory_cell::MemoryCell;
use async_trait::async_trait;

#[async_trait]
pub trait ActionCell: Send + Sync {
    fn id(&self) -> &str;
    fn preload(&self, triggers: &[String], memory: &Arc<MemoryCell>);

    fn command(&self) -> Option<ActionCommand> {
        None
    }

    async fn execute(&self, engine: &ActionEngine) -> Result<Option<String>, ActionError> {
        if let Some(cmd) = self.command() {
            let res = engine.execute(cmd).await?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
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
