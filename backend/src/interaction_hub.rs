use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::action::diagnostics_node::DiagnosticsNode;
use crate::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use crate::context::context_storage::ContextStorage;
use crate::idempotent_store::IdempotentStore;
use crate::system::{host_metrics::HostMetrics, io_watcher::IoWatcher};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU64, Ordering};
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
    metrics: Arc<MetricsCollectorNode>,
    diagnostics: Arc<DiagnosticsNode>,
    trigger_detector: Arc<TriggerDetector>,
    scheduler: RwLock<TaskScheduler>,
    allowed_tokens: RwLock<Vec<String>>,
    rate: RwLock<std::collections::HashMap<String, (u64, u32)>>,
    rate_limit_per_min: u32,
    rate_key_mode: RateKeyMode,
    requests: RwLock<LruCache<String, String>>,
    idem: Option<IdempotentStore>,
    persist_require_session_id: bool,
    _host_metrics_interval_ms: u64,
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

impl InteractionHub {
    pub fn new(
        registry: Arc<NodeRegistry>,
        memory: Arc<MemoryNode>,
        metrics: Arc<MetricsCollectorNode>,
        diagnostics: Arc<DiagnosticsNode>,
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
        let idem_persist = std::env::var("IDEMPOTENT_PERSIST")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
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
        let persist_require_session_id = std::env::var("PERSIST_REQUIRE_SESSION_ID")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let host_metrics_interval_ms = std::env::var("HOST_METRICS_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30_000);
        let io_watcher_enabled = std::env::var("IO_WATCHER_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let io_watcher_threshold_ms = std::env::var("IO_WATCHER_THRESHOLD_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);

        registry.register_action_node(metrics.clone());
        registry.register_action_node(diagnostics.clone());

        let hub = Self {
            registry,
            memory,
            metrics: metrics.clone(),
            diagnostics,
            trigger_detector: Arc::new(TriggerDetector::default()),
            scheduler: RwLock::new(TaskScheduler::default()),
            allowed_tokens: RwLock::new(Vec::new()),
            rate: RwLock::new(std::collections::HashMap::new()),
            rate_limit_per_min,
            rate_key_mode,
            requests: RwLock::new(LruCache::new(NonZeroUsize::new(10_000).unwrap())),
            idem,
            persist_require_session_id,
            _host_metrics_interval_ms: host_metrics_interval_ms,
        };

        // Spawn host metrics polling loop
        let mut host_metrics = HostMetrics::new(metrics.clone());
        tokio::spawn(async move {
            let mut interval_timer = interval(Duration::from_millis(host_metrics_interval_ms));
            loop {
                interval_timer.tick().await;
                host_metrics.poll();
            }
        });

        // Optionally spawn IO watcher
        if io_watcher_enabled {
            let watcher = IoWatcher::new(metrics, io_watcher_threshold_ms);
            tokio::spawn(async move {
                watcher.run().await;
            });
        }

        hub
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

    pub fn check_auth(&self, token: &str) -> bool {
        self.authorize(token)
    }

    pub fn add_trigger_keyword(&self, keyword: impl Into<String>) {
        self.trigger_detector.add_keyword(keyword.into());
    }

    pub async fn chat(
        &self,
        node_id: &str,
        chat_id: &str,
        session_id: Option<String>,
        message: &str,
        storage: &dyn ContextStorage,
        auth: &str,
        persist: bool,
        request_id: Option<String>,
    ) -> Result<ChatOutput, String> {
        metrics::counter!("chat_requests_total").increment(1);
        if !self.authorize(auth) {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("unauthorized".into());
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
        {
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
        }

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
                        while let Some(c2) = it.next() {
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
                        while let Some(c2) = it.next() {
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
        // Triggers integration: preload action nodes
        for node in self.registry.action_nodes() {
            node.preload(&triggers, &self.memory);
        }

        // Metrics for incoming message
        // metrics could be recorded here via `metrics` crate

        let node = self.registry.get_chat_node(node_id).ok_or_else(|| {
            metrics::counter!("chat_errors_total").increment(1);
            "chat node not found".to_string()
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

        let t0 = Instant::now();

        let response = node
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
                metrics::counter!("analysis_errors_total").increment(1);
                info!("analysis {} timed out", id);
                Some(r)
            }
            _ = cancel_token.cancelled() => {
                let mut r = AnalysisResult::new(id, "", vec![]);
                r.status = NodeStatus::Error;
                self.memory.save_checkpoint(id, &r);
                metrics::counter!("analysis_errors_total").increment(1);
                info!("analysis {} cancelled", id);
                Some(r)
            }
            res = handle => {
                if let Ok(result) = res {
                    let elapsed = start.elapsed().as_millis();
                    if result.status == NodeStatus::Error {
                        metrics::counter!("analysis_errors_total").increment(1);
                        self.memory.save_checkpoint(id, &result);
                    } else {
                        self.memory.push_metrics(&result);
                        self.metrics.record(MetricsRecord {
                            id: result.id.clone(),
                            metrics: result.quality_metrics.clone(),
                        });
                        self.memory.update_time(id, elapsed);
                        let mem = self.memory.clone();
                        let rid = id.to_string();
                        mem.recalc_priority_async(rid);
                    }
                    metrics::histogram!("analysis_node_request_duration_ms")
                        .record(elapsed as f64);
                    metrics::histogram!("analysis_node_request_duration_ms_p95")
                        .record(elapsed as f64);
                    metrics::histogram!("analysis_node_request_duration_ms_p99")
                        .record(elapsed as f64);
                    info!(analysis_id=%id, duration_ms=elapsed, "analysis completed");
                    Some(result)
                } else {
                    metrics::counter!("analysis_errors_total").increment(1);
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

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
