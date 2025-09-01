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
summary: Задержки переходов между стадиями читаются из ORGANS_BUILDER_STAGE_DELAYS.
*/
/* neira:meta
id: NEI-20250620-organ-builder-stage-delays-env
intent: code
summary: переименована переменная на ORGANS_BUILDER_STAGE_DELAYS.
*/
/* neira:meta
id: NEI-20251115-organ-cancel-build
intent: code
summary: сохраняются JoinHandle задач и добавлен cancel_build для их остановки.
*/
/* neira:meta
id: NEI-20251220-organ-builder-cleanup
intent: code
summary: добавлен фоновый таймер очистки и удаление записей templates/statuses вместе с файлом.
*/
/* neira:meta
id: NEI-20250601-organ-builder-restore
intent: code
summary: выделено восстановление шаблонов из templates_dir с логированием количества.
*/

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};

use serde::Serialize;
use serde_json::Value;
use tokio::sync::broadcast;
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
    events_tx: broadcast::Sender<(String, OrganState)>,
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
        let stages_env = std::env::var("ORGANS_BUILDER_STAGE_DELAYS")
            .or_else(|_| {
                let deprecated = std::env::var("ORGANS_BUILDER_STAGE_DELAYS_MS");
                if deprecated.is_ok() {
                    tracing::warn!(
                        "ORGANS_BUILDER_STAGE_DELAYS_MS is deprecated; use ORGANS_BUILDER_STAGE_DELAYS"
                    );
                }
                deprecated
            })
            .unwrap_or_else(|_| "50,50,50".into());
        let stages = parse_stage_delays(&stages_env);
        if enabled {
            let _ = std::fs::create_dir_all(&templates_dir);
        }
        let (events_tx, _rx) = broadcast::channel(16);
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
            events_tx,
        });
        if enabled {
            let restored = this.restore_existing();
            metrics::counter!("organ_build_restored_total").increment(restored);
            info!(restored_count = restored, "organ builder restored organs");
        }
        if enabled && ttl_secs > 0 {
            let this_bg = Arc::clone(&this);
            tokio::spawn(async move {
                let interval = this_bg.ttl;
                loop {
                    tokio::time::sleep(interval).await;
                    this_bg.cleanup_expired().await;
                }
            });
        }
        this
    }

    /// Загружает сохранённые шаблоны и статусы с диска.
    fn restore_existing(&self) -> u64 {
        let mut restored = 0u64;
        let mut max_id = 0u64;
        if let Ok(entries) = std::fs::read_dir(&self.templates_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Some(num) = id.strip_prefix("organ-") {
                        if let Ok(n) = num.parse::<u64>() {
                            max_id = max_id.max(n);
                        }
                    }
                    if let Ok(data) = std::fs::read_to_string(&path) {
                        if let Ok(tpl) = serde_json::from_str::<Value>(&data) {
                            self.templates.write().unwrap().insert(id.to_string(), tpl);
                            self.statuses
                                .write()
                                .unwrap()
                                .insert(id.to_string(), OrganState::Stable);
                            restored += 1;
                        }
                    }
                }
            }
        }
        self.counter.store(max_id + 1, Ordering::Relaxed);
        restored
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
        let _ = self.events_tx.send((id.clone(), OrganState::Draft));
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

    /* neira:meta
    id: NEI-20251205-organ-rebuild
    intent: code
    summary: добавлен метод `rebuild` для повторного запуска сборки по сохранённому шаблону.
    */
    /// Перезапускает сборку органа с тем же идентификатором.
    pub fn rebuild(self: &Arc<Self>, id: &str) -> bool {
        let tpl = {
            let templates = self.templates.read().unwrap();
            match templates.get(id) {
                Some(t) => t.clone(),
                None => return false,
            }
        };
        if let Some(handle) = self.handles.write().unwrap().remove(id) {
            handle.abort();
        }
        self.statuses
            .write()
            .unwrap()
            .insert(id.to_string(), OrganState::Draft);
        self.start_times
            .write()
            .unwrap()
            .insert(id.to_string(), Instant::now());
        let path = self.templates_dir.join(format!("{id}.json"));
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(path, tpl.to_string());
        metrics::counter!("organ_rebuild_attempts_total").increment(1);
        info!(organ_id = %id, "organ rebuild started");
        let _ = self.events_tx.send((id.to_string(), OrganState::Draft));
        let this = Arc::clone(self);
        let build_id = id.to_string();
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
        self.handles.write().unwrap().insert(id.to_string(), handle);
        true
    }

    /* neira:meta
    id: NEI-20260407-organ-builder-list
    intent: code
    summary: добавлен метод list для выдачи идентификаторов и стадий всех органов.
    */
    /// Возвращает все известные органы и их статусы.
    pub fn list(&self) -> Vec<(String, OrganState)> {
        metrics::counter!("organ_build_list_queries_total").increment(1);
        self.statuses
            .read()
            .unwrap()
            .iter()
            .map(|(id, st)| (id.clone(), *st))
            .collect()
    }

    /// Возвращает статус сборки.
    pub fn status(&self, id: &str) -> Option<OrganState> {
        metrics::counter!("organ_build_status_queries_total").increment(1);
        self.statuses.read().unwrap().get(id).copied()
    }

    /* neira:meta
    id: NEI-20250317-organ-builder-status-error
    intent: code
    summary: increments error counter when updating status of missing organ.
    */
    /// Ручное обновление статуса.
    pub fn update_status(self: &Arc<Self>, id: &str, state: OrganState) -> Option<OrganState> {
        let mut statuses = self.statuses.write().unwrap();
        let prev = match statuses.get_mut(id) {
            Some(prev) => prev,
            None => {
                metrics::counter!("organ_build_status_errors_total").increment(1);
                return None;
            }
        };
        *prev = state;
        let _ = self.events_tx.send((id.to_string(), state));
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
                    this.statuses.write().unwrap().remove(&id);
                    let _ = tokio::fs::remove_file(path).await;
                });
            }
        }
        Some(*prev)
    }

    /* neira:meta
    id: NEI-20260501-organ-status-broadcast
    intent: code
    summary: добавлен канал оповещений о смене статуса органа.
    */
    /// Подписка на события изменения статусов.
    pub fn subscribe(&self) -> broadcast::Receiver<(String, OrganState)> {
        self.events_tx.subscribe()
    }

    /// Удаляет просроченные шаблоны и статусы.
    async fn cleanup_expired(self: &Arc<Self>) {
        let cutoff = SystemTime::now()
            .checked_sub(self.ttl)
            .unwrap_or(SystemTime::UNIX_EPOCH);
        if let Ok(mut entries) = tokio::fs::read_dir(&self.templates_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                if let Ok(meta) = entry.metadata().await {
                    if let Ok(modified) = meta.modified() {
                        if modified < cutoff {
                            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                let id = stem.to_string();
                                self.templates.write().unwrap().remove(&id);
                                self.statuses.write().unwrap().remove(&id);
                            }
                            let _ = tokio::fs::remove_file(&path).await;
                        }
                    }
                }
            }
        }
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
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn cancel_build_stops_task() {
        std::env::set_var("ORGANS_BUILDER_ENABLED", "1");
        std::env::set_var("ORGANS_BUILDER_STAGE_DELAYS", "1000,1000,1000");
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

    #[tokio::test]
    #[serial]
    async fn stage_delays_ms_env_is_supported() {
        std::env::remove_var("ORGANS_BUILDER_STAGE_DELAYS");
        std::env::set_var("ORGANS_BUILDER_STAGE_DELAYS_MS", "10,20,30");
        let builder = OrganBuilder::new();
        assert_eq!(
            builder.stages,
            vec![
                (OrganState::Canary, 10),
                (OrganState::Experimental, 20),
                (OrganState::Stable, 30),
            ]
        );
        std::env::remove_var("ORGANS_BUILDER_STAGE_DELAYS_MS");
    }
}
