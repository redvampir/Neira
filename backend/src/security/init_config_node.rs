use std::sync::Arc;

use crate::action_node::ActionNode;
use crate::memory_node::MemoryNode;

/// Node responsible for initializing configuration variables.
/// Ensures `INTEGRITY_ROOT` is set based on `MemoryNode` base path.
pub struct InitConfigNode;

impl InitConfigNode {
    pub fn new() -> Self {
        Self
    }

    fn ensure_integrity_root(&self, memory: &Arc<MemoryNode>) {
        if std::env::var("INTEGRITY_ROOT").is_err() {
            let base = memory.base_path();
            std::env::set_var("INTEGRITY_ROOT", base);
        }
    }
}

impl ActionNode for InitConfigNode {
    fn id(&self) -> &str {
        "system.init_config"
    }

    fn preload(&self, _triggers: &[String], memory: &Arc<MemoryNode>) {
        self.ensure_integrity_root(memory);
    }
}
