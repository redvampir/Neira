/* neira:meta
id: NEI-20280430-120400-healing-config
intent: code
summary: |
  Конфигурация «Исцеляющего сна», загрузка из TOML и пороги серьёзности.
*/
use std::fs;
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use super::incident::IncidentSeverity;
use super::trainer::PromptTrainingConfig;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConsultantSettings {
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealingSleepConfig {
    #[serde(default = "defaults::enabled")]
    pub enabled: bool,
    #[serde(default = "defaults::allow_external")]
    pub allow_external_consultant: bool,
    #[serde(default)]
    pub preferred_consultant: Option<String>,
    #[serde(default = "defaults::max_incidents_per_cycle")]
    pub max_incidents_per_cycle: usize,
    #[serde(default = "defaults::journal_limit")]
    pub journal_limit: usize,
    #[serde(default = "defaults::reflection_window_minutes")]
    pub reflection_window_minutes: u64,
    #[serde(default = "defaults::min_severity")]
    pub min_severity: IncidentSeverity,
    #[serde(default)]
    pub prompt: PromptTrainingConfig,
    #[serde(default)]
    pub consultant: ConsultantSettings,
}

impl Default for HealingSleepConfig {
    fn default() -> Self {
        Self {
            enabled: defaults::enabled(),
            allow_external_consultant: defaults::allow_external(),
            preferred_consultant: Some("openai".into()),
            max_incidents_per_cycle: defaults::max_incidents_per_cycle(),
            journal_limit: defaults::journal_limit(),
            reflection_window_minutes: defaults::reflection_window_minutes(),
            min_severity: defaults::min_severity(),
            prompt: PromptTrainingConfig::default(),
            consultant: ConsultantSettings::default(),
        }
    }
}

impl HealingSleepConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, HealingSleepConfigError> {
        let path = path.as_ref();
        let contents = match fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(err) => {
                return Err(HealingSleepConfigError::Io {
                    path: path.display().to_string(),
                    source: err,
                });
            }
        };

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            #[serde(default)]
            healing_sleep: Option<HealingSleepConfig>,
        }

        let parsed: Wrapper =
            toml::from_str(&contents).map_err(|source| HealingSleepConfigError::Toml {
                path: path.display().to_string(),
                source,
            })?;

        Ok(parsed.healing_sleep.unwrap_or_default())
    }

    pub fn should_consult(&self, severity: IncidentSeverity) -> bool {
        self.enabled && severity >= self.min_severity && self.allow_external_consultant
    }
}

#[derive(Debug, Error)]
pub enum HealingSleepConfigError {
    #[error("не удалось прочитать {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("не удалось разобрать {path}: {source}")]
    Toml {
        path: String,
        #[source]
        source: toml::de::Error,
    },
}

mod defaults {
    use super::IncidentSeverity;

    pub fn enabled() -> bool {
        true
    }

    pub fn allow_external() -> bool {
        false
    }

    pub fn max_incidents_per_cycle() -> usize {
        5
    }

    pub fn journal_limit() -> usize {
        120
    }

    pub fn reflection_window_minutes() -> u64 {
        45
    }

    pub fn min_severity() -> IncidentSeverity {
        IncidentSeverity::Medium
    }
}
