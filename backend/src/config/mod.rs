use serde::Deserialize;

/// Configuration for optional components of the backend.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub nervous_system: NervousSystemConfig,
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

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        let enabled = std::env::var("NERVOUS_SYSTEM_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        Self {
            nervous_system: NervousSystemConfig { enabled },
        }
    }
}
