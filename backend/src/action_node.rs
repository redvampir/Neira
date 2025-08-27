use crate::memory_node::MemoryNode;

pub trait ActionNode: Send + Sync {
    fn id(&self) -> &str;
    fn preload(&self, triggers: &[String], memory: &MemoryNode);
}

pub struct PreloadAction;

impl ActionNode for PreloadAction {
    fn id(&self) -> &str {
        "preload.action"
    }

    fn preload(&self, triggers: &[String], memory: &MemoryNode) {
        let _ = memory.preload_by_trigger(triggers);
    }
}

impl Default for PreloadAction {
    fn default() -> Self {
        Self
    }
}
