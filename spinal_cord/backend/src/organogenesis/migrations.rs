use semver::Version;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Migration {
    async fn up(&self, organ: &mut serde_json::Value) -> Result<(), String>;
    async fn down(&self, organ: &mut serde_json::Value) -> Result<(), String>;
}

pub struct MigrationManager {
    migrations: HashMap<String, Vec<Box<dyn Migration + Send + Sync>>>,
}

impl MigrationManager {
    pub fn new() -> Self {
        Self {
            migrations: HashMap::new()
        }
    }

    pub async fn migrate_to_version(
        &self,
        organ_id: &str,
        organ: &mut serde_json::Value,
        target: &Version
    ) -> Result<(), String> {
        let current = self.get_current_version(organ)?;
        let migrations = self.migrations.get(organ_id).ok_or("Миграции не найдены")?;

        if current < *target {
            // Миграция вверх
            for migration in migrations {
                migration.up(organ).await?;
            }
        } else {
            // Откат
            for migration in migrations.iter().rev() {
                migration.down(organ).await?;
            }
        }

        self.update_version(organ, target)?;
        Ok(())
    }

    fn get_current_version(&self, organ: &serde_json::Value) -> Result<Version, String> {
        let version = organ.get("version")
            .and_then(|v| v.as_str())
            .ok_or("Версия не найдена")?;
        
        Version::parse(version).map_err(|e| e.to_string())
    }

    fn update_version(&self, organ: &mut serde_json::Value, version: &Version) -> Result<(), String> {
        if let Some(v) = organ.as_object_mut() {
            v.insert("version".into(), version.to_string().into());
            Ok(())
        } else {
            Err("Неверная структура органа".into())
        }
    }
}
