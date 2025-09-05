/* neira:meta
id: NEI-20250922-adaptive-queues
intent: code
summary: |
  Очереди анализа выбирают адаптивные пороги на основе истории и
  переопределяются через переменные окружения.
*/
/* neira:meta
id: NEI-20250220-env-flag-hub
intent: refactor
summary: Несколько флагов хаба парсятся через env_flag.
*/
/* neira:meta
id: NEI-20250214-watchdog-refactor
intent: refactor
summary: Логика watchdog вынесена в модуль nervous_system::watchdog.
*/
/* neira:meta
id: NEI-20250316-stemcell-rename
intent: refactor
summary: Обновлены ссылки на StemCellFactory и связанные типы.
*/
/* neira:meta
id: NEI-20250215-immune-import-hub
intent: refactor
summary: Добавлен импорт immune_system.
*/
/* neira:meta
id: NEI-20250902-host-metrics-factory
intent: refactor
summary: HostMetrics теперь принимает фабрику для учёта новых клеток.
*/
/* neira:meta
id: NEI-20240607-probe-stop
intent: feature
summary: SynapseHub хранит токены проб и останавливает их при завершении работы.
*/
/* neira:meta
id: NEI-20250226-synapse-flow
intent: feature
summary: SynapseHub использует DataFlowController для маршрутизации задач и событий.
*/
/* neira:meta
id: NEI-20260522-flow-consumer
intent: fix
summary: Подписчик DataFlowController сохраняет приёмник и выводит FlowMessage через tracing.
*/
/* neira:meta
id: NEI-20260614-brain-loop-init
intent: feature
summary: Запуск brain_loop обрабатывает FlowMessage и активирует клетки.
*/
/* neira:meta
id: NEI-20270310-local-analysis
intent: refactor
summary: Анализ выполняется локально без уведомления brain_loop.
*/
/* neira:meta
id: NEI-20250224-blocking-analyze
intent: fix
summary: Анализ выполняется в отдельном блокирующем пуле tokio::task.
*/
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::action::diagnostics_cell::DiagnosticsCell;
use crate::action::metrics_collector_cell::{MetricsCollectorCell, MetricsRecord};
use crate::analysis_cell::QualityMetrics;
use crate::circulatory_system::DataFlowController;
use crate::config::Config;
use crate::context::context_storage::{ChatMessage, ContextStorage, Role};
use crate::event_bus::{CellCreated, EventBus, OrganBuilt};
use crate::factory::{FabricatorCell, SelectorCell, StemCellFactory};
use crate::hearing;
use crate::idempotent_store::IdempotentStore;
use crate::immune_system::ImmuneSystemSubscriber;
use crate::nervous_system::{
    host_metrics::HostMetrics, io_watcher::IoWatcher, watchdog::Watchdog, NervousSystemSubscriber,
    SystemProbe,
};
use crate::organ_builder::{OrganBuilder, OrganState};
use crate::security::integrity_checker_cell::IntegrityCheckerCell;
use crate::security::quarantine_cell::QuarantineCell;
use crate::security::safe_mode_controller::SafeModeController;
use jsonschema_valid::ValidationError;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::{interval, sleep};
use tokio_util::sync::CancellationToken;

use crate::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use crate::brain::{Brain, BrainSubscriber};
use crate::cell_registry::CellRegistry;
use crate::memory_cell::MemoryCell;
use crate::queue_config::QueueConfig;
use crate::task_scheduler::TaskScheduler;
use crate::trigger_detector::TriggerDetector;
use serde_json::json;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Read,
    Write,
    Admin,
}

#[derive(Clone, Debug)]
struct TokenInfo {
    scopes: Vec<Scope>,
}

struct ProbeHandle {
    handle: JoinHandle<()>,
    token: CancellationToken,
}

