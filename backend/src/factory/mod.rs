/* neira:meta
id: NEI-20250923-factory-core
intent: code
summary: |
  Минимальный каркас фабрики узлов: сервис записей, простейший Fabricator (Adapter‑only) и Selector (reuse vs create) без исполнения кода.
*/
/* neira:meta
id: NEI-20250316-stemcell-rename
intent: refactor
summary: Введены StemCellFactory и StemCellRecord.
*/
/* neira:meta
id: NEI-20250215-factory-watch
intent: refactor
summary: Добавлены вызовы nervous_system::watch и immune_system::observe при создании записи.
*/
/* neira:meta
id: NEI-20251227-factory-event-bus
intent: refactor
summary: Прямые вызовы watch/observe убраны в пользу событий.
*/

use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use chrono::{DateTime, Utc};

use crate::action_cell::ActionCell;
use crate::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use crate::cell_registry::CellRegistry;
use crate::cell_template::CellTemplate;
use crate::factory::format_state_local as _format_state_local_import;
use jsonschema_valid::ValidationError;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StemCellState {
    Draft,
    Canary,
    Experimental,
    Stable,
    Disabled,
    RolledBack,
}

#[derive(Clone)]
pub struct StemCellRecord {
    pub id: String,
    pub backend: String,
    pub template_id: String,
    pub state: StemCellState,
    pub created_at: DateTime<Utc>,
}

#[derive(Default)]
pub struct StemCellFactory {
    records: RwLock<HashMap<String, StemCellRecord>>,
    adapter_enabled: bool,
}

