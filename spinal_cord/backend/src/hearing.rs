/* neira:meta
id: NEI-20240519-hearing-wrapper
intent: code
summary: |
  Обёртка вокруг tracing, отправляющая сообщения
  в Систему раздражителей.
*/

use tracing::{info as tracing_info, warn as tracing_warn};

const STIMULI_COUNTER: &str = "stimuli_events_total";

/// Запись информационного сообщения и отправка его в Систему раздражителей.
pub fn info(message: &str) {
    tracing_info!("{}", message);
    send("info", message);
}

/// Запись предупреждения и отправка его в Систему раздражителей.
pub fn warn(message: &str) {
    tracing_warn!("{}", message);
    send("warn", message);
}

fn send(level: &'static str, _message: &str) {
    metrics::counter!(STIMULI_COUNTER, "level" => level).increment(1);
}
