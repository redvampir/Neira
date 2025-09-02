pub mod action;
pub mod action_cell;
pub mod analysis_cell;
pub mod config;
pub mod context;
pub mod hearing;
pub mod idempotent_store;
pub mod interaction_hub;
pub mod memory_cell;
pub mod nervous_system;
pub mod cell_registry;
pub mod node_template;
pub mod queue_config;
pub mod security;
pub mod task_scheduler;
pub mod trigger_detector;
// duplicates removed

// Global hub reference (optional), used for lightweight signals like Anti-Idle activity marks
use crate::interaction_hub::InteractionHub;
use std::sync::{Arc, OnceLock, RwLock};

pub static GLOBAL_HUB: OnceLock<RwLock<Option<Arc<InteractionHub>>>> = OnceLock::new();

pub mod factory;
pub mod organ_builder;
pub mod policy;
