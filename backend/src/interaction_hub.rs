use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use tokio::task::spawn_blocking;
use tokio::time::{interval, sleep};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::analysis_node::{AnalysisResult, NodeStatus};
use crate::memory_node::MemoryNode;
use crate::node_registry::NodeRegistry;
use crate::task_scheduler::{Queue, TaskScheduler};
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

    fn authorize(&self, token: &str) -> bool {
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
        auth: &str,
        cancel_token: &CancellationToken,
    ) -> Option<AnalysisResult> {
        if !self.authorize(auth) {
            return None;
        }

        let triggers = self.trigger_detector.detect(input);
        for node in self.registry.action_nodes() {
            node.preload(&triggers, &self.memory);
        }

        let priority = self.memory.get_priority(id);
        let avg_time = self.memory.average_time_ms(id).unwrap_or(0);
        let queue = if avg_time < 100 {
            Queue::Fast
        } else if avg_time < 1000 {
            Queue::Standard
        } else {
            Queue::Long
        };
        self.scheduler.write().unwrap().enqueue(
            queue,
            id.to_string(),
            input.to_string(),
            priority,
            None,
            vec![id.to_string()],
        );

        let (task_id, task_input) = self.scheduler.write().unwrap().next()?;
        let node = self.registry.get_analysis_node(&task_id)?;
        let cancel = cancel_token.clone();

        let handle = spawn_blocking(move || node.analyze(&task_input, &cancel));

        let start = Instant::now();
        let checkpoint_mem = self.memory.clone();
        let checkpoint_id = id.to_string();
        let checkpoint_cancel = cancel_token.clone();
        let cfg = self.scheduler.read().unwrap().config.clone();
        let mut interval_timer = interval(Duration::from_millis(cfg.checkpoint_interval_ms));
        tokio::spawn(async move {
            loop {
                interval_timer.tick().await;
                if checkpoint_cancel.is_cancelled() {
                    break;
                }
                let r = AnalysisResult::new(&checkpoint_id, "", vec![]);
                checkpoint_mem.save_checkpoint(&checkpoint_id, &r);
            }
        });

        tokio::select! {
            _ = sleep(Duration::from_millis(cfg.global_time_budget)) => {
                cancel_token.cancel();
                let mut r = AnalysisResult::new(id, "", vec![]);
                r.status = NodeStatus::Error;
                self.memory.save_checkpoint(id, &r);
                info!("analysis {} timed out", id);
                Some(r)
            }
            _ = cancel_token.cancelled() => {
                let mut r = AnalysisResult::new(id, "", vec![]);
                r.status = NodeStatus::Error;
                self.memory.save_checkpoint(id, &r);
                info!("analysis {} cancelled", id);
                Some(r)
            }
            res = handle => {
                if let Ok(result) = res {
                    let elapsed = start.elapsed().as_millis();
                    if result.status == NodeStatus::Error {
                        self.memory.save_checkpoint(id, &result);
                    } else {
                        self.memory.push_metrics(&result);
                        self.memory.update_time(id, elapsed);
                        let mem = self.memory.clone();
                        let rid = id.to_string();
                        mem.recalc_priority_async(rid);
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
