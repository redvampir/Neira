use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellSpecialization {
    pub cell_type: CellType,
    pub capabilities: Vec<String>,
    pub adaptation_level: f32,
    pub specialization_history: Vec<SpecializationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellType {
    Processor { throughput: f32 },
    Memory { capacity_mb: u32 },
    Analyzer { accuracy: f32 },
    Connector { bandwidth: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializationEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trigger: String,
    pub changes: HashMap<String, serde_json::Value>,
    pub effectiveness: f32,
}

impl CellSpecialization {
    pub async fn adapt_to_load(&mut self, load_metrics: &LoadMetrics) {
        // Адаптируем параметры под нагрузку
        match &mut self.cell_type {
            CellType::Processor { throughput } => {
                *throughput = (*throughput + load_metrics.required_throughput) / 2.0;
            }
            CellType::Memory { capacity_mb } => {
                *capacity_mb = (*capacity_mb + load_metrics.required_memory) / 2;
            }
            // ...другие типы...
        }

        self.adaptation_level += 0.1;
        self.record_adaptation("load_based", load_metrics.effectiveness);
    }

    fn record_adaptation(&mut self, trigger: &str, effectiveness: f32) {
        self.specialization_history.push(SpecializationEvent {
            timestamp: chrono::Utc::now(),
            trigger: trigger.to_string(),
            changes: HashMap::new(), // TODO: добавить детали изменений
            effectiveness,
        });
    }
}
