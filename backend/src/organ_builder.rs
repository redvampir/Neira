/* neira:meta
id: NEI-20251010-organ-builder
intent: code
summary: |-
  Асинхронная сборка органов со стадиями Draft→Canary→Experimental→Stable,
  сохранением шаблонов на диск, удалением по TTL после стабилизации,
  метрикой времени сборки, остановкой при ручном изменении статуса и
  восстановлением счётчика идентификаторов при рестарте.
*/
/* neira:meta
id: NEI-20251101-organ-builder-stage-delays
intent: code
summary: Задержки переходов между стадиями читаются из ORGANS_BUILDER_STAGE_DELAYS_MS.
*/
/* neira:meta
id: NEI-20251115-organ-cancel-build
intent: code
summary: сохраняются JoinHandle задач и добавлен cancel_build для их остановки.
*/

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::Value;
use tokio::task::JoinHandle;
use tracing::info;

/// Состояние сборки органа.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrganState {
    Draft,
    Canary,
    Experimental,
    Stable,
    Failed,
}

/// Хранит шаблоны органов и их статусы.
pub struct OrganBuilder {
    templates: RwLock<HashMap<String, Value>>,
    statuses: RwLock<HashMap<String, OrganState>>,
    start_times: RwLock<HashMap<String, Instant>>,
    handles: RwLock<HashMap<String, JoinHandle<()>>>,
    counter: AtomicU64,
    templates_dir: PathBuf,
    enabled: bool,
    ttl: Duration,
    stages: Vec<(OrganState, u64)>,
}

