/* neira:meta
id: NEI-20270323-heartbeat-module
intent: feat
summary: |-
  Обёртка для обновления метрики активных SSE-подключений.
*/

use metrics;

/// Увеличивает `sse_active` при открытии SSE-подключения.
pub fn increment_active() {
    metrics::gauge!("sse_active").increment(1.0);
}

/// Уменьшает `sse_active` при закрытии SSE-подключения.
pub fn decrement_active() {
    metrics::gauge!("sse_active").decrement(1.0);
}
