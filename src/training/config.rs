/* neira:meta
id: NEI-20250211-163500-training-config-api
intent: feature
summary: |
  Добавил в конфиг обучения валидацию, загрузку из файла и частичные обновления.
*/
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG_PATH: &str = "config/training.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub success_threshold: f64,
    pub failure_threshold: f64,
    pub min_attempts: u64,
    pub data_dir: PathBuf,
}

#[derive(Debug)]
pub enum TrainingConfigError {
    Io(std::io::Error),
    ParseToml(toml::de::Error),
    SerializeToml(toml::ser::Error),
    Validation(String),
}

impl Display for TrainingConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TrainingConfigError::Io(err) => write!(f, "Ошибка ввода-вывода: {}", err),
            TrainingConfigError::ParseToml(err) => write!(f, "Не удалось разобрать TOML: {}", err),
            TrainingConfigError::SerializeToml(err) => {
                write!(f, "Не удалось сериализовать TOML: {}", err)
            }
            TrainingConfigError::Validation(message) => write!(f, "Конфигурация недопустима: {}", message),
        }
    }
}

impl Error for TrainingConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TrainingConfigError::Io(err) => Some(err),
            TrainingConfigError::ParseToml(err) => Some(err),
            TrainingConfigError::SerializeToml(err) => Some(err),
            TrainingConfigError::Validation(_) => None,
        }
    }
}

impl From<std::io::Error> for TrainingConfigError {
    fn from(value: std::io::Error) -> Self {
        TrainingConfigError::Io(value)
    }
}

impl From<toml::de::Error> for TrainingConfigError {
    fn from(value: toml::de::Error) -> Self {
        TrainingConfigError::ParseToml(value)
    }
}

impl From<toml::ser::Error> for TrainingConfigError {
    fn from(value: toml::ser::Error) -> Self {
        TrainingConfigError::SerializeToml(value)
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            success_threshold: 0.8,
            failure_threshold: 0.6,
            min_attempts: 5,
            data_dir: "data".into(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct TrainingConfigUpdate {
    pub success_threshold: Option<f64>,
    pub failure_threshold: Option<f64>,
    pub min_attempts: Option<u64>,
    pub data_dir: Option<PathBuf>,
}

impl TrainingConfig {
    pub fn resolve_path() -> PathBuf {
        env::var("NEIRA_TRAINING_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(DEFAULT_CONFIG_PATH))
    }

    pub fn load_or_default(path: &Path) -> Result<Self, TrainingConfigError> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config: TrainingConfig = toml::from_str(&content)?;
            config.validate()?;
            Ok(config)
        } else {
            Ok(TrainingConfig::default())
        }
    }

    pub fn save_to_path(&self, path: &Path) -> Result<(), TrainingConfigError> {
        self.validate()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), TrainingConfigError> {
        if !(0.0..=1.0).contains(&self.success_threshold) {
            return Err(TrainingConfigError::Validation(
                "success_threshold должен быть в диапазоне [0, 1]".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.failure_threshold) {
            return Err(TrainingConfigError::Validation(
                "failure_threshold должен быть в диапазоне [0, 1]".to_string(),
            ));
        }
        if self.success_threshold < self.failure_threshold {
            return Err(TrainingConfigError::Validation(
                "success_threshold не может быть меньше failure_threshold".to_string(),
            ));
        }
        if self.min_attempts == 0 {
            return Err(TrainingConfigError::Validation(
                "min_attempts должен быть больше нуля".to_string(),
            ));
        }
        Ok(())
    }

    pub fn apply_update(&mut self, update: TrainingConfigUpdate) {
        if let Some(value) = update.success_threshold {
            self.success_threshold = value;
        }
        if let Some(value) = update.failure_threshold {
            self.failure_threshold = value;
        }
        if let Some(value) = update.min_attempts {
            self.min_attempts = value;
        }
        if let Some(value) = update.data_dir {
            self.data_dir = value;
        }
    }
}

pub fn config_path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn validate_default_config() {
        let config = TrainingConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn reject_invalid_thresholds() {
        let mut config = TrainingConfig::default();
        config.success_threshold = 1.2;
        assert!(matches!(
            config.validate(),
            Err(TrainingConfigError::Validation(_))
        ));
    }

    #[test]
    fn load_and_save_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("training.toml");
        let mut config = TrainingConfig::default();
        config.min_attempts = 10;
        config.save_to_path(&path).unwrap();

        let loaded = TrainingConfig::load_or_default(&path).unwrap();
        assert_eq!(loaded.min_attempts, 10);
    }

    #[test]
    fn apply_update_changes_fields() {
        let mut config = TrainingConfig::default();
        let mut update = TrainingConfigUpdate::default();
        update.success_threshold = Some(0.9);
        update.failure_threshold = Some(0.7);
        update.min_attempts = Some(12);
        update.data_dir = Some(PathBuf::from("alt"));

        config.apply_update(update);

        assert_eq!(config.success_threshold, 0.9);
        assert_eq!(config.failure_threshold, 0.7);
        assert_eq!(config.min_attempts, 12);
        assert_eq!(config.data_dir, PathBuf::from("alt"));
    }
}
