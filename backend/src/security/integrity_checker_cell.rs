/* neira:meta
id: NEI-20250829-175425-integrity-checker
intent: docs
scope: backend/security
summary: |
  Проверяет контрольные суммы файлов и отправляет подозрительные в карантин.
env:
  - INTEGRITY_CONFIG_PATH
  - INTEGRITY_CHECK_INTERVAL_MS
*/

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use sha2::{Digest, Sha256};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::action_cell::ActionCell;
use crate::memory_cell::MemoryCell;

/* neira:meta
id: NEI-20250505-000000-integrity-metrics
intent: feature
summary: |
  Добавлены метрики успехов и ошибок проверки целостности.
*/

/// Узел, проверяющий контрольные суммы файлов на соответствие эталонным значениям.
pub struct IntegrityCheckerCell {
    config_path: PathBuf,
    interval_ms: u64,
    memory: Arc<MemoryCell>,
    quarantine: UnboundedSender<String>,
}

impl IntegrityCheckerCell {
    /// Создаёт узел и запускает периодическую проверку.
    pub fn new(memory: Arc<MemoryCell>, quarantine: UnboundedSender<String>) -> Arc<Self> {
        let config_path = std::env::var("INTEGRITY_CONFIG_PATH")
            .unwrap_or_else(|_| "config/integrity.json".into());
        let interval_ms = std::env::var("INTEGRITY_CHECK_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60_000);
        let cell = Arc::new(Self {
            config_path: PathBuf::from(config_path),
            interval_ms,
            memory,
            quarantine,
        });
        let cell_clone = cell.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_millis(cell_clone.interval_ms));
            loop {
                ticker.tick().await;
                let n = cell_clone.clone();
                tokio::task::spawn_blocking(move || {
                    if let Err(e) = n.check_once() {
                        error!("{e}");
                    }
                });
            }
        });
        cell
    }

    fn check_once(&self) -> Result<(), String> {
        let result: Result<(), String> = (|| {
            let base = if let Some(res) = self.memory.load_checkpoint("base_path") {
                PathBuf::from(res.output)
            } else {
                std::env::current_dir().map_err(|e| format!("current_dir: {e}"))?
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
                    let _ = self.quarantine.send(path.display().to_string());
                }
            }
            Ok(())
        })();

        match result {
            Ok(()) => {
                metrics::counter!(
                    "immune_actions_total",
                    "action" => "integrity_check"
                )
                .increment(1);
                Ok(())
            }
            Err(e) => {
                metrics::counter!(
                    "immune_action_failures_total",
                    "action" => "integrity_check"
                )
                .increment(1);
                Err(e)
            }
        }
    }
}

impl ActionCell for IntegrityCheckerCell {
    fn id(&self) -> &str {
        "security.integrity_checker"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryCell>) {}
}
