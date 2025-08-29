use serde::Deserialize;
use std::collections::HashMap;

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
    /// Enables metrics collection and the `/metrics` endpoint when `true`.
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
        let enabled = std::env::var("NERVOUS_SYSTEM_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let host_metrics_enabled = std::env::var("PROBES_HOST_METRICS_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let io_watcher_enabled = std::env::var("PROBES_IO_WATCHER_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
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
