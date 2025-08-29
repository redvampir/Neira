use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use sha2::{Digest, Sha256};
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::action_node::ActionNode;
use crate::memory_node::MemoryNode;

/// Узел, проверяющий контрольные суммы файлов на соответствие эталонным значениям.
pub struct IntegrityCheckerNode {
    config_path: PathBuf,
    interval_ms: u64,
}

impl IntegrityCheckerNode {
    /// Создаёт узел и запускает периодическую проверку.
    pub fn new() -> Arc<Self> {
        let config_path = std::env::var("INTEGRITY_CONFIG_PATH")
            .unwrap_or_else(|_| "config/integrity.json".into());
        let interval_ms = std::env::var("INTEGRITY_CHECK_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60_000);
        let node = Arc::new(Self {
            config_path: PathBuf::from(config_path),
            interval_ms,
        });
        let node_clone = node.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_millis(node_clone.interval_ms));
            loop {
                ticker.tick().await;
                let n = node_clone.clone();
                tokio::task::spawn_blocking(move || {
                    if let Err(e) = n.check_once() {
                        error!("{e}");
                    }
                });
            }
        });
        node
    }

    fn check_once(&self) -> Result<(), String> {
        let base = match std::env::var("INTEGRITY_ROOT") {
            Ok(p) => PathBuf::from(p),
            Err(_) => std::env::current_dir().map_err(|e| format!("current_dir: {e}"))?,
        };
        let cfg_path = if self.config_path.is_absolute() {
            self.config_path.clone()
        } else {
            base.join(&self.config_path)
        };
        let data = fs::read_to_string(&cfg_path)
            .map_err(|e| format!("read {}: {e}", cfg_path.display()))?;
        let map: HashMap<String, String> = serde_json::from_str(&data)
            .map_err(|e| format!("parse {}: {e}", cfg_path.display()))?;
        for (rel, expected) in map.iter() {
            let rel_path = PathBuf::from(rel);
            let path = if rel_path.is_absolute() {
                rel_path
            } else {
                base.join(rel_path)
            };
            let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let actual = format!("{:x}", hasher.finalize());
            if &actual == expected {
                info!(file=%path.display(), "integrity ok");
            } else {
                warn!(file=%path.display(), expected=%expected, actual=%actual, "integrity mismatch");
            }
        }
        Ok(())
    }
}

impl ActionNode for IntegrityCheckerNode {
    fn id(&self) -> &str {
        "security.integrity_checker"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryNode>) {}
}
