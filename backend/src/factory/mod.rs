/* neira:meta
id: NEI-20250923-factory-core
intent: code
summary: |
  Минимальный каркас фабрики узлов: сервис записей, простейший Fabricator (Adapter‑only) и Selector (reuse vs create) без исполнения кода.
*/

use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, RwLock};

use chrono::{DateTime, Utc};

use crate::action_node::ActionNode;
use crate::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use crate::node_registry::NodeRegistry;
use crate::factory::format_state_local as _format_state_local_import;
use crate::node_template::NodeTemplate;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FabricationState {
    Draft,
    Canary,
    Experimental,
    Stable,
    Disabled,
    RolledBack,
}

#[derive(Clone)]
pub struct FactoryRecord {
    pub id: String,
    pub backend: String,
    pub template_id: String,
    pub state: FabricationState,
    pub created_at: DateTime<Utc>,
}

#[derive(Default)]
pub struct FactoryService {
    records: RwLock<HashMap<String, FactoryRecord>>,
    adapter_enabled: bool,
}

impl FactoryService {
    pub fn new() -> Arc<Self> {
        let adapter_enabled = std::env::var("FACTORY_ADAPTER_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Arc::new(Self {
            records: RwLock::new(HashMap::new()),
            adapter_enabled,
        })
    }

    pub fn is_adapter_enabled(&self) -> bool { self.adapter_enabled }

    pub fn dry_run(&self, tpl: &NodeTemplate) -> serde_json::Value {
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

    pub fn create_record(&self, backend: &str, tpl: &NodeTemplate) -> FactoryRecord {
        let rec = FactoryRecord {
            id: format!("{}:{}", backend, tpl.id),
            backend: backend.to_string(),
            template_id: tpl.id.clone(),
            state: FabricationState::Draft,
            created_at: Utc::now(),
        };
        self.records
            .write()
            .unwrap()
            .insert(rec.id.clone(), rec.clone());
        metrics::counter!("factory_nodes_created_total").increment(1);
        let _ = Self::audit_log(&serde_json::json!({
            "ts": Utc::now().to_rfc3339(),
            "event": "factory.create",
            "id": rec.id,
            "backend": backend,
            "template_id": tpl.id
        }));
        rec
    }

    pub fn advance(&self, id: &str) -> Option<FabricationState> {
        let mut map = self.records.write().unwrap();
        if let Some(rec) = map.get_mut(id) {
            rec.state = match rec.state {
                FabricationState::Draft => FabricationState::Canary,
                FabricationState::Canary => FabricationState::Experimental,
                FabricationState::Experimental => FabricationState::Stable,
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

    pub fn disable(&self, id: &str) -> Option<FabricationState> {
        let mut map = self.records.write().unwrap();
        if let Some(rec) = map.get_mut(id) {
            rec.state = FabricationState::Disabled;
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

    pub fn rollback(&self, id: &str) -> Option<FabricationState> {
        let mut map = self.records.write().unwrap();
        if let Some(rec) = map.get_mut(id) {
            rec.state = FabricationState::RolledBack;
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

    pub fn counts(&self) -> (usize, usize) {
        let map = self.records.read().unwrap();
        let total = map.len();
        let active = map.values().filter(|r| matches!(r.state, FabricationState::Draft|FabricationState::Canary|FabricationState::Experimental|FabricationState::Stable)).count();
        (total, active)
    }

    fn audit_log(value: &serde_json::Value) -> std::io::Result<()> {
        let dir = std::path::Path::new("logs");
        let _ = std::fs::create_dir_all(dir);
        let path = dir.join("factory_audit.ndjson");
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(f, "{}", value.to_string())
    }
}

// Простейший фабрикатор (Adapter‑only) — ActionNode заглушка
pub struct FabricatorNode;

impl ActionNode for FabricatorNode {
    fn id(&self) -> &str { "factory.adapter" }
    fn preload(&self, _triggers: &[String], _memory: &Arc<crate::memory_node::MemoryNode>) {}
}

impl Default for FabricatorNode { fn default() -> Self { Self } }

// Selector: анализатор reuse vs create
pub struct SelectorNode {
    registry: Arc<NodeRegistry>,
}

impl SelectorNode {
    pub fn new(registry: Arc<NodeRegistry>) -> Self { Self { registry } }
}

impl AnalysisNode for SelectorNode {
    fn id(&self) -> &str { "factory.selector" }
    fn analysis_type(&self) -> &str { "factory" }
    fn status(&self) -> NodeStatus { NodeStatus::Active }
    fn links(&self) -> &[String] { &[] }
    fn confidence_threshold(&self) -> f32 { 0.0 }
    fn analyze(&self, input: &str, _cancel_token: &CancellationToken) -> AnalysisResult {
        // Расширенные правила (минимум): prefer_id, allowed_types, blocked_types, prefer_version
        let parsed: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::json!({}));
        let want_id = parsed.get("prefer_id").and_then(|v| v.as_str()).unwrap_or("");
        let allowed_types: Vec<String> = parsed
            .get("allowed_types").and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(|| env_list("SELECTOR_ALLOWED_TYPES"));
        let blocked_types: Vec<String> = parsed
            .get("blocked_types").and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(|| env_list("SELECTOR_BLOCKED_TYPES"));
        // Эвристика reuse: если id есть и тип не заблокирован — reuse, иначе create
        let mut decision = "create".to_string();
        let mut explain = String::new();
        if !want_id.is_empty() && self.registry.get_analysis_node(want_id).is_some() {
            decision = "reuse".to_string();
            explain = format!("Reuse analysis node: {}", want_id);
        }
        if let Some(atype) = parsed.get("analysis_type").and_then(|v| v.as_str()) {
            if !allowed_types.is_empty() && !allowed_types.iter().any(|x| x==atype) {
                decision = "create".to_string();
                explain = format!("Type {} not in allowed types", atype);
            }
            if blocked_types.iter().any(|x| x==atype) {
                decision = "create".to_string();
                explain = format!("Type {} is blocked", atype);
            }
        }
        let mut r = AnalysisResult::new(self.id(), decision, vec![]);
        if !explain.is_empty() { r.explanation = Some(explain); }
        r
    }
    fn explain(&self) -> String { "Select reuse vs create".into() }
}

fn env_list(key: &str) -> Vec<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect())
        .unwrap_or_default()
}

// local helper for audit logs
pub(crate) fn format_state_local(st: FabricationState) -> &'static str {
    match st {
        FabricationState::Draft => "draft",
        FabricationState::Canary => "canary",
        FabricationState::Experimental => "experimental",
        FabricationState::Stable => "stable",
        FabricationState::Disabled => "disabled",
        FabricationState::RolledBack => "rolled_back",
    }
}

// Adapter Contracts: единый интерфейс для адаптеров
pub trait AdapterBackend {
    fn validate(&self) -> Result<(), String>;
    fn register(&self, registry: &NodeRegistry) -> Result<(), String>;
    fn ns_is_hooks(&self) -> Result<(), String>;
}

pub struct NodeTemplateAdapter<'a> {
    pub tpl: &'a NodeTemplate,
}

impl<'a> AdapterBackend for NodeTemplateAdapter<'a> {
    fn validate(&self) -> Result<(), String> {
        if self.tpl.id.trim().is_empty() { return Err("invalid_template: empty id".into()); }
        if self.tpl.analysis_type.trim().is_empty() { return Err("invalid_template: empty analysis_type".into()); }
        Ok(())
    }
    fn register(&self, registry: &NodeRegistry) -> Result<(), String> {
        registry.register_template(self.tpl.clone())
    }
    fn ns_is_hooks(&self) -> Result<(), String> { Ok(()) }
}
