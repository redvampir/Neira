/* neira:meta
id: NEI-20250829-175425-safe-mode
intent: docs
summary: |
  Контролирует переход системы в безопасный режим.
*/

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::warn;

/* neira:meta
id: NEI-20250505-000000-safe-mode-metrics
intent: feature
summary: |
  Добавлены метрики успешных и повторных переходов в safe mode.
*/
/// Контроллер безопасного режима.
/// При активации отключает все необязательные узлы,
/// оставляя только базовый функционал.
pub struct SafeModeController {
    in_safe_mode: AtomicBool,
}

impl SafeModeController {
    /// Создаёт новый контроллер безопасного режима.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            in_safe_mode: AtomicBool::new(false),
        })
    }

    /// Переводит систему в безопасный режим.
    /// В реальной реализации здесь будет логика отключения
    /// необязательных узлов и остановки фоновых задач.
    pub fn enter_safe_mode(&self) {
        if self.in_safe_mode.swap(true, Ordering::SeqCst) {
            metrics::counter!(
                "immune_action_failures_total",
                "action" => "safe_mode"
            )
            .increment(1);
            return;
        }
        warn!("entering safe mode: disabling non-essential cells");
        metrics::counter!("immune_actions_total", "action" => "safe_mode").increment(1);
    }

    /// Возвращает `true`, если система уже находится в безопасном режиме.
    pub fn is_safe_mode(&self) -> bool {
        self.in_safe_mode.load(Ordering::SeqCst)
    }
}
