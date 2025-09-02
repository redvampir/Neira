/* neira:meta
id: NEI-20250214-watchdog-module
intent: code
summary: |-
  Выделен модуль Watchdog: вычисление таймаутов по ENV,
  инкремент счётчиков и отправка webhook при hard‑срабатываниях.
*/
/* neira:meta
id: NEI-20250902-watchdog-anomaly
intent: feature
summary: |
  Добавлена проверка аномалий в метриках watchdog.
*/

use regex::Regex;
use serde_json::json;
use tracing::warn;

/// Минимальный набор метрик watchdog.
#[derive(Debug, Default)]
pub struct Metrics {
    pub soft: f64,
    pub hard: f64,
}

/// Простое правило: любое значение hard > 0 считается аномалией.
pub fn detect_anomaly(metrics: &Metrics) -> bool {
    metrics.hard > 0.0
}

/// Конфигурация и утилиты сторожевого таймера (watchdog).
pub struct Watchdog {
    /// Мягкий таймаут, мс.
    pub soft_ms: u64,
    /// Жёсткий таймаут, мс.
    pub hard_ms: u64,
    webhook: Option<String>,
}

impl Watchdog {
    fn env_ms(key: &str, default_ms: u64) -> u64 {
        std::env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_ms)
    }

    /// Настройки для конкретной клетки.
    /// `global_time_budget` используется как базовый hard‑таймаут.
    pub fn for_cell(cell_id: &str, global_time_budget: u64) -> Self {
        let base_soft = Self::env_ms("WATCHDOG_REASONING_SOFT_MS", 30_000);
        let base_hard = Self::env_ms("WATCHDOG_REASONING_HARD_MS", global_time_budget);

        // per-cell override: WATCHDOG_SOFT_MS_<ID>, WATCHDOG_HARD_MS_<ID>
        let mut up = cell_id
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect::<String>();
        if up.is_empty() {
            up = "DEFAULT".into();
        }
        let soft_key = format!("WATCHDOG_SOFT_MS_{}", up);
        let hard_key = format!("WATCHDOG_HARD_MS_{}", up);
        let soft_ms = Self::env_ms(&soft_key, base_soft);
        let hard_ms = Self::env_ms(&hard_key, base_hard);
        let webhook = std::env::var("INCIDENT_WEBHOOK_URL").ok();
        Self {
            soft_ms,
            hard_ms,
            webhook,
        }
    }

    /// Сообщить о soft‑таймауте.
    pub fn soft_timeout(&self) {
        metrics::counter!("watchdog_timeouts_total", "kind" => "soft").increment(1);
    }

    /// Сообщить о hard‑таймауте и отправить webhook, если он настроен.
    pub fn hard_timeout(&self, id: &str) {
        metrics::counter!("watchdog_timeouts_total", "kind" => "hard").increment(1);
        if let Some(url) = &self.webhook {
            let payload = json!({
                "type": "watchdog_hard",
                "id": id,
                "ts": chrono::Utc::now().to_rfc3339(),
            });
            let url = url.clone();
            tokio::spawn(async move {
                let _ = reqwest::Client::new().post(url).json(&payload).send().await;
            });
        }
    }

    /// Извлечь значения счётчиков `watchdog_timeouts_total` из текста /metrics.
    /// Возвращает `(soft, hard)`.
    pub fn parse_metrics(text: &str) -> (f64, f64) {
        let re = |kind: &str| -> f64 {
            let pat = format!(
                "(?m)^watchdog_timeouts_total\\{{[^}}]*kind=\\\"{}\\\"[^}}]*\\}}\\s+([0-9]+(?:\\.[0-9]+)?)$",
                kind
            );
            Regex::new(&pat)
                .ok()
                .and_then(|rg| {
                    rg.captures(text)
                        .and_then(|c| c.get(1))
                        .and_then(|m| m.as_str().parse::<f64>().ok())
                })
                .unwrap_or(0.0)
        };
        let metrics = Metrics {
            soft: re("soft"),
            hard: re("hard"),
        };
        if detect_anomaly(&metrics) {
            warn!(?metrics, "watchdog anomaly detected");
        }
        (metrics.soft, metrics.hard)
    }
}