pub struct SynapseHub {
    pub registry: Arc<CellRegistry>,
    pub memory: Arc<MemoryCell>,
    metrics: Arc<MetricsCollectorCell>,
    trigger_detector: Arc<TriggerDetector>,
    pub(crate) scheduler: Arc<RwLock<TaskScheduler>>,
    queue_cfg: RwLock<QueueConfig>,
    allowed_tokens: RwLock<std::collections::HashMap<String, TokenInfo>>,
    rate: RwLock<std::collections::HashMap<String, (u64, u32)>>,
    rate_limit_per_min: u32,
    rate_key_mode: RateKeyMode,
    requests: RwLock<LruCache<String, String>>,
    idem: Option<IdempotentStore>,
    persist_require_session_id: bool,
    probe_handles: RwLock<std::collections::HashMap<String, ProbeHandle>>,
    io_watcher_threshold_ms: u64,
    safe_mode: Arc<SafeModeController>,
    cancels:
        RwLock<std::collections::HashMap<(String, String), tokio_util::sync::CancellationToken>>,
    analysis_cancels:
        RwLock<std::collections::HashMap<String, tokio_util::sync::CancellationToken>>,
    // Tracing store (in-memory, optional)
    traces: RwLock<std::collections::HashMap<String, Vec<serde_json::Value>>>,
    trace_enabled: AtomicBool,
    trace_max_events: usize,
    // Factory service (adapter-only for now)
    factory: Arc<StemCellFactory>,
    organ_builder: Arc<OrganBuilder>,
    event_bus: Arc<EventBus>,
    flow: Arc<DataFlowController>,
    brain: Arc<Brain>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RateKeyMode {
    Auth,
    Chat,
    Session,
}

pub struct ChatOutput {
    pub response: String,
    pub session_id: Option<String>,
    pub idempotent: bool,
}

impl SynapseHub {
    pub fn new(
        registry: Arc<CellRegistry>,
        memory: Arc<MemoryCell>,
        metrics: Arc<MetricsCollectorCell>,
        diagnostics: Arc<DiagnosticsCell>,
        config: &Config,
    ) -> Self {
        let rate_limit_per_min = std::env::var("CHAT_RATE_LIMIT_PER_MIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(120);
        let rate_key = std::env::var("CHAT_RATE_KEY").unwrap_or_else(|_| "auth".into());
        let rate_key_mode = match rate_key.to_lowercase().as_str() {
            "auth" => RateKeyMode::Auth,
            "chat" => RateKeyMode::Chat,
            "session" => RateKeyMode::Session,
            _ => RateKeyMode::Auth,
        };
        let idem_persist = crate::config::env_flag("IDEMPOTENT_PERSIST", false);
        let idem = if idem_persist {
            let dir = std::env::var("IDEMPOTENT_STORE_DIR").unwrap_or_else(|_| "context".into());
            let ttl = std::env::var("IDEMPOTENT_TTL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(86_400);
            Some(IdempotentStore::new(dir, ttl))
        } else {
            None
        };
        let persist_require_session_id =
            crate::config::env_flag("PERSIST_REQUIRE_SESSION_ID", false);
        let io_watcher_threshold_ms = std::env::var("IO_WATCHER_THRESHOLD_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        let host_metrics_enabled = config.probes.get("host_metrics").is_none_or(|p| p.enabled);
        let io_watcher_enabled = config.probes.get("io_watcher").is_some_and(|p| p.enabled);

        registry.register_action_cell(metrics.clone());
        registry.register_action_cell(diagnostics.clone());
        registry.register_action_cell(Arc::new(
            crate::nervous_system::base_path_resolver::BasePathResolverCell::new(),
        ));
        let safe_mode = SafeModeController::new();
        let (quarantine, quarantine_tx, _dev_rx) = QuarantineCell::new(safe_mode.clone());
        registry.register_action_cell(quarantine);
        registry.register_action_cell(IntegrityCheckerCell::new(memory.clone(), quarantine_tx));

        let queue_cfg = QueueConfig::new(&memory);

        let (data_flow, df_rx) = DataFlowController::new();
        let event_bus = EventBus::new();
        event_bus.attach_flow_controller(data_flow.clone());
        /* neira:meta
        id: NEI-20240930-brain-subscriber-hook
        intent: feat
        summary: Подписывает BrainSubscriber на события EventBus.
        */
        event_bus.subscribe(Arc::new(BrainSubscriber::new(data_flow.clone())));
        event_bus.subscribe(Arc::new(NervousSystemSubscriber));
        event_bus.subscribe(Arc::new(ImmuneSystemSubscriber));

        let scheduler = Arc::new(RwLock::new(TaskScheduler::default()));
        scheduler
            .write()
            .unwrap()
            .set_flow_controller(data_flow.clone());

        /* neira:meta
        id: NEI-20240821-brain-metrics-call
        intent: refactor
        summary: Передаёт MetricsCollectorCell в Brain для публикации метрик.
        */
        let brain = Arc::new(Brain::new(
            df_rx,
            data_flow.clone(),
            registry.clone(),
            scheduler.clone(),
            event_bus.clone(),
            metrics.clone(),
        ));

        let hub = Self {
            registry,
            memory,
            metrics: metrics.clone(),
            trigger_detector: Arc::new(TriggerDetector::default()),
            scheduler: scheduler.clone(),
            queue_cfg: RwLock::new(queue_cfg),
            allowed_tokens: RwLock::new(std::collections::HashMap::new()),
            rate: RwLock::new(std::collections::HashMap::new()),
            rate_limit_per_min,
            rate_key_mode,
            requests: RwLock::new(LruCache::new(NonZeroUsize::new(10_000).unwrap())),
            idem,
            persist_require_session_id,
            probe_handles: RwLock::new(std::collections::HashMap::new()),
            io_watcher_threshold_ms,
            safe_mode,
            cancels: RwLock::new(std::collections::HashMap::new()),
            analysis_cancels: RwLock::new(std::collections::HashMap::new()),
            traces: RwLock::new(std::collections::HashMap::new()),
            trace_enabled: AtomicBool::new(crate::config::env_flag("TRACE_ENABLED", false)),
            trace_max_events: std::env::var("TRACE_MAX_EVENTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(200),
            factory: StemCellFactory::new(),
            organ_builder: OrganBuilder::new(),
            event_bus: event_bus.clone(),
            flow: data_flow.clone(),
            brain: brain.clone(),
        };

        brain.clone().spawn();

        let flow_metrics = hub.flow.clone();
        let metrics_cell = hub.metrics.clone();
        tokio::spawn(async move {
            loop {
                let ms = metrics_cell.get_interval_ms();
                sleep(Duration::from_millis(ms)).await;
                let sent = flow_metrics.sent_count();
                let received = flow_metrics.received_count();
                metrics::gauge!("flow_messages_sent_total").set(sent as f64);
                metrics::gauge!("flow_messages_received_total").set(received as f64);
                metrics_cell.record(MetricsRecord {
                    id: "hub.flow.sent".to_string(),
                    metrics: QualityMetrics {
                        credibility: None,
                        recency_days: None,
                        demand: Some(sent as u32),
                    },
                });
                metrics_cell.record(MetricsRecord {
                    id: "hub.flow.received".to_string(),
                    metrics: QualityMetrics {
                        credibility: None,
                        recency_days: None,
                        demand: Some(received as u32),
                    },
                });
            }
        });

        // Spawn host metrics polling loop
        if host_metrics_enabled {
            let token = CancellationToken::new();
            let mut host_metrics =
                HostMetrics::new(hub.metrics.clone(), hub.factory.clone(), token.clone());
            let handle = tokio::spawn(async move {
                host_metrics.start().await;
            });
            hub.probe_handles
                .write()
                .unwrap()
                .insert("host_metrics".into(), ProbeHandle { handle, token });
        }

        // Optionally spawn IO watcher
        if io_watcher_enabled {
            let token = CancellationToken::new();
            let mut watcher =
                IoWatcher::new(hub.metrics.clone(), io_watcher_threshold_ms, token.clone());
            let handle = tokio::spawn(async move {
                watcher.start().await;
            });
            hub.probe_handles
                .write()
                .unwrap()
                .insert("io_watcher".into(), ProbeHandle { handle, token });
        }

        // Register factory helper cells (Adapter + Selector)
        hub.registry.register_action_cell(Arc::new(FabricatorCell));
        hub.registry
            .register_analysis_cell(Arc::new(SelectorCell::new(hub.registry.clone())));

        hub
    }

    pub fn toggle_probe(&self, name: &str) -> Result<bool, String> {
        let mut probes = self.probe_handles.write().unwrap();
        if let Some(probe) = probes.remove(name) {
            probe.token.cancel();
            probe.handle.abort();
            return Ok(false);
        }
        let (handle, token) = match name {
            "host_metrics" => {
                let token = CancellationToken::new();
                let mut probe =
                    HostMetrics::new(self.metrics.clone(), self.factory.clone(), token.clone());
                (tokio::spawn(async move { probe.start().await }), token)
            }
            "io_watcher" => {
                let token = CancellationToken::new();
                let mut watcher = IoWatcher::new(
                    self.metrics.clone(),
                    self.io_watcher_threshold_ms,
                    token.clone(),
                );
                (tokio::spawn(async move { watcher.start().await }), token)
            }
            _ => return Err(format!("unknown probe {name}")),
        };
        probes.insert(name.to_string(), ProbeHandle { handle, token });
        Ok(true)
    }

    /// Количество активных SSE-стримов (по зарегистрированным токенам отмены)
    pub fn active_streams(&self) -> usize {
        self.cancels.read().unwrap().len()
    }

    // Factory service accessors (adapter-only for now)
    pub fn factory_is_adapter_enabled(&self) -> bool {
        self.factory.is_adapter_enabled()
    }
    pub fn factory_dry_run(&self, tpl: &crate::cell_template::CellTemplate) -> serde_json::Value {
        self.factory.dry_run(tpl)
    }
    /* neira:meta
    id: NEI-20260514-factory-create-result
    intent: code
    summary: Возвращает Result с ошибкой валидации при создании записи.
    */
    #[allow(clippy::result_large_err)]
    pub fn factory_create(
        &self,
        backend: &str,
        tpl: &crate::cell_template::CellTemplate,
    ) -> Result<crate::factory::StemCellRecord, ValidationError> {
        let rec = self.factory.create_record(backend, tpl)?;
        self.event_bus.publish(&CellCreated {
            record: rec.clone(),
        });
        Ok(rec)
    }
    pub fn factory_advance(&self, id: &str) -> Option<crate::factory::StemCellState> {
        self.factory.advance(id)
    }
    pub fn factory_disable(&self, id: &str) -> Option<crate::factory::StemCellState> {
        self.factory.disable(id)
    }
    pub fn factory_rollback(&self, id: &str) -> Option<crate::factory::StemCellState> {
        self.factory.rollback(id)
    }
    pub fn factory_counts(&self) -> (usize, usize) {
        self.factory.counts()
    }

    // Organ builder accessors
    pub fn organ_builder_enabled(&self) -> bool {
        self.organ_builder.is_enabled()
    }
    /* neira:meta
    id: NEI-20251010-organ-builder-update
    intent: code
    summary: добавлены методы обновления и получения статусов органа.
    */
    /* neira:meta
    id: NEI-20251227-organ-built-event
    intent: code
    summary: Публикует событие OrganBuilt при запуске сборки.
    */
    pub fn organ_build(&self, tpl: serde_json::Value) -> String {
        let id = self.organ_builder.start_build(tpl);
        self.event_bus.publish(&OrganBuilt { id: id.clone() });
        id
    }

    /* neira:meta
    id: NEI-20260407-organ-list-hub
    intent: code
    summary: проксирует орган-билдер для выдачи списка органов.
    */
    pub fn organ_list(&self) -> Vec<(String, OrganState)> {
        self.organ_builder.list()
    }
    pub fn organ_status(&self, id: &str) -> Option<OrganState> {
        self.organ_builder.status(id)
    }
    pub fn organ_update_status(&self, id: &str, st: OrganState) -> Option<OrganState> {
        self.organ_builder.update_status(id, st)
    }
    /* neira:meta
    id: NEI-20260501-organ-builder-subscribe
    intent: code
    summary: проксирует подписку на события смены статуса органа.
    */
    pub fn organ_subscribe(&self) -> broadcast::Receiver<(String, OrganState)> {
        self.organ_builder.subscribe()
    }
    /* neira:meta
    id: NEI-20251115-organ-cancel-build-method
    intent: code
    summary: добавлен метод отмены сборки органа.
    */
    pub fn organ_cancel_build(&self, id: &str) -> bool {
        self.organ_builder.cancel_build(id)
    }

    /* neira:meta
    id: NEI-20251205-organ-rebuild-method
    intent: code
    summary: добавлен метод перезапуска сборки органа по шаблону.
    */
    pub fn organ_rebuild(&self, id: &str) -> bool {
        self.organ_builder.rebuild(id)
    }

    pub fn is_trace_enabled(&self) -> bool {
        self.trace_enabled.load(Ordering::Relaxed)
    }
    pub fn set_trace_enabled(&self, enabled: bool) {
        self.trace_enabled.store(enabled, Ordering::Relaxed)
    }

    /// Отмена всех активных SSE-стримов. Возвращает количество отменённых.
    pub fn cancel_all_streams(&self) -> usize {
        let mut n = 0usize;
        let mut map = self.cancels.write().unwrap();
        for (_k, token) in map.iter() {
            token.cancel();
            n += 1;
        }
        map.clear();
        n
    }

    pub fn add_auth_token(&self, token: impl Into<String>) {
        // backwards compatible: full scopes
        self.add_token_with_scopes(token, &[Scope::Read, Scope::Write, Scope::Admin]);
    }

    pub fn add_token_with_scopes(&self, token: impl Into<String>, scopes: &[Scope]) {
        let t = token.into();
        self.allowed_tokens.write().unwrap().insert(
            t,
            TokenInfo {
                scopes: scopes.to_vec(),
            },
        );
    }

    pub fn is_safe_mode(&self) -> bool {
        self.safe_mode.is_safe_mode()
    }

    pub fn trace_event(&self, request_id: Option<&str>, event: &str, data: serde_json::Value) {
        if !self.is_trace_enabled() {
            return;
        }
        let id = match request_id {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return,
        };
        let ev = json!({
            "ts_ms": (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i128),
            "event": event,
            "data": data,
        });
        let mut store = self.traces.write().unwrap();
        let list = store.entry(id).or_default();
        if list.len() >= self.trace_max_events {
            list.remove(0);
        }
        list.push(ev);
    }

    pub fn trace_dump(&self, request_id: &str) -> Option<serde_json::Value> {
        if !self.is_trace_enabled() {
            return None;
        }
        let store = self.traces.read().unwrap();
        store
            .get(request_id)
            .map(|v| json!({"request_id": request_id, "events": v}))
    }

    fn authorize(&self, token: &str) -> bool {
        self.allowed_tokens.read().unwrap().contains_key(token)
    }

    pub fn check_auth(&self, token: &str) -> bool {
        self.authorize(token)
    }

    pub fn check_scope(&self, token: &str, scope: Scope) -> bool {
        if let Some(info) = self.allowed_tokens.read().unwrap().get(token) {
            if scope == Scope::Write && self.safe_mode.is_safe_mode() {
                // in safe mode only admin can write
                return info.scopes.contains(&Scope::Admin);
            }
            info.scopes.contains(&scope) || info.scopes.contains(&Scope::Admin)
        } else {
            false
        }
    }

    pub fn add_trigger_keyword(&self, keyword: impl Into<String>) {
        self.trigger_detector.add_keyword(keyword.into());
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn chat(
        &self,
        cell_id: &str,
        chat_id: &str,
        session_id: Option<String>,
        message: &str,
        storage: &dyn ContextStorage,
        auth: &str,
        persist: bool,
        request_id: Option<String>,
        source: Option<String>,
        thread_id: Option<String>,
    ) -> Result<ChatOutput, String> {
        metrics::counter!("chat_requests_total").increment(1);
        if !self.authorize(auth) {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("unauthorized".into());
        }
        // export safe mode status as gauge 0/1 for nervous system
        metrics::gauge!("safe_mode").set(if self.safe_mode.is_safe_mode() {
            1.0
        } else {
            0.0
        });
        let will_write = persist || session_id.is_some();
        if will_write && !self.check_scope(auth, Scope::Write) {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("forbidden: write scope required".into());
        }
        if message.trim().is_empty() {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("empty message".into());
        }

        // rate limiting (per minute)
        let key = match self.rate_key_mode {
            RateKeyMode::Auth => format!("auth:{}", auth),
            RateKeyMode::Chat => format!("chat:{}", chat_id),
            RateKeyMode::Session => match &session_id {
                Some(s) => format!("session:{}:{}", chat_id, s),
                None => format!("chat:{}", chat_id),
            },
        };
        let now_min = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs())
            / 60;
        let remaining = {
            let mut map = self.rate.write().unwrap();
            let entry = map.entry(key).or_insert((now_min, 0));
            if entry.0 != now_min {
                *entry = (now_min, 0);
            }
            if entry.1 >= self.rate_limit_per_min {
                metrics::counter!("chat_errors_total").increment(1);
                return Err("rate limited".into());
            }
            entry.1 += 1;
            self.rate_limit_per_min.saturating_sub(entry.1)
        };

        // Parse training command like: "train script=... dry_run=true"
        let mut triggers = self.trigger_detector.detect(message);
        if message.to_lowercase().starts_with("train") {
            triggers.push("train".into());
            // parse key=value with quotes
            fn parse_kv(input: &str) -> Vec<(String, String)> {
                let mut out = Vec::new();
                let mut key = String::new();
                let mut val = String::new();
                let mut in_key = true;
                let mut in_val = false;
                let mut quote: Option<char> = None;
                let mut it = input.chars().peekable();
                while let Some(ch) = it.next() {
                    if in_key {
                        if ch.is_whitespace() {
                            continue;
                        }
                        key.push(ch);
                        // read until '='
                        for c2 in it.by_ref() {
                            if c2 == '=' {
                                in_key = false;
                                in_val = true;
                                break;
                            } else {
                                key.push(c2);
                            }
                        }
                    }
                    if in_val {
                        // skip possible spaces
                        while let Some(' ') = it.peek().copied() {
                            it.next();
                        }
                        // detect quote
                        if let Some(c) = it.peek().copied() {
                            if c == '"' || c == '\'' {
                                quote = Some(c);
                                it.next();
                            }
                        }
                        for c2 in it.by_ref() {
                            if let Some(q) = quote {
                                if c2 == q {
                                    break;
                                }
                            } else if c2.is_whitespace() {
                                break;
                            }
                            val.push(c2);
                        }
                        out.push((key.trim().to_string(), val.clone()));
                        key.clear();
                        val.clear();
                        in_key = true;
                        in_val = false;
                        quote = None;
                    }
                }
                out
            }
            for (k, v) in parse_kv(&message[5..]) {
                // after 'train'
                match k.to_lowercase().as_str() {
                    "script" => std::env::set_var("TRAINING_SCRIPT", v),
                    "dry_run" | "dry" => std::env::set_var(
                        "TRAINING_DRY_RUN",
                        if v.eq_ignore_ascii_case("true") || v == "1" {
                            "true"
                        } else {
                            "false"
                        },
                    ),
                    _ => {}
                }
            }
        }
        // Triggers integration: preload action cells
        for cell in self.registry.action_cells() {
            cell.preload(&triggers, &self.memory);
        }

        // Metrics for incoming message
        // metrics could be recorded here via `metrics` crate

        let cell = self.registry.get_chat_cell(cell_id).ok_or_else(|| {
            metrics::counter!("chat_errors_total").increment(1);
            "chat cell not found".to_string()
        })?;

        if let Some(req_id) = &request_id {
            let cache_key = format!(
                "{}|{}|{}",
                chat_id,
                session_id.clone().unwrap_or_else(|| "<none>".into()),
                req_id
            );
            if let Some(resp) = self.requests.write().unwrap().get(&cache_key).cloned() {
                metrics::counter!("requests_idempotent_hits").increment(1);
                return Ok(ChatOutput {
                    response: resp,
                    session_id: session_id.clone(),
                    idempotent: true,
                });
            }
            if let Some(store) = &self.idem {
                if let Some(resp) = store.get(&cache_key) {
                    metrics::counter!("requests_idempotent_hits").increment(1);
                    // also warm LRU
                    self.requests
                        .write()
                        .unwrap()
                        .put(cache_key.clone(), resp.clone());
                    return Ok(ChatOutput {
                        response: resp,
                        session_id: session_id.clone(),
                        idempotent: true,
                    });
                }
            }
        }

        if persist && session_id.is_none() && self.persist_require_session_id {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("session_id required for persist".into());
        }

        let sid_effective = if persist {
            Some(session_id.unwrap_or_else(|| {
                metrics::counter!("sessions_created_total").increment(1);
                metrics::gauge!("sessions_active").increment(1.0);
                metrics::counter!("sessions_autocreated_total").increment(1);
                format!(
                    "auto-{}-{:x}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    NEXT_ID.fetch_add(1, Ordering::Relaxed)
                )
            }))
        } else {
            session_id
        };

        // Save incoming user message when writing to a session
        if let Some(ref sid) = sid_effective {
            let msg = ChatMessage {
                role: Role::User,
                content: message.to_string(),
                timestamp_ms: (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()) as i64,
                source: Some(source.clone().unwrap_or_else(|| "user".into())),
                message_id: None,
                thread_id: thread_id.clone(),
                parent_id: None,
            };
            let _ = storage.save_message(chat_id, sid, &msg);
            hearing::info(&format!(
                "user message saved; safe_mode={} chat_id={} session_id={} source={} thread_id={} trace_id={}",
                self.safe_mode.is_safe_mode(),
                chat_id,
                sid,
                msg.source.clone().unwrap_or_default(),
                msg.thread_id.clone().unwrap_or_default(),
                request_id.clone().unwrap_or_else(|| "<none>".into())
            ));
        }

        let t0 = Instant::now();

        let response = cell
            .chat(chat_id, sid_effective.clone(), message, storage)
            .await;

        metrics::histogram!("chat_response_time_ms")
            .record((t0.elapsed().as_micros() as f64) / 1000.0);

        if let Some(req_id) = &request_id {
            let key = format!(
                "{}|{}|{}",
                chat_id,
                sid_effective.clone().unwrap_or_else(|| "<none>".into()),
                req_id
            );
            self.requests
                .write()
                .unwrap()
                .put(key.clone(), response.clone());
            if let Some(store) = &self.idem {
                store.put(&key, &response);
            }
        }

        // Metrics for response
        // metrics could be recorded here via `metrics` crate

        hearing::info(&format!(
            "chat rate updated; rate_limit={} rate_remaining={}",
            self.rate_limit_per_min, remaining
        ));
        Ok(ChatOutput {
            response,
            session_id: sid_effective,
            idempotent: false,
        })
    }

    pub async fn analyze(
        &self,
        id: &str,
        input: &str,
        auth: &str,
        cancel_token: &CancellationToken,
    ) -> Option<AnalysisResult> {
        metrics::counter!("analysis_requests_total").increment(1);
        if !self.authorize(auth) {
            metrics::counter!("analysis_errors_total").increment(1);
            return None;
        }

        let triggers = self.trigger_detector.detect(input);
        for cell in self.registry.action_cells() {
            cell.preload(&triggers, &self.memory);
        }

        let priority = self.memory.get_priority(id);
        let avg_time = self.memory.average_time_ms(id).unwrap_or(0);
        let queue = self
            .queue_cfg
            .write()
            .unwrap()
            .classify(avg_time, &self.memory);
        let (task_id, task_input) = self.scheduler.write().unwrap().enqueue_local(
            queue,
            id.to_string(),
            input.to_string(),
            priority,
            None,
            vec![id.to_string()],
        )?;
        let cell = self.registry.get_analysis_cell(&task_id)?;
        let cancel = cancel_token.clone();

        let mut handle = tokio::task::spawn_blocking(move || cell.analyze(&task_input, &cancel));

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

        // Конфигурация watchdog для узла
        let wd = Watchdog::for_cell(id, cfg.global_time_budget);
        let soft_ms = wd.soft_ms;
        let hard_ms = wd.hard_ms;
        let mut soft_fired = false;
        let result_opt: Option<AnalysisResult>;
        loop {
            if soft_fired {
                tokio::select! {
                    _ = sleep(Duration::from_millis(hard_ms.saturating_sub(soft_ms))) => {
                        cancel_token.cancel();
                        let mut r = AnalysisResult::new(id, "", vec![]);
                        r.status = CellStatus::Error;
                        self.memory.save_checkpoint(id, &r);
                        wd.hard_timeout(id);
                        metrics::counter!("analysis_errors_total").increment(1);
                        hearing::info(&format!(
                            "analysis_id={} kind=hard watchdog timeout hard; cancelled",
                            id
                        ));
                        result_opt = Some(r);
                        break;
                    }
                    _ = cancel_token.cancelled() => {
                        let mut r = AnalysisResult::new(id, "", vec![]);
                        r.status = CellStatus::Error;
                        self.memory.save_checkpoint(id, &r);
                        metrics::counter!("analysis_errors_total").increment(1);
                        hearing::info(&format!("analysis {} cancelled", id));
                        result_opt = Some(r);
                        break;
                    }
                    res = &mut handle => {
                if let Ok(mut result) = res {
                    let elapsed = start.elapsed().as_millis();
                    // step budget (env-based)
                    if let Ok(b) = std::env::var("REASONING_STEPS_BUDGET").and_then(|v| v.parse::<usize>().map_err(|_| std::env::VarError::NotPresent)) {
                        if b > 0 && result.reasoning_chain.len() > b { let _ = result.reasoning_chain.drain(b..); result.explanation = Some(format!("Ограничено по бюджету шагов: {}", b)); metrics::counter!("analysis_budget_steps_hits_total").increment(1); }
                    }
                    if result.status == CellStatus::Error {
                        metrics::counter!("analysis_errors_total").increment(1);
                        self.memory.save_checkpoint(id, &result);
                    } else {
                        self.memory.push_metrics(&result);
                                self.metrics.record(MetricsRecord { id: result.id.clone(), metrics: result.quality_metrics.clone(), });
                                self.memory.update_time(id, elapsed);
                                let mem = self.memory.clone(); let rid = id.to_string(); mem.recalc_priority_async(rid);
                            }
                            metrics::histogram!("analysis_cell_request_duration_ms").record(elapsed as f64);
                            metrics::histogram!("analysis_cell_request_duration_ms_p95").record(elapsed as f64);
                            metrics::histogram!("analysis_cell_request_duration_ms_p99").record(elapsed as f64);
                            hearing::info(&format!(
                                "analysis_id={} duration_ms={} soft_timeout=true analysis completed after soft timeout",
                                id, elapsed
                            ));
                            result_opt = Some(result);
                            break;
                        } else {
                            metrics::counter!("analysis_errors_total").increment(1);
                            result_opt = None; break;
                        }
                    }
                }
            } else {
                tokio::select! {
                    _ = sleep(Duration::from_millis(soft_ms)) => {
                        soft_fired = true;
                        wd.soft_timeout();
                        let auto_requeue = std::env::var("AUTO_REQUEUE_ON_SOFT").map(|v| v=="1"||v.eq_ignore_ascii_case("true")).unwrap_or(false);
                        if auto_requeue {
                            // Re-enqueue into Long queue with lower priority and return draft result now
                            self.scheduler.write().unwrap().enqueue(
                                crate::task_scheduler::Queue::Long,
                                id.to_string(),
                                input.to_string(),
                                crate::task_scheduler::Priority::Low,
                                None,
                                vec![id.to_string()],
                            );
                            let mut r = AnalysisResult::new(id, "", vec![]);
                            r.status = CellStatus::Draft;
                            r.explanation = Some("Re-queued to long after soft timeout".into());
                            self.memory.save_checkpoint(id, &r);
                            hearing::info(&format!(
                                "analysis_id={} kind=soft requeued=true watchdog soft timeout; re-queued to long and returning draft",
                                id
                            ));
                            result_opt = Some(r);
                            break;
                        } else {
                            hearing::info(&format!(
                                "analysis_id={} kind=soft watchdog soft timeout; allowing grace until hard",
                                id
                            ));
                            continue;
                        }
                    }
                    _ = cancel_token.cancelled() => {
                        let mut r = AnalysisResult::new(id, "", vec![]);
                        r.status = CellStatus::Error;
                        self.memory.save_checkpoint(id, &r);
                        metrics::counter!("analysis_errors_total").increment(1);
                        hearing::info(&format!("analysis {} cancelled", id));
                        result_opt = Some(r);
                        break;
                    }
                    res = &mut handle => {
                        if let Ok(mut result) = res {
                            let elapsed = start.elapsed().as_millis();
                            if let Ok(b) = std::env::var("REASONING_STEPS_BUDGET").and_then(|v| v.parse::<usize>().map_err(|_| std::env::VarError::NotPresent)) {
                                if b > 0 && result.reasoning_chain.len() > b { let _ = result.reasoning_chain.drain(b..); result.explanation = Some(format!("Ограничено по бюджету шагов: {}", b)); metrics::counter!("analysis_budget_steps_hits_total").increment(1); }
                            }
                            if result.status == CellStatus::Error {
                                metrics::counter!("analysis_errors_total").increment(1);
                                self.memory.save_checkpoint(id, &result);
                            } else {
                                self.memory.push_metrics(&result);
                                self.metrics.record(MetricsRecord { id: result.id.clone(), metrics: result.quality_metrics.clone(), });
                                self.memory.update_time(id, elapsed);
                                let mem = self.memory.clone(); let rid = id.to_string(); mem.recalc_priority_async(rid);
                            }
                            metrics::histogram!("analysis_cell_request_duration_ms").record(elapsed as f64);
                            metrics::histogram!("analysis_cell_request_duration_ms_p95").record(elapsed as f64);
                            metrics::histogram!("analysis_cell_request_duration_ms_p99").record(elapsed as f64);
                            hearing::info(&format!(
                                "analysis_id={} duration_ms={} soft_timeout=false analysis completed",
                                id, elapsed
                            ));
                            result_opt = Some(result);
                            break;
                        } else { metrics::counter!("analysis_errors_total").increment(1); result_opt=None; break; }
                    }
                }
            }
        }
        result_opt
    }

    pub fn resume(&self, id: &str, auth: &str) -> Option<AnalysisResult> {
        if !self.authorize(auth) {
            return None;
        }
        self.memory.load_checkpoint(id)
    }

    pub fn rate_info(
        &self,
        auth: &str,
        chat_id: &str,
        session_id: Option<&str>,
    ) -> (u32, u32, u32, String) {
        let key = match self.rate_key_mode {
            RateKeyMode::Auth => format!("auth:{}", auth),
            RateKeyMode::Chat => format!("chat:{}", chat_id),
            RateKeyMode::Session => match session_id {
                Some(s) => format!("session:{}:{}", chat_id, s),
                None => format!("chat:{}", chat_id),
            },
        };
        let now_min = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs())
            / 60;
        let used = if let Some((minute, count)) = self.rate.read().unwrap().get(&key) {
            if *minute == now_min {
                *count
            } else {
                0
            }
        } else {
            0
        };
        let limit = self.rate_limit_per_min;
        let remaining = limit.saturating_sub(used);
        (limit, remaining, used, key)
    }

    /// Регистрация нейрона в мозге
    pub fn register_neuron(&self, cell: Arc<dyn AnalysisCell + Send + Sync>) {
        self.brain.register_neuron(cell);
    }

    // SSE cancellation registry
    pub fn register_stream_cancel(
        &self,
        chat_id: &str,
        session_id: &str,
        token: tokio_util::sync::CancellationToken,
    ) {
        self.cancels
            .write()
            .unwrap()
            .insert((chat_id.to_string(), session_id.to_string()), token);
    }

    // Analysis cancellation registry
    pub fn register_analysis_cancel(&self, id: &str, token: tokio_util::sync::CancellationToken) {
        self.analysis_cancels
            .write()
            .unwrap()
            .insert(id.to_string(), token);
    }

    pub fn remove_analysis_cancel(&self, id: &str) {
        self.analysis_cancels.write().unwrap().remove(id);
    }

    pub fn cancel_analysis(&self, id: &str) -> bool {
        if let Some(tok) = self.analysis_cancels.write().unwrap().remove(id) {
            tok.cancel();
            return true;
        }
        false
    }

    pub fn cancel_stream(&self, chat_id: &str, session_id: &str) -> bool {
        if let Some(tok) = self
            .cancels
            .write()
            .unwrap()
            .remove(&(chat_id.to_string(), session_id.to_string()))
        {
            tok.cancel();
            return true;
        }
        false
    }
}

/* neira:meta
id: NEI-20241003-hub-flow-metrics
intent: feat
summary: SynapseHub периодически публикует счётчики кровотока в MetricsCollectorCell и gauge метрики.
*/

impl Drop for SynapseHub {
    fn drop(&mut self) {
        let mut probes = self.probe_handles.write().unwrap();
        for (_, probe) in probes.drain() {
            probe.token.cancel();
            probe.handle.abort();
        }
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
/* neira:meta
id: NEI-20250829-setup-meta-hub
intent: docs
scope: spinal_cord/hub
summary: |
  Политики чата: скоупы read/write/admin, idempotency (LRU+file TTL), rate-limit с ключом,
  safe-mode (write=admin), сохранение входящего user-сообщения с метаданными (source/thread_id).
links:
  - docs/api/spinal_cord.md
  - docs/reference/env.md
  - docs/reference/metrics.md
env:
  - CHAT_RATE_LIMIT_PER_MIN
  - CHAT_RATE_KEY
  - IDEMPOTENT_PERSIST
  - IDEMPOTENT_STORE_DIR
  - IDEMPOTENT_TTL_SECS
  - PERSIST_REQUIRE_SESSION_ID
metrics:
  - chat_requests_total
  - chat_errors_total
  - chat_response_time_ms
  - safe_mode
  - sessions_autocreated_total
  - requests_idempotent_hits
risks: low
safe_mode:
  affects_write: true
  requires_admin: true
i18n:
    reviewer_note: |
    Центр координации политик и лимитов. Следить за скоупами и idempotency.
*/
/* neira:meta
id: NEI-20240513-synapse-lints
intent: chore
summary: Убраны предупреждения Clippy: is_none_or/is_some_and, устранены while-let на итераторах, добавлены allow для больших ошибок и количества аргументов.
*/
