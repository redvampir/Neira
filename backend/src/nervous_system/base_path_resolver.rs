/* neira:meta
id: NEI-20250829-175425-base-path-resolver
intent: docs
summary: |
  Определяет базовый путь проекта и сохраняет его в памяти.
*/

use std::path::PathBuf;
use std::sync::Arc;

use crate::action_node::ActionNode;
use crate::analysis_node::AnalysisResult;
use crate::memory_node::MemoryNode;

/// Resolve the base path of the project by walking up from the current
/// executable location until a `config/integrity.json` file is found.
pub fn resolve_base_path() -> Option<PathBuf> {
    let mut exe = std::env::current_exe().ok()?;
    exe.pop();
    loop {
        if exe.join("config/integrity.json").exists() {
            return Some(exe);
        }
        if !exe.pop() {
            break;
        }
    }
    None
}

/// Node that resolves the base path once and stores it in the `MemoryNode`
/// under the `base_path` key.
#[derive(Default)]
pub struct BasePathResolverNode;

impl BasePathResolverNode {
    pub fn new() -> Self {
        Self
    }
}

impl ActionNode for BasePathResolverNode {
    fn id(&self) -> &str {
        "system.base_path_resolver"
    }

    fn preload(&self, _triggers: &[String], memory: &Arc<MemoryNode>) {
        if memory.load_checkpoint("base_path").is_some() {
            return;
        }
        if let Some(base) = resolve_base_path() {
            let result = AnalysisResult::new("base_path", base.display().to_string(), vec![]);
            memory.save_checkpoint("base_path", &result);
        }
    }
}
