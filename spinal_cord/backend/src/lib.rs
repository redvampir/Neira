/* neira:meta
id: NEI-20270318-120120-training-export
intent: feature
summary: Экспортирован модуль training для автоматизированного обучения.
*/
/* neira:meta
id: NEI-20250215-immune-export
intent: code
summary: Экспортирован модуль immune_system.
*/
#![cfg_attr(test, allow(clippy::type_complexity))]
pub mod action;
pub mod action_cell;
/* neira:meta
id: NEI-20270520-lib-action-engine-export
intent: code
summary: Экспортирован модуль action_engine.
*/
pub mod action_engine;
pub mod analysis_cell;
pub mod cell_registry;
pub mod cell_template;
pub mod config;
pub mod context;
pub mod healing_sleep;
pub mod hearing;
pub mod idempotent_store;
/* neira:meta
id: NEI-20251227-event-bus-export
intent: code
summary: Экспортирован модуль event_bus.
*/
pub mod event_bus;
/* neira:meta
id: NEI-20260614-brain-export
intent: code
summary: Экспортирован модуль brain.
*/
pub mod brain;
pub mod immune_system;
pub mod memory_cell;
pub mod persona;
/* neira:meta
id: NEI-20260530-digestive-export
intent: code
summary: Экспортирован модуль digestive_pipeline.
*/
pub mod digestive_pipeline;
/* neira:meta
id: NEI-20261005-time-metrics-export
intent: code
summary: Экспортирован модуль time_metrics.
*/
pub mod nervous_system;
pub mod time_metrics;
/* neira:meta
id: NEI-20250226-circulatory-export
intent: code
summary: Экспортирован модуль circulatory_system.
*/
pub mod circulatory_system;
pub mod queue_config;
pub mod security;
pub mod synapse_hub;
pub mod task_scheduler;
pub mod training;
pub mod trigger_detector;
// duplicates removed

// Global hub reference (optional), used for lightweight signals like Anti-Idle activity marks

use std::sync::{Arc, OnceLock, RwLock};

pub static GLOBAL_HUB: OnceLock<RwLock<Option<Arc<synapse_hub::SynapseHub>>>> = OnceLock::new();

pub mod factory;
pub mod organ_builder;
pub mod policy;
/* neira:meta
id: NEI-20270310-120200-event-log-export
intent: code
summary: Экспортирован модуль event_log.
*/
pub mod event_log;
pub mod voice;

/* neira:meta
id: NEI-20240513-lib-test-allow
intent: chore
summary: Разрешён clippy::type_complexity для тестов через cfg_attr.
*/

pub use training::{
    curriculum::RussianLiteracyCurriculum, metrics::LearningMetrics, scheduler::AdaptiveScheduler,
};

pub use action::diagnostics_cell::DiagnosticsCell;
pub use action::metrics_collector_cell::{MetricsCollectorCell, MetricsRecord};
pub use analysis_cell::{AnalysisCell, AnalysisResult, CellStatus, QualityMetrics};
pub use cell_template::{CellTemplate, Metadata};
pub use digestive_pipeline::{DigestivePipeline, ParsedInput, PipelineError};
pub use healing_sleep::{
    ConsultationPrompt, ConsultationRecord, HealingIncident, HealingOutcome, HealingSleep,
    HealingSleepConfig, HealingSleepConfigError, IncidentScope, IncidentSeverity,
    PromptTrainingConfig,
};
pub use memory_cell::MemoryCell;
pub use synapse_hub::SynapseHub;
pub mod learning;

// Модули
mod neural_network;
mod self_learning;

pub use neural_network::{NetworkMetrics, NeuralCore};
pub use self_learning::{LearningConfig, SelfLearning};

// Реэкспорт публичного API
pub use cell_registry::CellRegistry;
