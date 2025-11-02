use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganTemplate {
    pub template_id: String,
    pub base_type: OrganType,
    pub default_capabilities: Vec<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub compatibility: CompatibilityInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganType {
    Analyzer,
    Processor,
    Adapter,
    Storage,
    Sensor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub condition: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    pub min_version: String,
    pub required_organs: Vec<String>,
    pub optional_organs: Vec<String>,
}

impl OrganTemplate {
    pub fn validate_instance(&self, instance: &serde_json::Value) -> Result<(), String> {
        // Проверяем базовую структуру
        if !self.validate_structure(instance) {
            return Err("Неверная структура органа".into());
        }

        // Проверяем совместимость
        if !self.check_compatibility(instance) {
            return Err("Проблемы совместимости".into());
        }

        Ok(())
    }

    fn validate_structure(&self, _instance: &serde_json::Value) -> bool {
        // TODO: Реализовать проверку структуры
        true
    }

    fn check_compatibility(&self, _instance: &serde_json::Value) -> bool {
        // TODO: Реализовать проверку совместимости
        true
    }
}
