/* neira:meta
id: NEI-20251010-organ-builder
intent: code
summary: |-
  Асинхронная сборка органов со стадиями Draft→Canary→Experimental→Stable,
  сохранением шаблонов на диск и метрикой времени сборки.
*/

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::Value;
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
    counter: AtomicU64,
    templates_dir: PathBuf,
    enabled: bool,
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
        if enabled {
            let _ = std::fs::create_dir_all(&templates_dir);
        }
        Arc::new(Self {
            templates: RwLock::new(HashMap::new()),
            statuses: RwLock::new(HashMap::new()),
            start_times: RwLock::new(HashMap::new()),
            counter: AtomicU64::new(1),
            templates_dir,
            enabled,
        })
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
        tokio::spawn(async move {
            let stages = [
                (OrganState::Canary, 50u64),
                (OrganState::Experimental, 50u64),
                (OrganState::Stable, 50u64),
            ];
            for (state, delay) in stages {
                tokio::time::sleep(Duration::from_millis(delay)).await;
                this.update_status(&build_id, state);
            }
        });
        id
    }

    /// Возвращает статус сборки.
    pub fn status(&self, id: &str) -> Option<OrganState> {
        metrics::counter!("organ_build_status_queries_total").increment(1);
        self.statuses.read().unwrap().get(id).copied()
    }

    /// Ручное обновление статуса.
    pub fn update_status(&self, id: &str, state: OrganState) -> Option<OrganState> {
        let mut statuses = self.statuses.write().unwrap();
        let prev = statuses.get_mut(id)?;
        *prev = state;
        if state == OrganState::Stable {
            if let Some(start) = self.start_times.write().unwrap().remove(id) {
                let ms = start.elapsed().as_millis() as f64;
                metrics::histogram!("organ_build_duration_ms").record(ms);
            }
        }
        Some(*prev)
    }
}