impl StemCellFactory {
    pub fn new() -> Arc<Self> {
        let adapter_enabled = std::env::var("FACTORY_ADAPTER_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Arc::new(Self {
            records: RwLock::new(HashMap::new()),
            adapter_enabled,
        })
    }

    pub fn is_adapter_enabled(&self) -> bool {
        self.adapter_enabled
    }

    pub fn dry_run(&self, tpl: &CellTemplate) -> serde_json::Value {
        metrics::counter!("factory_dryrun_requests_total").increment(1);
        // Минимальный отчёт: линки, тип, риск (нет исполнения)
        serde_json::json!({
            "ok": true,
            "report": {
                "analysis_type": tpl.analysis_type,
                "links": tpl.links,
                "risks": [],
            }
        })
    }

    /* neira:meta
    id: NEI-20260514-preflight-call
    intent: code
    summary: Добавлен вызов immune_system::preflight_check при создании записи.
    */
    pub fn create_record(
        &self,
        backend: &str,
        tpl: &CellTemplate,
    ) -> Result<StemCellRecord, ValidationError> {
        let rec = StemCellRecord {
            id: format!("{}:{}", backend, tpl.id),
            backend: backend.to_string(),
            template_id: tpl.id.clone(),
            state: StemCellState::Draft,
            created_at: Utc::now(),
        };
        crate::immune_system::preflight_check(&rec)?;
        self.records
            .write()
            .unwrap()
            .insert(rec.id.clone(), rec.clone());
        metrics::counter!("factory_cells_created_total").increment(1);
        let _ = Self::audit_log(&serde_json::json!({
            "ts": Utc::now().to_rfc3339(),
            "event": "factory.create",
            "id": rec.id,
            "backend": backend,
            "template_id": tpl.id
        }));
        Ok(rec)
    }

    pub fn advance(&self, id: &str) -> Option<StemCellState> {
        let mut map = self.records.write().unwrap();
        if let Some(rec) = map.get_mut(id) {
            rec.state = match rec.state {
                StemCellState::Draft => StemCellState::Canary,
                StemCellState::Canary => StemCellState::Experimental,
                StemCellState::Experimental => StemCellState::Stable,
                s => s,
            };
            metrics::counter!("factory_approvals_total").increment(1);
            let _ = Self::audit_log(&serde_json::json!({
                "ts": Utc::now().to_rfc3339(),
                "event": "factory.approve",
                "id": id,
                "state": format!("{}", _format_state_local_import(rec.state)),
            }));
            return Some(rec.state);
        }
        None
    }

    pub fn disable(&self, id: &str) -> Option<StemCellState> {
        let mut map = self.records.write().unwrap();
        if let Some(rec) = map.get_mut(id) {
            rec.state = StemCellState::Disabled;
            metrics::counter!("factory_rollbacks_total").increment(1);
            let _ = Self::audit_log(&serde_json::json!({
                "ts": Utc::now().to_rfc3339(),
                "event": "factory.disable",
                "id": id
            }));
            return Some(rec.state);
        }
        None
    }

    pub fn rollback(&self, id: &str) -> Option<StemCellState> {
        let mut map = self.records.write().unwrap();
        if let Some(rec) = map.get_mut(id) {
            rec.state = StemCellState::RolledBack;
            metrics::counter!("factory_rollbacks_total").increment(1);
            let _ = Self::audit_log(&serde_json::json!({
                "ts": Utc::now().to_rfc3339(),
                "event": "factory.rollback",
                "id": id
            }));
            return Some(rec.state);
        }
        None
    }

    /* neira:meta
    id: NEI-20250215-factory-auto-responses
    intent: code
    summary: Добавлены auto_heal и auto_rollback для реакций immune_system.
    */
    /* neira:meta
    id: NEI-20250310-factory-auto-failure-metrics
    intent: code
    summary: Добавлены счётчики неудачных auto_heal и auto_rollback.
    */
    /* neira:meta
    id: NEI-20250320-factory-auto-response-duration
    intent: code
    summary: Замеряем длительность auto_heal и auto_rollback.
    */
    pub fn auto_heal(&self, id: &str) -> Option<StemCellState> {
        let start = Instant::now();
        let res = self.disable(id);
        if let Some(_) = res {
            metrics::counter!("factory_auto_heals_total").increment(1);
            let _ = Self::audit_log(&serde_json::json!({
                "ts": Utc::now().to_rfc3339(),
                "event": "factory.auto_heal",
                "id": id
            }));
        } else {
            metrics::counter!("factory_auto_heal_failures_total").increment(1);
        }
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        metrics::histogram!(
            "factory_auto_response_duration_ms",
            "action" => "heal"
        )
        .record(elapsed_ms);
        res
    }

    pub fn auto_rollback(&self, id: &str) -> Option<StemCellState> {
        let start = Instant::now();
        let res = self.rollback(id);
        if let Some(_) = res {
            metrics::counter!("factory_auto_rollbacks_total").increment(1);
            let _ = Self::audit_log(&serde_json::json!({
                "ts": Utc::now().to_rfc3339(),
                "event": "factory.auto_rollback",
                "id": id
            }));
        } else {
            metrics::counter!("factory_auto_rollback_failures_total").increment(1);
        }
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        metrics::histogram!(
            "factory_auto_response_duration_ms",
            "action" => "rollback"
        )
        .record(elapsed_ms);
        res
    }

    pub fn counts(&self) -> (usize, usize) {
        let map = self.records.read().unwrap();
        let total = map.len();
        let active = map
            .values()
            .filter(|r| {
                matches!(
                    r.state,
                    StemCellState::Draft
                        | StemCellState::Canary
                        | StemCellState::Experimental
                        | StemCellState::Stable
                )
            })
            .count();
        (total, active)
    }

    fn audit_log(value: &serde_json::Value) -> std::io::Result<()> {
        let dir = std::path::Path::new("logs");
        let _ = std::fs::create_dir_all(dir);
        let path = dir.join("factory_audit.ndjson");
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        writeln!(f, "{}", value.to_string())
    }
}

// Простейший фабрикатор (Adapter‑only) — ActionCell заглушка
pub struct FabricatorCell;

impl ActionCell for FabricatorCell {
    fn id(&self) -> &str {
        "factory.adapter"
    }
    fn preload(&self, _triggers: &[String], _memory: &Arc<crate::memory_cell::MemoryCell>) {}
}

