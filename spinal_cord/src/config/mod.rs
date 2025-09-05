/* neira:meta
id: NEI-20250220-env-flag
intent: refactor
summary: Добавлена функция env_flag для чтения булевых флагов из окружения.
*/
use serde::Deserialize;
use std::collections::HashMap;

/// Читает булево значение из переменной окружения.
/// Возвращает `default`, если переменная не установлена.
pub fn env_flag(key: &str, default: bool) -> bool {
    std::env::var(key)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(default)
}

/// Configuration for optional components of the backend.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub nervous_system: NervousSystemConfig,
    #[serde(default)]
    pub probes: HashMap<String, ProbeConfig>,
}

/// Settings for the nervous system subsystem.
#[derive(Debug, Clone, Deserialize)]
pub struct NervousSystemConfig {
    /// Enables metrics collection and the /metrics endpoint when true.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl Default for NervousSystemConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProbeConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        let enabled = env_flag("NERVOUS_SYSTEM_ENABLED", true);

        let host_metrics_enabled = env_flag("PROBES_HOST_METRICS_ENABLED", true);

        // Новое решение: сначала читаем PROBES_IO_WATCHER_ENABLED,
        // при ошибке — проверяем legacy-переменную IO_WATCHER_ENABLED,
        // по умолчанию выключено (false).
        let io_watcher_enabled = env_flag(
            "PROBES_IO_WATCHER_ENABLED",
            env_flag("IO_WATCHER_ENABLED", false),
        );

        let mut probes = HashMap::new();
        probes.insert(
            "host_metrics".to_string(),
            ProbeConfig {
                enabled: host_metrics_enabled,
            },
        );
        probes.insert(
            "io_watcher".to_string(),
            ProbeConfig {
                enabled: io_watcher_enabled,
            },
        );

        Self {
            nervous_system: NervousSystemConfig { enabled },
            probes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::env_flag;

    #[test]
    fn parses_true_values() {
        std::env::set_var("ENV_FLAG_TEST_TRUE", "1");
        assert!(env_flag("ENV_FLAG_TEST_TRUE", false));
        std::env::set_var("ENV_FLAG_TEST_TRUE", "true");
        assert!(env_flag("ENV_FLAG_TEST_TRUE", false));
        std::env::set_var("ENV_FLAG_TEST_TRUE", "TRUE");
        assert!(env_flag("ENV_FLAG_TEST_TRUE", false));
        std::env::remove_var("ENV_FLAG_TEST_TRUE");
    }

    #[test]
    fn parses_false_and_default() {
        std::env::set_var("ENV_FLAG_TEST_FALSE", "0");
        assert!(!env_flag("ENV_FLAG_TEST_FALSE", true));
        std::env::set_var("ENV_FLAG_TEST_FALSE", "false");
        assert!(!env_flag("ENV_FLAG_TEST_FALSE", true));
        std::env::remove_var("ENV_FLAG_TEST_FALSE");
        assert!(env_flag("ENV_FLAG_TEST_FALSE", true));
        assert!(!env_flag("ENV_FLAG_TEST_FALSE", false));
    }
}
