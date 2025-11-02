use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct TrainingConfig {
    pub success_threshold: f64,
    pub failure_threshold: f64,
    pub min_attempts: u64,
    pub data_dir: PathBuf,
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

impl TrainingConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
