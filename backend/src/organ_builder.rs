/* neira:meta
id: NEI-20251010-organ-builder
intent: code
summary: Минимальный орган-билдер: хранение шаблонов и статусов, логирование и метрики.
*/

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use serde::Serialize;
use serde_json::Value;
use tracing::info;

/// Состояние сборки органа.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrganState {
    Draft,
    Failed,
}

/// Хранит шаблоны органов и их статусы.
pub struct OrganBuilder {
    templates: RwLock<HashMap<String, Value>>,
    statuses: RwLock<HashMap<String, OrganState>>,
    counter: AtomicU64,
    enabled: bool,
}

impl OrganBuilder {
    /// Создаёт новый орган-билдер. Включение контролируется переменной окружения
    /// `ORGANS_BUILDER_ENABLED`.
    pub fn new() -> Arc<Self> {
        let enabled = std::env::var("ORGANS_BUILDER_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Arc::new(Self {
            templates: RwLock::new(HashMap::new()),
            statuses: RwLock::new(HashMap::new()),
            counter: AtomicU64::new(1),
            enabled,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Сохраняет шаблон и возвращает идентификатор органа.
    pub fn start_build(&self, tpl: Value) -> String {
        let id = format!("organ-{}", self.counter.fetch_add(1, Ordering::Relaxed));
        {
            self.templates.write().unwrap().insert(id.clone(), tpl);
            self.statuses
                .write()
                .unwrap()
                .insert(id.clone(), OrganState::Draft);
        }
        metrics::counter!("organ_build_attempts_total").increment(1);
        info!(organ_id = %id, "organ build started");
        id
    }

    /// Возвращает статус сборки.
    pub fn status(&self, id: &str) -> Option<OrganState> {
        metrics::counter!("organ_build_status_queries_total").increment(1);
        self.statuses.read().unwrap().get(id).copied()
    }
}
