pub mod action;
pub mod action_cell;
pub mod analysis_cell;
pub mod cell_registry;
pub mod cell_template;
pub mod config;
pub mod context;
pub mod hearing;
pub mod idempotent_store;
pub mod synapse_hub;
pub mod memory_cell;
pub mod nervous_system;
pub mod queue_config;
pub mod security;
pub mod task_scheduler;
pub mod trigger_detector;
// duplicates removed

// Global hub reference (optional), used for lightweight signals like Anti-Idle activity marks
use crate::synapse_hub::SynapseHub;
use std::sync::{Arc, OnceLock, RwLock};

pub static GLOBAL_HUB: OnceLock<RwLock<Option<Arc<SynapseHub>>>> = OnceLock::new();

pub mod factory;
pub mod organ_builder;
pub mod policy;
