/* neira:meta
id: NEI-20261005-digestive-time-metrics
intent: feature
summary: Метрики времени разбора и проверки схемы DigestivePipeline.
*/
use metrics::histogram;

/// Записывает время разбора входа в миллисекундах.
pub fn record_parse_duration_ms(ms: f64) {
    histogram!("digestive_parse_duration_ms").record(ms);
    histogram!("digestive_parse_duration_ms_p95").record(ms);
    histogram!("digestive_parse_duration_ms_p99").record(ms);
}

/// Записывает время проверки схемы в миллисекундах.
pub fn record_validation_duration_ms(ms: f64) {
    histogram!("digestive_validation_duration_ms").record(ms);
    histogram!("digestive_validation_duration_ms_p95").record(ms);
    histogram!("digestive_validation_duration_ms_p99").record(ms);
}
