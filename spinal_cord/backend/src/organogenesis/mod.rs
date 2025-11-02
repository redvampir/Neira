use crate::connectivity::ConnectivityValidator;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganBlueprint {
    pub name: String,
    pub purpose: String,
    pub dependencies: Vec<String>,
    pub estimated_complexity: u32,
    pub required_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganGrowthReport {
    pub blueprint: OrganBlueprint,
    pub status: GrowthStatus,
    pub progress: f32,
    pub eta_seconds: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GrowthStatus {
    Proposed,
    Approved,
    Growing,
    Complete,
    Failed,
}

pub struct Organogenesis {
    blueprints: RwLock<Vec<OrganBlueprint>>,
    growth_queue: RwLock<Vec<OrganGrowthReport>>,
    organs_path: PathBuf,
}

impl Organogenesis {
    pub fn new<P: Into<PathBuf>>(organs_path: P) -> Self {
        Self {
            blueprints: RwLock::new(Vec::new()),
            growth_queue: RwLock::new(Vec::new()),
            organs_path: organs_path.into(),
        }
    }

    pub async fn propose_organ(&self, need: &str) -> Option<OrganBlueprint> {
        let blueprint = self.design_organ(need).await?;
        info!("Предлагаю создать новый орган: {}", blueprint.name);
        
        Some(blueprint)
    }

    pub async fn begin_growth(&self, blueprint: OrganBlueprint) -> Result<(), String> {
        // Проверяем связи
        self.validator.validate_organ_creation(&blueprint).await?;

        // Проверяем необходимость создания дополнительных органов
        let missing = self.validator.suggest_missing_organs(&blueprint).await;
        if !missing.is_empty() {
            info!("Требуется создание дополнительных органов: {:?}", missing);
            
            for organ in missing {
                self.schedule_organ_creation(organ).await?;
            }
        }

        let report = OrganGrowthReport {
            blueprint: blueprint.clone(),
            status: GrowthStatus::Proposed,
            progress: 0.0,
            eta_seconds: self.estimate_growth_time(&blueprint),
        };

        info!("Запрашиваю разрешение на создание органа: {}", blueprint.name);
        self.notify_user(&report).await;

        Ok(())
    }

    async fn schedule_organ_creation(&self, blueprint: OrganBlueprint) -> Result<(), String> {
        info!("Планирую создание органа: {}", blueprint.name);
        let report = OrganGrowthReport {
            blueprint: blueprint.clone(),
            status: GrowthStatus::Proposed,
            progress: 0.0,
            eta_seconds: self.estimate_growth_time(&blueprint),
        };

        self.notify_user(&report).await;
        Ok(())
    }

    async fn notify_user(&self, report: &OrganGrowthReport) {
        // TODO: Реализовать уведомление через систему сообщений
        info!("🔄 Требуется новый орган: {}", report.blueprint.name);
        info!("📋 Назначение: {}", report.blueprint.purpose);
        info!("⏱ Ожидаемое время роста: {}с", report.eta_seconds);
    }

    async fn design_organ(&self, need: &str) -> Option<OrganBlueprint> {
        // Временная реализация
        Some(OrganBlueprint {
            name: format!("organ_{}", need),
            purpose: format!("Обработка {}", need),
            dependencies: Vec::new(),
            estimated_complexity: 1,
            required_capabilities: vec!["basic_growth".to_string()],
        })
    }

    fn estimate_growth_time(&self, blueprint: &OrganBlueprint) -> u64 {
        // Базовая формула оценки времени роста
        60 * blueprint.estimated_complexity
    }
}