impl Default for FabricatorCell {
    fn default() -> Self {
        Self
    }
}

// Selector: анализатор reuse vs create
pub struct SelectorCell {
    registry: Arc<CellRegistry>,
}

impl SelectorCell {
    pub fn new(registry: Arc<CellRegistry>) -> Self {
        Self { registry }
    }
}

impl AnalysisCell for SelectorCell {
    fn id(&self) -> &str {
        "factory.selector"
    }
    fn analysis_type(&self) -> &str {
        "factory"
    }
    fn status(&self) -> CellStatus {
        CellStatus::Active
    }
    fn links(&self) -> &[String] {
        &[]
    }
    fn confidence_threshold(&self) -> f32 {
        0.0
    }
    fn analyze(&self, input: &str, _cancel_token: &CancellationToken) -> AnalysisResult {
        // Расширенные правила (минимум): prefer_id, allowed_types, blocked_types, prefer_version
        let parsed: serde_json::Value =
            serde_json::from_str(input).unwrap_or(serde_json::json!({}));
        let want_id = parsed
            .get("prefer_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let allowed_types: Vec<String> = parsed
            .get("allowed_types")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| env_list("SELECTOR_ALLOWED_TYPES"));
        let blocked_types: Vec<String> = parsed
            .get("blocked_types")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| env_list("SELECTOR_BLOCKED_TYPES"));
        // Эвристика reuse: если id есть и тип не заблокирован — reuse, иначе create
        let mut decision = "create".to_string();
        let mut explain = String::new();
        if !want_id.is_empty() && self.registry.get_analysis_cell(want_id).is_some() {
            decision = "reuse".to_string();
            explain = format!("Reuse analysis cell: {}", want_id);
        }
        if let Some(atype) = parsed.get("analysis_type").and_then(|v| v.as_str()) {
            if !allowed_types.is_empty() && !allowed_types.iter().any(|x| x == atype) {
                decision = "create".to_string();
                explain = format!("Type {} not in allowed types", atype);
            }
            if blocked_types.iter().any(|x| x == atype) {
                decision = "create".to_string();
                explain = format!("Type {} is blocked", atype);
            }
        }
        let mut r = AnalysisResult::new(self.id(), decision, vec![]);
        if !explain.is_empty() {
            r.explanation = Some(explain);
        }
        r
    }
    fn explain(&self) -> String {
        "Select reuse vs create".into()
    }
}

fn env_list(key: &str) -> Vec<String> {
    std::env::var(key)
        .ok()
        .map(|s| {
            s.split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

// local helper for audit logs
pub(crate) fn format_state_local(st: StemCellState) -> &'static str {
    match st {
        StemCellState::Draft => "draft",
        StemCellState::Canary => "canary",
        StemCellState::Experimental => "experimental",
        StemCellState::Stable => "stable",
        StemCellState::Disabled => "disabled",
        StemCellState::RolledBack => "rolled_back",
    }
}

// Adapter Contracts: единый интерфейс для адаптеров
pub trait AdapterBackend {
    fn validate(&self) -> Result<(), String>;
    fn register(&self, registry: &CellRegistry) -> Result<(), String>;
    fn ns_is_hooks(&self) -> Result<(), String>;
}

pub struct CellTemplateAdapter<'a> {
    pub tpl: &'a CellTemplate,
}

impl<'a> AdapterBackend for CellTemplateAdapter<'a> {
    fn validate(&self) -> Result<(), String> {
        if self.tpl.id.trim().is_empty() {
            return Err("invalid_template: empty id".into());
        }
        if self.tpl.analysis_type.trim().is_empty() {
            return Err("invalid_template: empty analysis_type".into());
        }
        Ok(())
    }
    fn register(&self, registry: &CellRegistry) -> Result<(), String> {
        registry.register_template(self.tpl.clone())
    }
    fn ns_is_hooks(&self) -> Result<(), String> {
        Ok(())
    }
}
