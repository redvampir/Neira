use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionRequirement {
    pub required_organs: Vec<String>,
    pub optional_organs: Vec<String>,
    pub min_connections: u32,
    pub max_connections: u32,
}

pub struct ConnectivityValidator {
    requirements: RwLock<HashMap<String, ConnectionRequirement>>,
    active_connections: RwLock<HashMap<String, Vec<String>>>,
}

impl ConnectivityValidator {
    pub async fn validate_organ_creation(&self, blueprint: &OrganBlueprint) -> Result<(), String> {
        // Проверяем обязательные связи
        let reqs = self.get_requirements(blueprint).await?;
        
        for required in &reqs.required_organs {
            if !self.organ_exists(required).await {
                return Err(format!(
                    "Невозможно создать орган {}: отсутствует необходимый орган {}",
                    blueprint.name, required
                ));
            }
        }

        // Проверяем общую связность
        self.validate_connectivity(blueprint).await?;
        
        Ok(())
    }

    pub async fn suggest_missing_organs(&self, blueprint: &OrganBlueprint) -> Vec<OrganBlueprint> {
        let mut suggestions = Vec::new();
        let reqs = self.get_requirements(blueprint).await.unwrap_or_default();

        for required in reqs.required_organs {
            if !self.organ_exists(&required).await {
                if let Some(template) = self.get_organ_template(&required).await {
                    suggestions.push(template);
                }
            }
        }

        suggestions
    }
}
