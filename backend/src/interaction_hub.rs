use std::sync::{Arc, RwLock};

use tokio::task::spawn_blocking;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::analysis_node::{AnalysisResult, NodeStatus};
use crate::memory_node::MemoryNode;
use crate::node_registry::NodeRegistry;
use crate::task_scheduler::TaskScheduler;
use crate::trigger_detector::TriggerDetector;

pub struct InteractionHub {
    pub registry: Arc<NodeRegistry>,
    pub memory: Arc<MemoryNode>,
    trigger_detector: Arc<TriggerDetector>,
    scheduler: RwLock<TaskScheduler>,
    allowed_tokens: RwLock<Vec<String>>,
}

impl InteractionHub {
    pub fn new(registry: Arc<NodeRegistry>, memory: Arc<MemoryNode>) -> Self {
        Self {
            registry,
            memory,
            trigger_detector: Arc::new(TriggerDetector::default()),
            scheduler: RwLock::new(TaskScheduler::default()),
            allowed_tokens: RwLock::new(Vec::new()),
        }
    }

    pub fn add_auth_token(&self, token: impl Into<String>) {
        self.allowed_tokens.write().unwrap().push(token.into());
    }

    fn authorize(&self, token: &str) => bool {
        self.allowed_tokens
            .read()
            .unwrap()
            .iter()
            .any(|t| t == token)
    }

    pub fn add_trigger_keyword(&self, keyword: impl Into<String>) {
        self.trigger_detector.add_keyword(keyword.into());
    }

    pub async fn analyze(
        &self,
        id: &str,
        input: &str,
        priority: u8,
        auth: &str,
        cancel_token: &CancellationToken,
    ) -> Option<AnalysisResult> {
        if !self.authorize(auth) {
            return None;
        }

        let triggers = self.trigger_detector.detect(input);
        let _ = self.memory.preload_by_trigger(&triggers);

        self.scheduler
            .write()
            .unwrap()
            .enqueue(id.to_string(), input.to_string(), priority);

        let (task_id, task_input) = self.scheduler.write().unwrap().next()?;
        let node = self.registry.get_analysis_node(&task_id)?;
        let cancel = cancel_token.clone();

        let handle = spawn_blocking(move || node.analyze(&task_input, &cancel));

        tokio::select! {
            _ = cancel_token.cancelled() => {
                let mut r = AnalysisResult::new(id, "", vec![]);
                r.status = NodeStatus::Error;
                self.memory.save_checkpoint(id, &r);
                info!("analysis {} cancelled", id);
                Some(r)
            }
            res = handle => {
                if let Ok(result) = res {
                    if result.status == NodeStatus::Error {
                        self.memory.save_checkpoint(id, &result);
                    } else {
                        self.memory.push_metrics(&result);
                    }
                    info!("analysis {} completed", id);
                    Some(result)
                } else {
                    None
                }
            }
        }
    }

    pub fn resume(&self, id: &str, auth: &str) -> Option<AnalysisResult> {
        if !self.authorize(auth) {
            return None;
        }
        self.memory.load_checkpoint(id)
    }
}