impl OrganBuilder {
    /// Создаёт новый орган-билдер. Включение контролируется переменной окружения
    /// `ORGANS_BUILDER_ENABLED`.
    pub fn new() -> Arc<Self> {
        let enabled = std::env::var("ORGANS_BUILDER_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let dir = std::env::var("ORGANS_BUILDER_TEMPLATES_DIR")
            .unwrap_or_else(|_| "organ_templates".into());
        let templates_dir = PathBuf::from(dir);
        let ttl_secs = std::env::var("ORGANS_BUILDER_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);
        let stages_env =
            std::env::var("ORGANS_BUILDER_STAGE_DELAYS_MS").unwrap_or_else(|_| "50,50,50".into());
        let stages = parse_stage_delays(&stages_env);
        if enabled {
            let _ = std::fs::create_dir_all(&templates_dir);
        }
        let this = Arc::new(Self {
            templates: RwLock::new(HashMap::new()),
            statuses: RwLock::new(HashMap::new()),
            start_times: RwLock::new(HashMap::new()),
            handles: RwLock::new(HashMap::new()),
            counter: AtomicU64::new(1),
            templates_dir,
            enabled,
            ttl: Duration::from_secs(ttl_secs),
            stages,
        });
        if enabled {
            let mut restored = 0u64;
            let mut max_id = 0u64;
            if let Ok(entries) = std::fs::read_dir(&this.templates_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                        if let Some(num) = id.strip_prefix("organ-") {
                            if let Ok(n) = num.parse::<u64>() {
                                if n > max_id {
                                    max_id = n;
                                }
                            }
                        }
                        if let Ok(data) = std::fs::read_to_string(&path) {
                            if let Ok(tpl) = serde_json::from_str::<Value>(&data) {
                                this.templates.write().unwrap().insert(id.to_string(), tpl);
                                this.statuses
                                    .write()
                                    .unwrap()
                                    .insert(id.to_string(), OrganState::Stable);
                                restored += 1;
                            }
                        }
                    }
                }
            }
            this.counter.store(max_id + 1, Ordering::Relaxed);
            metrics::counter!("organ_build_restored_total").increment(restored);
            info!(restored, "organ builder restored organs");
        }
        this
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Сохраняет шаблон и возвращает идентификатор органа.
    pub fn start_build(self: &Arc<Self>, tpl: Value) -> String {
        let id = format!("organ-{}", self.counter.fetch_add(1, Ordering::Relaxed));
        {
            self.templates
                .write()
                .unwrap()
                .insert(id.clone(), tpl.clone());
            self.statuses
                .write()
                .unwrap()
                .insert(id.clone(), OrganState::Draft);
            self.start_times
                .write()
                .unwrap()
                .insert(id.clone(), Instant::now());
            let path = self.templates_dir.join(format!("{id}.json"));
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(path, tpl.to_string());
        }
        metrics::counter!("organ_build_attempts_total").increment(1);
        info!(organ_id = %id, "organ build started");
        let this = Arc::clone(self);
        let build_id = id.clone();
        let stages = self.stages.clone();
        let handle = tokio::spawn(async move {
            let mut expected = OrganState::Draft;
            for (state, delay) in stages {
                tokio::time::sleep(Duration::from_millis(delay)).await;
                if this.status(&build_id) != Some(expected) {
                    break;
                }
                this.update_status(&build_id, state);
                expected = state;
            }
            this.handles.write().unwrap().remove(&build_id);
        });
        self.handles.write().unwrap().insert(id.clone(), handle);
        id
    }

    /// Возвращает статус сборки.
    pub fn status(&self, id: &str) -> Option<OrganState> {
        metrics::counter!("organ_build_status_queries_total").increment(1);
        self.statuses.read().unwrap().get(id).copied()
    }

    /// Ручное обновление статуса.
    pub fn update_status(self: &Arc<Self>, id: &str, state: OrganState) -> Option<OrganState> {
        let mut statuses = self.statuses.write().unwrap();
        let prev = statuses.get_mut(id)?;
        *prev = state;
        if state == OrganState::Stable {
            if let Some(start) = self.start_times.write().unwrap().remove(id) {
                let ms = start.elapsed().as_millis() as f64;
                metrics::histogram!("organ_build_duration_ms").record(ms);
            }
            if self.ttl.as_secs() > 0 {
                let this = Arc::clone(self);
                let id = id.to_string();
                let path = this.templates_dir.join(format!("{id}.json"));
                tokio::spawn(async move {
                    tokio::time::sleep(this.ttl).await;
                    this.templates.write().unwrap().remove(&id);
                    let _ = tokio::fs::remove_file(path).await;
                });
            }
        }
        Some(*prev)
    }

    /// Отменяет сборку органа по идентификатору.
    pub fn cancel_build(self: &Arc<Self>, id: &str) -> bool {
        if let Some(handle) = self.handles.write().unwrap().remove(id) {
            handle.abort();
            self.start_times.write().unwrap().remove(id);
            self.update_status(id, OrganState::Failed);
            info!(organ_id = %id, "organ build cancelled");
            true
        } else {
            false
        }
    }
}

fn parse_stage_delays(input: &str) -> Vec<(OrganState, u64)> {
    let mut delays = [50u64, 50, 50];
    for (i, part) in input.split(',').enumerate() {
        if i >= delays.len() {
            break;
        }
        if let Ok(ms) = part.trim().parse::<u64>() {
            delays[i] = ms;
        }
    }
    [
        OrganState::Canary,
        OrganState::Experimental,
        OrganState::Stable,
    ]
    .into_iter()
    .zip(delays)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cancel_build_stops_task() {
        std::env::set_var("ORGANS_BUILDER_ENABLED", "1");
        std::env::set_var("ORGANS_BUILDER_STAGE_DELAYS_MS", "1000,1000,1000");
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.path());
        let builder = OrganBuilder::new();
        let id = builder.start_build(serde_json::json!({}));
        assert_eq!(builder.status(&id), Some(OrganState::Draft));
        assert!(builder.cancel_build(&id));
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(builder.status(&id), Some(OrganState::Failed));
        assert!(!builder.handles.read().unwrap().contains_key(&id));
    }
}
