use std::sync::{Arc, Mutex};

/* neira:meta
id: NEI-20250603-axum-ws-api
intent: refactor
summary: обновлена интеграция WebSocket для axum 0.8.
*/
/* neira:meta
id: NEI-20250214-watchdog-metrics
intent: refactor
summary: Парсинг счётчиков watchdog вынесен в модуль nervous_system::watchdog.
*/
use async_stream::stream;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::HeaderMap,
    response::sse::{Event, Sse},
    routing::{delete, get, post},
    Json, Router,
};
use backend::context::context_storage::set_runtime_mask_config;
use backend::hearing;
use backend::nervous_system::anti_idle;
use backend::nervous_system::backpressure_probe::BackpressureProbe;
use backend::nervous_system::loop_detector;
use backend::nervous_system::watchdog::Watchdog;
use dotenvy::dotenv;
use futures_core::stream::Stream;
use metrics_exporter_prometheus::PrometheusBuilder;
use regex::Regex;
use std::convert::Infallible;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::error;

use backend::action::chat_node::EchoChatNode;
use backend::action::diagnostics_node::DiagnosticsNode;
use backend::action::metrics_collector_node::MetricsCollectorNode;
use backend::action_node::PreloadAction;
use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::config::Config;
use backend::context::context_storage::FileContextStorage;
use backend::factory::{AdapterBackend, FabricationState, NodeTemplateAdapter};
use backend::interaction_hub::InteractionHub;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use backend::node_template::NodeTemplate;
use backend::policy::{Capability, PolicyEngine};
use backend::security::init_config_node::InitConfigNode;
mod http {
    pub mod training_routes;
}

#[derive(Clone)]
pub struct AppState {
    pub hub: Arc<InteractionHub>,
    backpressure: Arc<BackpressureProbe>,
    storage: Arc<FileContextStorage>,
    paused: Arc<AtomicBool>,
    pause_info: Arc<Mutex<Option<(std::time::Instant, String)>>>,
    shutdown: tokio_util::sync::CancellationToken,
}

impl axum::extract::FromRef<AppState> for Arc<InteractionHub> {
    fn from_ref(state: &AppState) -> Arc<InteractionHub> {
        state.hub.clone()
    }
}

async fn register_node(
    State(state): State<AppState>,
    Json(tpl): Json<NodeTemplate>,
) -> Result<String, (axum::http::StatusCode, String)> {
    state
        .hub
        .registry
        .register_template(tpl)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    Ok("registered".to_string())
}

fn auth_from_headers(headers: &HeaderMap) -> Option<String> {
    if let Some(v) = headers.get(axum::http::header::AUTHORIZATION) {
        if let Ok(s) = v.to_str() {
            let s = s.trim();
            if let Some(rest) = s.strip_prefix("Bearer ") {
                return Some(rest.to_string());
            }
            return Some(s.to_string());
        }
    }
    if let Some(v) = headers.get("x-auth-token") {
        if let Ok(s) = v.to_str() {
            return Some(s.to_string());
        }
    }
    None
}

async fn get_node(
    State(state): State<AppState>,
    Path((id, version)): Path<(String, String)>,
) -> Result<Json<NodeTemplate>, axum::http::StatusCode> {
    match state.hub.registry.get(&id) {
        Some(tpl) if tpl.version == version => Ok(Json(tpl)),
        _ => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn get_node_latest(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<NodeTemplate>, axum::http::StatusCode> {
    state
        .hub
        .registry
        .get(&id)
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

#[derive(serde::Deserialize)]
struct FactoryBody {
    #[serde(default)]
    backend: Option<String>,
    #[serde(flatten)]
    tpl: NodeTemplate,
}

async fn factory_dryrun(
    State(state): State<AppState>,
    Json(body): Json<FactoryBody>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let backend = body.backend.as_deref().unwrap_or("adapter");
    if backend != "adapter" {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::FactoryAdapter) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    Ok(Json(state.hub.factory_dry_run(&body.tpl)))
}

async fn factory_create(
    State(state): State<AppState>,
    Json(body): Json<FactoryBody>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let backend = body.backend.as_deref().unwrap_or("adapter");
    if backend != "adapter" {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::FactoryAdapter) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    let adapter = NodeTemplateAdapter { tpl: &body.tpl };
    adapter
        .validate()
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    adapter
        .register(&state.hub.registry)
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let rec = state.hub.factory_create(backend, &body.tpl);
    Ok(Json(serde_json::json!({"id": rec.id, "state": "draft"})))
}

async fn factory_approve(
    State(state): State<AppState>,
    Path(fid): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.hub.factory_advance(&fid) {
        Some(st) => Ok(Json(
            serde_json::json!({"id": fid, "state": format_state(st)}),
        )),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn factory_disable(
    State(state): State<AppState>,
    Path(fid): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.hub.factory_disable(&fid) {
        Some(st) => Ok(Json(
            serde_json::json!({"id": fid, "state": format_state(st)}),
        )),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn factory_rollback(
    State(state): State<AppState>,
    Path(fid): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.hub.factory_rollback(&fid) {
        Some(st) => Ok(Json(
            serde_json::json!({"id": fid, "state": format_state(st)}),
        )),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

#[derive(serde::Deserialize)]
struct OrganBuildReq {
    organ_template: serde_json::Value,
    #[serde(default)]
    dryrun: bool,
}

async fn organ_build(
    State(state): State<AppState>,
    Json(body): Json<OrganBuildReq>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    if body.dryrun {
        metrics::counter!("organ_build_dryrun_total").increment(1);
        hearing::info("organ build dry-run");
        return Ok(Json(
            serde_json::json!({"organ_id": serde_json::Value::Null, "state": "dry_run"}),
        ));
    }
    let id = state.hub.organ_build(body.organ_template);
    Ok(Json(serde_json::json!({"organ_id": id, "state": "draft"})))
}

/* neira:meta
id: NEI-20260407-organs-list-route
intent: code
summary: добавлен GET /organs для выдачи id и state всех органов.
*/
async fn organs_list(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    let organs: Vec<_> = state
        .hub
        .organ_list()
        .into_iter()
        .map(|(id, st)| serde_json::json!({"id": id, "state": format_organ_state(st)}))
        .collect();
    hearing::info(&format!("organs list queried; count={}", organs.len()));
    Ok(Json(serde_json::Value::Array(organs)))
}

async fn organ_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    match state.hub.organ_status(&id) {
        Some(st) => {
            hearing::info(&format!("organ status queried; organ_id={}", id));
            Ok(Json(
                serde_json::json!({"id": id, "state": format_organ_state(st)}),
            ))
        }
        None => {
            metrics::counter!("organ_status_not_found_total").increment(1);
            hearing::warn(&format!(
                "organ status missing; organ_id={} reason=not_found",
                id
            ));
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }
}

/* neira:meta
id: NEI-20260501-organ-stream-route
intent: code
summary: добавлен WS /organs/{id}/stream для трансляции смен статуса.
*/
async fn organ_stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
    ws: WebSocketUpgrade,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    Ok(ws.on_upgrade(move |sock| organ_stream_ws(sock, id, state.hub.clone())))
}

async fn organ_stream_ws(mut socket: WebSocket, id: String, hub: Arc<InteractionHub>) {
    let mut rx = hub.organ_subscribe();
    while let Ok((oid, st)) = rx.recv().await {
        if oid == id {
            let msg = serde_json::json!({"id": oid, "state": format_organ_state(st)});
            if socket
                .send(Message::Text(msg.to_string().into()))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct OrganStatusUpdateReq {
    state: String,
}

/* neira:meta
id: NEI-20251010-organ-status-update-route
intent: code
summary: добавлен POST /organs/{id}/status для ручного изменения стадии.
*/
async fn organ_update_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<OrganStatusUpdateReq>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    /* neira:meta
    id: NEI-20250317-organ-status-update-errors
    intent: code
    summary: logs rejection reasons and maps errors to 404/409 codes.
    */
    let st = match body.state.as_str() {
        "draft" => backend::organ_builder::OrganState::Draft,
        "canary" => backend::organ_builder::OrganState::Canary,
        "experimental" => backend::organ_builder::OrganState::Experimental,
        "stable" => backend::organ_builder::OrganState::Stable,
        "failed" => backend::organ_builder::OrganState::Failed,
        _ => {
            hearing::warn(&format!(
                "organ status update rejected; organ_id={} reason=invalid_state",
                id
            ));
            return Err(axum::http::StatusCode::CONFLICT);
        }
    };
    match state.hub.organ_update_status(&id, st) {
        Some(st) => {
            hearing::info(&format!(
                "organ status updated; organ_id={} new_state={}",
                id, body.state
            ));
            Ok(Json(
                serde_json::json!({"id": id, "state": format_organ_state(st)}),
            ))
        }
        None => {
            hearing::warn(&format!(
                "organ status update rejected; organ_id={} reason=not_found",
                id
            ));
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }
}

/* neira:meta
id: NEI-20251115-organ-cancel-build-route
intent: code
summary: добавлен DELETE /organs/{id}/build для остановки сборки органа.
*/
async fn organ_cancel_build(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> axum::http::StatusCode {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return axum::http::StatusCode::FORBIDDEN;
    }
    if state.hub.organ_cancel_build(&id) {
        metrics::counter!("organ_cancel_build_total").increment(1);
        hearing::info(&format!("organ build cancelled; organ_id={}", id));
        axum::http::StatusCode::NO_CONTENT
    } else {
        metrics::counter!("organ_cancel_not_found_total").increment(1);
        hearing::warn(&format!(
            "organ build cancel missing; organ_id={} reason=not_found",
            id
        ));
        axum::http::StatusCode::NOT_FOUND
    }
}

/* neira:meta
id: NEI-20251205-organ-rebuild-route
intent: code
summary: добавлен POST /organs/{id}/rebuild для перезапуска сборки органа.
*/
async fn organ_rebuild(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pe = PolicyEngine::new();
    if let Err(_e) = pe.require_capability(&state.hub, Capability::OrgansBuilder) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    if state.hub.organ_rebuild(&id) {
        metrics::counter!("organ_rebuild_total").increment(1);
        hearing::info(&format!("organ rebuild started; organ_id={}", id));
        Ok(Json(serde_json::json!({"organ_id": id, "state": "draft"})))
    } else {
        metrics::counter!("organ_rebuild_not_found_total").increment(1);
        hearing::warn(&format!(
            "organ rebuild missing; organ_id={} reason=not_found",
            id
        ));
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

fn format_organ_state(st: backend::organ_builder::OrganState) -> &'static str {
    match st {
        backend::organ_builder::OrganState::Draft => "draft",
        backend::organ_builder::OrganState::Canary => "canary",
        backend::organ_builder::OrganState::Experimental => "experimental",
        backend::organ_builder::OrganState::Stable => "stable",
        backend::organ_builder::OrganState::Failed => "failed",
    }
}

fn format_state(st: FabricationState) -> &'static str {
    match st {
        FabricationState::Draft => "draft",
        FabricationState::Canary => "canary",
        FabricationState::Experimental => "experimental",
        FabricationState::Stable => "stable",
        FabricationState::Disabled => "disabled",
        FabricationState::RolledBack => "rolled_back",
    }
}

#[derive(serde::Deserialize)]
struct AnalysisRequest {
    id: String,
    input: String,
    auth: String,
    #[serde(default)]
    budget_ms: Option<u64>,
}

#[derive(serde::Deserialize)]
struct ResumeRequest {
    id: String,
    auth: String,
}

async fn analyze_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut req): Json<AnalysisRequest>,
) -> Result<Json<AnalysisResult>, axum::http::StatusCode> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }
    // Anti-Idle: mark user activity
    anti_idle::mark_activity();
    // backpressure throttle for analysis
    state.backpressure.throttle().await;
    let req_id = headers
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    state.hub.trace_event(
        req_id.as_deref(),
        "analysis.start",
        serde_json::json!({"id": req.id, "len": req.input.len()}),
    );
    let token = tokio_util::sync::CancellationToken::new();
    state.hub.register_analysis_cancel(&req.id, token.clone());
    // per-request budget override via JSON or header x-reasoning-budget-ms
    if req.budget_ms.is_none() {
        if let Some(h) = headers
            .get("x-reasoning-budget-ms")
            .and_then(|v| v.to_str().ok())
        {
            req.budget_ms = h.parse::<u64>().ok();
        }
    }
    if let Some(ms) = req.budget_ms {
        let t = token.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            if !t.is_cancelled() {
                t.cancel();
                metrics::counter!("analysis_budget_hits_total").increment(1);
            }
        });
    }
    let result = state
        .hub
        .analyze(&req.id, &req.input, &req.auth, &token)
        .await
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    state.hub.remove_analysis_cancel(&req.id);
    state.hub.trace_event(
        req_id.as_deref(),
        "analysis.done",
        serde_json::json!({"id": req.id}),
    );
    Ok(Json(result))
}

async fn resume_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut req): Json<ResumeRequest>,
) -> Result<Json<AnalysisResult>, axum::http::StatusCode> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }
    // Anti-Idle: mark user activity
    anti_idle::mark_activity();
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    state
        .hub
        .resume(&req.id, &req.auth)
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

#[derive(serde::Deserialize)]
struct ChatRequest {
    node_id: String,
    chat_id: String,
    session_id: Option<String>,
    message: String,
    #[serde(default)]
    auth: String,
    #[serde(default)]
    persist: bool,
    request_id: Option<String>,
    source: Option<String>,
    thread_id: Option<String>,
    budget_tokens: Option<usize>,
}

#[derive(serde::Serialize)]
struct ChatResponse {
    response: String,
    used_context: bool,
    session_id: Option<String>,
    idempotent: bool,
}

async fn chat_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut req): Json<ChatRequest>,
) -> Result<(axum::http::HeaderMap, Json<ChatResponse>), (axum::http::StatusCode, String)> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err((axum::http::StatusCode::SERVICE_UNAVAILABLE, "paused".into()));
    }
    // Anti-Idle: mark user activity
    anti_idle::mark_activity();
    // backpressure throttle for chat
    state.backpressure.throttle().await;
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    let used_context = req.session_id.is_some();
    state.hub.trace_event(
        req.request_id.as_deref(),
        "chat.start",
        serde_json::json!({"node_id": req.node_id, "chat_id": req.chat_id, "persist": req.persist}),
    );
    let out = state
        .hub
        .chat(
            &req.node_id,
            &req.chat_id,
            req.session_id.clone(),
            &req.message,
            state.storage.as_ref(),
            &req.auth,
            req.persist,
            req.request_id.clone(),
            req.source.clone(),
            req.thread_id.clone(),
        )
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    let (limit, remaining, used, key) =
        state
            .hub
            .rate_info(&req.auth, &req.chat_id, req.session_id.as_deref());
    let mut h = axum::http::HeaderMap::new();
    h.insert(
        "X-RateLimit-Limit",
        axum::http::HeaderValue::from_str(&limit.to_string()).unwrap(),
    );
    h.insert(
        "X-RateLimit-Remaining",
        axum::http::HeaderValue::from_str(&remaining.to_string()).unwrap(),
    );
    h.insert(
        "X-RateLimit-Used",
        axum::http::HeaderValue::from_str(&used.to_string()).unwrap(),
    );
    h.insert(
        "X-RateLimit-Window",
        axum::http::HeaderValue::from_static("minute"),
    );
    h.insert(
        "X-RateLimit-Key",
        axum::http::HeaderValue::from_str(&key).unwrap(),
    );
    Ok((
        h,
        Json(ChatResponse {
            response: out.response,
            used_context,
            session_id: out.session_id,
            idempotent: out.idempotent,
        }),
    ))
}

async fn get_chat_index(
    Path(chat_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let path = std::path::Path::new(&base)
        .join(&chat_id)
        .join("index.json");
    if !path.exists() {
        return Ok(Json(serde_json::json!({})));
    }
    let data = std::fs::read_to_string(&path)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let v: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(v))
}

#[derive(serde::Deserialize)]
struct SessionQuery {
    from: Option<String>,
    to: Option<String>,
    offset: Option<usize>,
    limit: Option<usize>,
    since_id: Option<u64>,
    after_ts: Option<i64>,
}

async fn get_chat_session(
    Path((chat_id, session_id)): Path<(String, String)>,
    axum::extract::Query(q): axum::extract::Query<SessionQuery>,
) -> impl axum::response::IntoResponse {
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let dir = std::path::Path::new(&base).join(&chat_id);
    let mut body = String::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        let mut files: Vec<std::path::PathBuf> = rd.flatten().map(|e| e.path()).collect();
        files.sort();
        for p in files {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name == format!("{}.ndjson", session_id)
                || (name.starts_with(&format!("{}-", session_id))
                    && (name.ends_with(".ndjson") || name.ends_with(".ndjson.gz")))
            {
                if let (Some(ref from), Some(ref to)) = (&q.from, &q.to) {
                    // filter by YYYYMMDD window for rotated files
                    let parts: Vec<&str> = name
                        .trim_end_matches(".gz")
                        .trim_end_matches(".ndjson")
                        .split('-')
                        .collect();
                    if parts.len() >= 2 {
                        let date = parts[parts.len() - 1];
                        if date < from.as_str() || date > to.as_str() {
                            continue;
                        }
                    }
                }
                if name.ends_with(".gz") {
                    use std::io::Read;
                    let data = std::fs::read(&p).unwrap_or_default();
                    let mut d = flate2::read::GzDecoder::new(&data[..]);
                    let mut s = String::new();
                    let _ = d.read_to_string(&mut s);
                    body.push_str(&s);
                } else if let Ok(s) = std::fs::read_to_string(&p) {
                    body.push_str(&s);
                }
            }
        }
    }
    // filter by since_id/after_ts
    let mut filtered = String::new();
    if q.since_id.is_some() || q.after_ts.is_some() || q.offset.is_some() || q.limit.is_some() {
        let mut lines: Vec<String> = Vec::new();
        for line in body.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if q.since_id.is_none() && q.after_ts.is_none() {
                lines.push(line.to_string());
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                let mut pass = true;
                if let Some(sid) = q.since_id {
                    pass &= v
                        .get("message_id")
                        .and_then(|x| x.as_u64())
                        .map(|id| id > sid)
                        .unwrap_or(true);
                }
                if let Some(ts) = q.after_ts {
                    pass &= v
                        .get("timestamp_ms")
                        .and_then(|x| x.as_i64())
                        .map(|t| t > ts)
                        .unwrap_or(true);
                }
                if pass {
                    lines.push(line.to_string());
                }
            }
        }
        let offset = q.offset.unwrap_or(0);
        let limit = q.limit.unwrap_or(usize::MAX);
        for (i, l) in lines.into_iter().enumerate() {
            if i < offset {
                continue;
            }
            if i >= offset + limit {
                break;
            }
            filtered.push_str(&l);
            filtered.push('\n');
        }
        return (
            [(axum::http::header::CONTENT_TYPE, "application/x-ndjson")],
            filtered,
        );
    }
    (
        [(axum::http::header::CONTENT_TYPE, "application/x-ndjson")],
        body,
    )
}

#[derive(serde::Deserialize)]
struct NewSessionRequest {
    auth: String,
    prefix: Option<String>,
}
#[derive(serde::Serialize)]
struct NewSessionResponse {
    session_id: String,
}

fn gen_session_id(prefix: Option<&str>) -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let ts = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let suf = NEXT.fetch_add(1, Ordering::Relaxed);
    match prefix {
        Some(p) => format!("{}-{}-{:x}", p, ts, suf),
        None => format!("sess-{}-{:x}", ts, suf),
    }
}

async fn new_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut req): Json<NewSessionRequest>,
) -> Result<Json<NewSessionResponse>, axum::http::StatusCode> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    if !state.hub.check_auth(&req.auth) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    if !state
        .hub
        .check_scope(&req.auth, backend::interaction_hub::Scope::Write)
    {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    let id = gen_session_id(req.prefix.as_deref());
    metrics::counter!("sessions_created_total").increment(1);
    metrics::gauge!("sessions_active").increment(1.0);
    Ok(Json(NewSessionResponse { session_id: id }))
}

#[derive(serde::Deserialize)]
struct AuthQuery {
    auth: String,
}

async fn delete_session(
    State(state): State<AppState>,
    Path((chat_id, session_id)): Path<(String, String)>,
    headers: HeaderMap,
    axum::extract::Query(mut q): axum::extract::Query<AuthQuery>,
) -> Result<(), (axum::http::StatusCode, String)> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err((axum::http::StatusCode::SERVICE_UNAVAILABLE, "paused".into()));
    }
    if q.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            q.auth = h;
        }
    }
    if !state.hub.check_auth(&q.auth) {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
    }
    if !state
        .hub
        .check_scope(&q.auth, backend::interaction_hub::Scope::Write)
    {
        return Err((axum::http::StatusCode::FORBIDDEN, "forbidden".into()));
    }
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let dir = std::path::Path::new(&base).join(&chat_id);
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name == format!("{}.ndjson", session_id)
                    || name.starts_with(&format!("{}-", session_id))
                {
                    let _ = std::fs::remove_file(&p);
                    let gz = p.with_extension("ndjson.gz");
                    let _ = std::fs::remove_file(gz);
                }
            }
        }
    }
    // update index.json
    let idx = dir.join("index.json");
    if idx.exists() {
        if let Ok(s) = std::fs::read_to_string(&idx) {
            if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&s) {
                if let Some(map) = v.as_object_mut() {
                    map.remove(&session_id);
                }
                let _ = std::fs::write(
                    &idx,
                    serde_json::to_string_pretty(&v).unwrap_or("{}".into()),
                );
            }
        }
    }
    metrics::counter!("sessions_deleted_total").increment(1);
    metrics::counter!("sessions_closed_total").increment(1);
    metrics::gauge!("sessions_active").decrement(1.0);
    Ok(())
}

#[derive(serde::Deserialize)]
struct RenameRequest {
    auth: String,
    new_session_id: String,
}

async fn rename_session(
    State(state): State<AppState>,
    Path((chat_id, session_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(mut req): Json<RenameRequest>,
) -> Result<(), (axum::http::StatusCode, String)> {
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    if !state.hub.check_auth(&req.auth) {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
    }
    if !state
        .hub
        .check_scope(&req.auth, backend::interaction_hub::Scope::Write)
    {
        return Err((axum::http::StatusCode::FORBIDDEN, "forbidden".into()));
    }
    if req.new_session_id.trim().is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "empty new_session_id".into(),
        ));
    }
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let dir = std::path::Path::new(&base).join(&chat_id);
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name == format!("{}.ndjson", session_id) {
                    let _ = std::fs::rename(&p, dir.join(format!("{}.ndjson", req.new_session_id)));
                } else if name.starts_with(&format!("{}-", session_id)) && name.ends_with(".ndjson")
                {
                    let suffix = &name[(session_id.len() + 1)..];
                    let _ =
                        std::fs::rename(&p, dir.join(format!("{}-{}", req.new_session_id, suffix)));
                } else if name.starts_with(&format!("{}-", session_id))
                    && name.ends_with(".ndjson.gz")
                {
                    let suffix = &name[(session_id.len() + 1)..];
                    let _ =
                        std::fs::rename(&p, dir.join(format!("{}-{}", req.new_session_id, suffix)));
                }
            }
        }
    }
    // update index.json: rename key
    let idx = dir.join("index.json");
    if idx.exists() {
        if let Ok(s) = std::fs::read_to_string(&idx) {
            if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&s) {
                if let Some(map) = v.as_object_mut() {
                    if let Some(val) = map.remove(&session_id) {
                        map.insert(req.new_session_id.clone(), val);
                    }
                }
                let _ = std::fs::write(
                    &idx,
                    serde_json::to_string_pretty(&v).unwrap_or("{}".into()),
                );
            }
        }
    }
    Ok(())
}

async fn chat_stream(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (axum::http::StatusCode, String)> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err((axum::http::StatusCode::SERVICE_UNAVAILABLE, "paused".into()));
    }
    // Anti-Idle: mark user activity
    anti_idle::mark_activity();
    let used_context = req.session_id.is_some();
    state.hub.trace_event(
        req.request_id.as_deref(),
        "chat.stream.start",
        serde_json::json!({"node_id": req.node_id, "chat_id": req.chat_id}),
    );
    let out = state
        .hub
        .chat(
            &req.node_id,
            &req.chat_id,
            req.session_id.clone(),
            &req.message,
            state.storage.as_ref(),
            &req.auth,
            req.persist,
            req.request_id.clone(),
            req.source.clone(),
            req.thread_id.clone(),
        )
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    let (limit, remaining, used, key) =
        state
            .hub
            .rate_info(&req.auth, &req.chat_id, req.session_id.as_deref());
    let cancel = tokio_util::sync::CancellationToken::new();
    if let Some(ref sid) = req.session_id {
        state
            .hub
            .register_stream_cancel(&req.chat_id, sid, cancel.clone());
    }
    metrics::gauge!("sse_active").increment(1.0);
    let warn_after_ms = std::env::var("SSE_WARN_AFTER_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60_000);
    let hub_for_trace = state.hub.clone();
    let req_id2 = req.request_id.clone();
    let chat_id2 = req.chat_id.clone();
    let stream = stream! {
        use std::time::Instant;
        // first send metadata event
        let budget_total = req.budget_tokens.or_else(|| std::env::var("REASONING_TOKEN_BUDGET").ok().and_then(|v| v.parse::<usize>().ok())).unwrap_or(0);
        let meta = serde_json::json!({
            "used_context": used_context,
            "session_id": out.session_id,
            "idempotent": out.idempotent,
            "source": req.source,
            "thread_id": req.thread_id,
            "rate_limit": {"limit": limit, "remaining": remaining, "used": used, "window": "minute", "key": key},
            "budget_tokens": budget_total,
        });
        yield Ok(Event::default().event("meta").data(meta.to_string()));
        // then stream chunked response by words
        let mut sent = 0usize;
        let mut chars = 0usize;
        let start = Instant::now();
        let dev_delay_ms = std::env::var("SSE_DEV_DELAY_MS").ok().and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
        let loop_enabled = std::env::var("LOOP_DETECT_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let loop_win: usize = std::env::var("LOOP_WINDOW_TOKENS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(50);
        let loop_thresh: f32 = std::env::var("LOOP_REPEAT_THRESHOLD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.6);
        let entropy_min: f32 = std::env::var("LOOP_ENTROPY_MIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);
        let mut win: std::collections::VecDeque<String> =
            std::collections::VecDeque::with_capacity(loop_win.max(1));
        for w in out.response.split_whitespace() {
            if cancel.is_cancelled() { break; }
            anti_idle::mark_activity();
            yield Ok(Event::default().event("message").data(w.to_string()));
            sent += 1;
            chars += w.len();
            if dev_delay_ms > 0 { tokio::time::sleep(std::time::Duration::from_millis(dev_delay_ms)).await; }
            if budget_total > 0 {
                let remaining = budget_total.saturating_sub(sent);
                if sent % 10 == 0 || remaining == 0 {
                    let prog = serde_json::json!({"budget_remaining": remaining});
                    yield Ok(Event::default().event("progress").data(prog.to_string()));
                }
                if remaining == 0 { metrics::counter!("budget_hits_total").increment(1); break; }
            }
            if loop_enabled && loop_win > 0 {
                if let Some(ratio) = loop_detector::check_sequence(
                    &mut win,
                    w,
                    loop_win,
                    loop_thresh,
                    entropy_min,
                ) {
                    hearing::warn(&format!(
                        "loop detected in SSE stream; terminating early; chat_id={} session_id={} window={} ratio={}",
                        req.chat_id,
                        req.session_id.clone().unwrap_or_default(),
                        loop_win,
                        ratio,
                    ));
                    break;
                }
            }
            if sent % 10 == 0 {
                let elapsed = start.elapsed().as_secs_f64().max(0.001);
                let tps = (sent as f64) / elapsed;
                let prog = serde_json::json!({"tokens": sent, "tokens_per_sec": tps, "partial_len": chars});
                yield Ok(Event::default().event("progress").data(prog.to_string()));
            }
        }
        let elapsed = start.elapsed().as_secs_f64().max(0.001);
        let tps = (sent as f64) / elapsed;
        let prog = serde_json::json!({"tokens": sent, "tokens_per_sec": tps, "partial_len": chars});
        yield Ok(Event::default().event("progress").data(prog.to_string()));
        yield Ok(Event::default().event("done").data("true"));
        metrics::gauge!("sse_active").decrement(1.0);
        if (elapsed * 1000.0) as u64 > warn_after_ms {
            hearing::warn(&format!(
                "sse stream slow; duration_ms={} chat_id={} session_id={}",
                (elapsed * 1000.0) as u64,
                req.chat_id,
                req.session_id.clone().unwrap_or_default()
            ));
        }
        if let Some(rid) = req_id2.as_deref() {
            hub_for_trace.trace_event(Some(rid), "chat.stream.done", serde_json::json!({"chat_id": chat_id2}));
        }
    };
    Ok(Sse::new(stream))
}

#[derive(serde::Deserialize)]
struct CancelStream {
    auth: String,
    chat_id: String,
    session_id: String,
}

async fn cancel_stream(
    State(state): State<AppState>,
    Json(req): Json<CancelStream>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    if !state.hub.check_auth(&req.auth) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    if !state
        .hub
        .check_scope(&req.auth, backend::interaction_hub::Scope::Write)
    {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    let ok = state.hub.cancel_stream(&req.chat_id, &req.session_id);
    if ok {
        metrics::counter!("sse_cancellations_total").increment(1);
    }
    Ok(Json(serde_json::json!({"cancelled": ok})))
}

#[derive(serde::Deserialize, Clone)]
struct SearchQuery {
    q: String,
    #[serde(default)]
    regex: bool,
    #[serde(default)]
    prefix: bool,
    since_id: Option<u64>,
    after_ts: Option<i64>,
    offset: Option<usize>,
    limit: Option<usize>,
    role: Option<String>,
    sort: Option<String>,
}

async fn search_chat(
    Path((chat_id, session_id)): Path<(String, String)>,
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<(axum::http::HeaderMap, String), (axum::http::StatusCode, String)> {
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let dir = std::path::Path::new(&base).join(&chat_id);
    let mut out = String::new();
    let mut matches: Vec<(i64, String)> = Vec::new();
    let q = params.q.clone();
    let regex = if params.regex {
        regex::Regex::new(&q).map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?
    } else {
        regex::Regex::new(&regex::escape(&q)).unwrap()
    };
    if let Ok(rd) = std::fs::read_dir(&dir) {
        let mut files: Vec<std::path::PathBuf> = rd.flatten().map(|e| e.path()).collect();
        files.sort();
        for p in files {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name == format!("{}.ndjson", session_id)
                || (name.starts_with(&format!("{}-", session_id))
                    && (name.ends_with(".ndjson") || name.ends_with(".ndjson.gz")))
            {
                let mut content = String::new();
                if name.ends_with(".gz") {
                    use std::io::Read;
                    let data = std::fs::read(&p).map_err(|e| {
                        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    })?;
                    let mut d = flate2::read::GzDecoder::new(&data[..]);
                    d.read_to_string(&mut content).map_err(|e| {
                        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    })?;
                } else {
                    content = std::fs::read_to_string(&p).map_err(|e| {
                        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    })?;
                }
                for line in content.lines() {
                    let lt = line.trim();
                    if lt.is_empty() {
                        continue;
                    }
                    // optional id/time filters
                    let mut ok = true;
                    if params.since_id.is_some() || params.after_ts.is_some() {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(lt) {
                            if let Some(min_id) = params.since_id {
                                if v.get("message_id")
                                    .and_then(|x| x.as_u64())
                                    .map(|id| id <= min_id)
                                    .unwrap_or(false)
                                {
                                    ok = false;
                                }
                            }
                            if let Some(min_ts) = params.after_ts {
                                if v.get("timestamp_ms")
                                    .and_then(|x| x.as_i64())
                                    .map(|t| t <= min_ts)
                                    .unwrap_or(false)
                                {
                                    ok = false;
                                }
                            }
                        }
                    }
                    if !ok {
                        continue;
                    }
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(lt) {
                        // role filter
                        if let Some(ref want) = params.role {
                            let vr = v.get("role").and_then(|x| x.as_str()).unwrap_or("");
                            let ok_role = match want.as_str() {
                                "user" => vr.eq_ignore_ascii_case("user"),
                                "assistant" => vr.eq_ignore_ascii_case("assistant"),
                                _ => true,
                            };
                            if !ok_role {
                                continue;
                            }
                        }
                        let text_ok = v
                            .get("content")
                            .and_then(|x| x.as_str())
                            .map(|text| {
                                (params.prefix && text.starts_with(&q))
                                    || (!params.prefix && regex.is_match(text))
                            })
                            .unwrap_or(false);
                        if text_ok {
                            let ts = v.get("timestamp_ms").and_then(|x| x.as_i64()).unwrap_or(0);
                            matches.push((ts, lt.to_string()));
                        }
                    }
                }
            }
        }
    }
    // sort by timestamp
    let asc = !matches!(params.sort.as_deref(), Some("desc"));
    matches.sort_by_key(|(ts, _)| *ts);
    if !asc {
        matches.reverse();
    }
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(usize::MAX);
    for (i, (_ts, line)) in matches.into_iter().enumerate() {
        if i < offset {
            continue;
        }
        if i >= offset + limit {
            break;
        }
        out.push_str(&line);
        out.push('\n');
    }
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/x-ndjson".parse().unwrap(),
    );
    Ok((headers, out))
}

#[derive(serde::Deserialize)]
struct ExportQuery {
    from: Option<String>,
    to: Option<String>,
}

async fn export_chat(
    Path(chat_id): Path<String>,
    axum::extract::Query(q): axum::extract::Query<ExportQuery>,
) -> impl axum::response::IntoResponse {
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let dir = std::path::Path::new(&base).join(&chat_id);
    let mut body = String::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        let mut files: Vec<std::path::PathBuf> = rd.flatten().map(|e| e.path()).collect();
        files.sort();
        for p in files {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            // filter by date window if provided
            if let (Some(ref from), Some(ref to)) = (&q.from, &q.to) {
                let parts: Vec<&str> = name
                    .trim_end_matches(".gz")
                    .trim_end_matches(".ndjson")
                    .split('-')
                    .collect();
                if parts.len() >= 2 {
                    let date = parts[parts.len() - 1];
                    if date < from.as_str() || date > to.as_str() {
                        continue;
                    }
                }
            }
            if name.ends_with(".gz") {
                use std::io::Read;
                let data = std::fs::read(&p).unwrap_or_default();
                let mut d = flate2::read::GzDecoder::new(&data[..]);
                let mut s = String::new();
                let _ = d.read_to_string(&mut s);
                body.push_str(&s);
            } else if name.ends_with(".ndjson") {
                if let Ok(s) = std::fs::read_to_string(&p) {
                    body.push_str(&s);
                }
            }
        }
    }
    (
        [(axum::http::header::CONTENT_TYPE, "application/x-ndjson")],
        body,
    )
}

async fn import_chat(
    State(state): State<AppState>,
    Path((chat_id, session_id)): Path<(String, String)>,
    headers: HeaderMap,
    axum::extract::Query(mut q): axum::extract::Query<AuthQuery>,
    body: String,
) -> Result<(), (axum::http::StatusCode, String)> {
    if state.paused.load(std::sync::atomic::Ordering::Relaxed) {
        return Err((axum::http::StatusCode::SERVICE_UNAVAILABLE, "paused".into()));
    }
    if q.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            q.auth = h;
        }
    }
    if !state.hub.check_auth(&q.auth) {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
    }
    if !state
        .hub
        .check_scope(&q.auth, backend::interaction_hub::Scope::Write)
    {
        return Err((axum::http::StatusCode::FORBIDDEN, "forbidden".into()));
    }
    let mut msgs = Vec::new();
    for line in body.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<backend::context::context_storage::ChatMessage>(line) {
            Ok(m) => msgs.push(m),
            Err(e) => {
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("invalid ndjson: {e}"),
                ))
            }
        }
    }
    state
        .storage
        .import_messages(&chat_id, &session_id, msgs)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct MaskingUpdate {
    auth: String,
    enabled: Option<bool>,
    regex: Option<Vec<String>>,
    roles: Option<Vec<String>>,
    preset: Option<String>,
}

async fn update_masking(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut req): Json<MaskingUpdate>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    if !state.hub.check_auth(&req.auth) {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
    }
    if !state
        .hub
        .check_scope(&req.auth, backend::interaction_hub::Scope::Admin)
    {
        return Err((axum::http::StatusCode::FORBIDDEN, "forbidden".into()));
    }
    let mut regexes = req.regex;
    if regexes.is_none() {
        if let Some(name) = req.preset.as_deref() {
            let list = backend::context::context_storage::load_mask_preset(name)
                .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
            regexes = Some(list);
        }
    }
    set_runtime_mask_config(req.enabled, regexes, req.roles)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    Ok(Json(serde_json::json!({"status":"ok"})))
}

#[derive(serde::Serialize)]
struct MaskingConfigView {
    enabled: bool,
    regex: Vec<String>,
    roles: Vec<String>,
}

async fn masking_config_view() -> Result<Json<MaskingConfigView>, (axum::http::StatusCode, String)>
{
    let cfg = backend::context::context_storage::get_runtime_mask_config();
    Ok(Json(MaskingConfigView {
        enabled: cfg.enabled,
        regex: cfg.regex,
        roles: cfg.roles,
    }))
}

#[derive(serde::Deserialize)]
struct MaskingDryRun {
    text: String,
    regex: Option<Vec<String>>,
    roles: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
struct MaskingDryRunResult {
    masked: String,
}

async fn masking_dry_run(
    Json(req): Json<MaskingDryRun>,
) -> Result<Json<MaskingDryRunResult>, (axum::http::StatusCode, String)> {
    let masked = backend::context::context_storage::mask_preview(
        &req.text,
        req.regex.clone(),
        req.roles.clone(),
    )
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    Ok(Json(MaskingDryRunResult { masked }))
}

async fn toggle_probe(
    State(state): State<AppState>,
    Path(name): Path<String>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let token = auth_from_headers(&headers).unwrap_or_default();
    if !state.hub.check_auth(&token) {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
    }
    let enabled = state
        .hub
        .toggle_probe(&name)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    Ok(Json(serde_json::json!({ "enabled": enabled })))
}

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let cfg = Config::from_env();
    let logs_dir = "logs";
    let _ = std::fs::create_dir_all(logs_dir);

    let file_appender = tracing_appender::rolling::daily(logs_dir, "backend.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let json_logs = std::env::var("NERVOUS_SYSTEM_JSON_LOGS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let fmt_builder = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());
    if json_logs {
        fmt_builder.json().init();
    } else {
        fmt_builder.init();
    }

    let templates_dir =
        std::env::var("NODE_TEMPLATES_DIR").unwrap_or_else(|_| "./templates".into());
    let _ = std::fs::create_dir_all(&templates_dir);
    let registry = Arc::new(NodeRegistry::new(&templates_dir).expect("registry"));
    let memory = Arc::new(MemoryNode::new());
    registry.register_init_node(Arc::new(InitConfigNode::new()), &memory);
    let (metrics, metrics_rx) = MetricsCollectorNode::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsNode::new(metrics_rx, 5, metrics.clone());
    let hub = Arc::new(InteractionHub::new(
        registry.clone(),
        memory.clone(),
        metrics,
        diagnostics,
        &cfg,
    ));
    let backpressure = Arc::new(BackpressureProbe::new(hub.clone()));
    // Expose hub globally for lightweight activity signals (optional)
    backend::GLOBAL_HUB.get_or_init(|| std::sync::RwLock::new(None));
    if let Some(lock) = backend::GLOBAL_HUB.get() {
        let _ = lock.write().map(|mut w| {
            *w = Some(hub.clone());
        });
    }
    hub.add_auth_token("secret");
    hub.add_trigger_keyword("echo");
    registry.register_action_node(Arc::new(PreloadAction::default()));
    registry.register_scripted_training_node();
    // Register a default chat node
    registry.register_chat_node(Arc::new(EchoChatNode::default()));

    // Context storage
    let storage = Arc::new(FileContextStorage::new("context"));
    // Initialize sessions_active gauge by reading index.json files
    {
        let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
        let mut total = 0u64;
        if let Ok(rd) = std::fs::read_dir(&base) {
            for e in rd.flatten() {
                let p = e.path().join("index.json");
                if p.exists() {
                    if let Ok(s) = std::fs::read_to_string(&p) {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                            if let Some(obj) = v.as_object() {
                                total += obj.len() as u64;
                            }
                        }
                    }
                }
            }
        }
        metrics::gauge!("sessions_active").set(total as f64);
    }
    // Пример узла анализа
    struct EchoNode;
    impl AnalysisNode for EchoNode {
        fn id(&self) -> &str {
            "example.analysis"
        }
        fn analysis_type(&self) -> &str {
            "summary"
        }
        fn status(&self) -> NodeStatus {
            NodeStatus::Active
        }
        fn links(&self) -> &[String] {
            &[]
        }
        fn confidence_threshold(&self) -> f32 {
            0.0
        }
        fn analyze(
            &self,
            input: &str,
            cancel_token: &tokio_util::sync::CancellationToken,
        ) -> AnalysisResult {
            if cancel_token.is_cancelled() {
                let mut r = AnalysisResult::new(self.id(), input, vec![]);
                r.status = NodeStatus::Error;
                return r;
            }
            AnalysisResult::new(self.id(), input, vec!["echo".into()])
        }
        fn explain(&self) -> String {
            "Echoes input".into()
        }
    }

    registry.register_analysis_node(Arc::new(EchoNode));
    let metrics_handle = if cfg.nervous_system.enabled {
        Some(
            PrometheusBuilder::new()
                .install_recorder()
                .expect("metrics"),
        )
    } else {
        None
    };

    let shutdown_token = tokio_util::sync::CancellationToken::new();

    let state = AppState {
        hub: hub.clone(),
        backpressure: backpressure.clone(),
        storage: storage.clone(),
        paused: Arc::new(AtomicBool::new(false)),
        pause_info: Arc::new(Mutex::new(None)),
        shutdown: shutdown_token.clone(),
    };

    anti_idle::init();

    // Register auth tokens from environment for development/admin access
    if let Ok(admin) = std::env::var("NEIRA_ADMIN_TOKEN") {
        hub.add_token_with_scopes(admin, &[backend::interaction_hub::Scope::Admin]);
    }
    if let Ok(write) = std::env::var("NEIRA_WRITE_TOKEN") {
        hub.add_token_with_scopes(
            write,
            &[
                backend::interaction_hub::Scope::Write,
                backend::interaction_hub::Scope::Read,
            ],
        );
    }
    if let Ok(read) = std::env::var("NEIRA_READ_TOKEN") {
        hub.add_token_with_scopes(read, &[backend::interaction_hub::Scope::Read]);
    }

    // Anti-Idle core (dry-run): update idle_state and idle_minutes_today
    if anti_idle::is_enabled() {
        let hub_for_idle = hub.clone();
        tokio::spawn(async move {
            use std::time::Duration;
            let t = *anti_idle::thresholds();
            let idle_secs = t.idle_secs;
            let long_secs = t.long_secs;
            let deep_secs = t.deep_secs;
            let alpha = anti_idle::ema_alpha();
            let dry_depth_env = anti_idle::dryrun_queue_depth();
            let dryrun_enabled = std::env::var("LEARNING_MICROTASKS_DRYRUN")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            let mut accum_idle_secs: u64 = 0;
            let mut idle_ema: f64 = 0.0;
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
                if !anti_idle::is_enabled() {
                    metrics::gauge!("idle_state").set(0.0);
                    metrics::gauge!("microtask_queue_depth").set(0.0);
                    metrics::gauge!("time_since_activity_seconds").set(0.0);
                    metrics::counter!("autonomous_time_spent_seconds").increment(0);
                    continue;
                }
                let (state_idx, since) = anti_idle::idle_state(hub_for_idle.active_streams());
                metrics::gauge!("idle_state").set(state_idx as f64);
                let dry_depth = if dryrun_enabled && state_idx > 0 {
                    dry_depth_env
                } else {
                    0
                };
                metrics::gauge!("microtask_queue_depth").set(dry_depth as f64);
                metrics::gauge!("time_since_activity_seconds").set(since as f64);
                metrics::counter!("autonomous_time_spent_seconds").increment(0);
                idle_ema = if idle_ema == 0.0 {
                    state_idx as f64
                } else {
                    alpha * (state_idx as f64) + (1.0 - alpha) * idle_ema
                };
                metrics::gauge!("idle_state_smoothed").set(idle_ema);
                if state_idx > 0 {
                    accum_idle_secs += 5;
                    if accum_idle_secs >= 60 {
                        let mins = accum_idle_secs / 60;
                        accum_idle_secs %= 60;
                        metrics::counter!("idle_minutes_today").increment(mins as u64);
                    }
                } else {
                    accum_idle_secs = 0;
                }
            }
        });
    }

    let mut app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/admin",
            get(|| async {
                match std::fs::read_to_string("backend/static/admin.html") {
                    Ok(s) => (
                        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
                        s,
                    ),
                    Err(_) => (
                        [(axum::http::header::CONTENT_TYPE, "text/plain")],
                        String::from("admin.html not found"),
                    ),
                }
            }),
        )
        .route(
            "/training",
            get(|| async {
                match std::fs::read_to_string("backend/static/training.html") {
                    Ok(s) => (
                        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
                        s,
                    ),
                    Err(_) => (
                        [(axum::http::header::CONTENT_TYPE, "text/plain")],
                        String::from("training.html not found"),
                    ),
                }
            }),
        )
        /* neira:meta
        id: NEI-20250607-axum-route-syntax
        intent: refactor
        summary: обновлён синтаксис параметров маршрутов для axum >=0.7.
        */
        .route("/nodes", post(register_node))
        .route("/nodes/{id}", get(get_node_latest))
        .route("/nodes/{id}/{version}", get(get_node))
        // Factory API (draft)
        .route("/factory/nodes/dryrun", post(factory_dryrun))
        .route("/factory/nodes", post(factory_create))
        .route("/factory/nodes/{fid}/approve", post(factory_approve))
        .route("/factory/nodes/{fid}/disable", post(factory_disable))
        .route("/factory/nodes/{fid}/rollback", post(factory_rollback))
        // Organ builder
        .route("/organs", get(organs_list))
        .route("/organs/build", post(organ_build))
        .route("/organs/{id}/build", delete(organ_cancel_build))
        .route("/organs/{id}/rebuild", post(organ_rebuild))
        .route(
            "/organs/{id}/status",
            get(organ_status).post(organ_update_status),
        )
        .route("/organs/{id}/stream", get(organ_stream))
        .route("/api/neira/analysis", post(analyze_request))
        .route("/api/neira/analysis/resume", post(resume_request))
        .route("/api/neira/chat", post(chat_request))
        .route("/api/neira/chat/stream", post(chat_stream))
        .route("/api/neira/chat/session/new", post(new_session))
        .route(
            "/api/neira/chat/{chat_id}/{session_id}",
            delete(delete_session),
        )
        .route(
            "/api/neira/chat/{chat_id}/{session_id}/rename",
            post(rename_session),
        )
        .route(
            "/api/neira/training/run",
            post(http::training_routes::training_run),
        )
        .route(
            "/api/neira/training/status",
            get(http::training_routes::training_status),
        )
        .route(
            "/api/neira/training/stream",
            get(http::training_routes::training_stream),
        )
        .route("/api/neira/context/masking", post(update_masking))
        .route(
            "/api/neira/context/masking/config",
            get(masking_config_view),
        )
        .route("/api/neira/context/masking/dry_run", post(masking_dry_run))
        .route("/api/neira/probes/{name}/toggle", post(toggle_probe))
        .route(
            "/context/{*path}",
            get(|Path(path): Path<String>| async move {
                let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
                let full = std::path::Path::new(&base).join(path);
                match std::fs::read(&full) {
                    Ok(bytes) => {
                        let ct = if full.extension().and_then(|s| s.to_str()) == Some("html") {
                            "text/html; charset=utf-8"
                        } else {
                            "application/octet-stream"
                        };
                        ([(axum::http::header::CONTENT_TYPE, ct)], bytes)
                    }
                    Err(_) => (
                        [(axum::http::header::CONTENT_TYPE, "text/plain")],
                        b"not found".to_vec(),
                    ),
                }
            }),
        )
        .route("/api/neira/chat/{chat_id}/export", get(export_chat))
        .route(
            "/api/neira/chat/{chat_id}/import/{session_id}",
            post(import_chat),
        )
        .route("/api/neira/chat/{chat_id}/index", get(get_chat_index))
        .route(
            "/api/neira/chat/{chat_id}/{session_id}",
            get(get_chat_session),
        )
        .route(
            "/api/neira/chat/{chat_id}/{session_id}/search",
            get(search_chat),
        )
        .route("/api/neira/chat/stream/cancel", post(cancel_stream))
        .layer(CorsLayer::permissive());
    // Control Plane (admin)

    async fn control_pause(
        State(state): State<AppState>,
        Json(mut body): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let auth = body
            .get_mut("auth")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        if !state.hub.check_auth(&auth) {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
        if !state
            .hub
            .check_scope(&auth, backend::interaction_hub::Scope::Admin)
        {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let allow = std::env::var("CONTROL_ALLOW_PAUSE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        if !allow {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let reason = body
            .get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let request_id = body
            .get("request_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        state
            .paused
            .store(true, std::sync::atomic::Ordering::Relaxed);
        metrics::gauge!("paused_state").set(1.0);
        metrics::counter!("pause_events_total").increment(1);
        // причина как лейбл (осторожно с кардинальностью)
        metrics::counter!("pause_reason_total", "reason" => reason.clone()).increment(1);
        {
            let mut g = state.pause_info.lock().unwrap();
            *g = Some((std::time::Instant::now(), reason.clone()));
        }
        if body
            .get("drain_active_streams")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let n = state.hub.cancel_all_streams();
            if n > 0 {
                metrics::counter!("sse_cancellations_total").increment(n as u64);
            }
            metrics::counter!("pause_drain_events_total").increment(1);
            hearing::info(&format!(
                "control: pause with drain; auth={} reason={} cancelled_streams={}",
                auth, reason, n
            ));
        }
        hearing::info(&format!(
            "control: pause; request_id={} auth={} reason={}",
            request_id, auth, reason
        ));
        let now_ms = chrono::Utc::now().timestamp_millis();
        Ok(Json(
            serde_json::json!({"paused": true, "reason": reason, "paused_since_ts_ms": now_ms}),
        ))
    }
    async fn control_resume(
        State(state): State<AppState>,
        Json(mut body): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let auth = body
            .get_mut("auth")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        if !state.hub.check_auth(&auth) {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
        if !state
            .hub
            .check_scope(&auth, backend::interaction_hub::Scope::Admin)
        {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let allow = std::env::var("CONTROL_ALLOW_PAUSE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        if !allow {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let request_id = body
            .get("request_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        state
            .paused
            .store(false, std::sync::atomic::Ordering::Relaxed);
        metrics::gauge!("paused_state").set(0.0);
        hearing::info(&format!(
            "control: resume; request_id={} auth={}",
            request_id, auth
        ));
        Ok(Json(serde_json::json!({"paused": false})))
    }
    async fn control_status(
        State(state): State<AppState>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let (paused, since_ms, reason) = {
            let p = state.paused.load(std::sync::atomic::Ordering::Relaxed);
            let mut since_ms: u128 = 0;
            let mut reason = String::new();
            if let Some((inst, r)) = state.pause_info.lock().unwrap().clone() {
                since_ms = inst.elapsed().as_millis();
                reason = r;
            }
            (p, since_ms, reason)
        };
        let active_tasks = state.hub.active_streams() as u64;
        let (qf, qs, ql) = state.backpressure.queue_lengths();
        let backpressure = (qf + qs + ql) as u64;
        let now_ms = chrono::Utc::now().timestamp_millis();
        Ok(Json(serde_json::json!({
            "paused": paused,
            "paused_for_ms": since_ms,
            "paused_since_ts_ms": (now_ms as i128 - since_ms as i128).max(0) as i64,
            "reason": reason,
            "active_tasks": active_tasks,
            "backpressure": backpressure,
            "queues": {"fast": qf, "standard": qs, "long": ql}
        })))
    }
    async fn control_kill(
        State(state): State<AppState>,
        Json(mut body): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let auth = body
            .get_mut("auth")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        if !state.hub.check_auth(&auth) {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
        if !state
            .hub
            .check_scope(&auth, backend::interaction_hub::Scope::Admin)
        {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let allow = std::env::var("CONTROL_ALLOW_KILL")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        if !allow {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let grace_ms = body
            .get("grace_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(1_000);
        let request_id = body
            .get("request_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        hearing::warn(&format!(
            "control: kill (graceful); request_id={} auth={} grace_ms={}",
            request_id, auth, grace_ms
        ));
        metrics::counter!("kill_switch_total").increment(1);
        // Инициируем graceful shutdown сервера
        state.shutdown.cancel();
        // Форсируем выход по таймауту как safeguard
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(grace_ms)).await;
            std::process::exit(0);
        });
        Ok(Json(
            serde_json::json!({"stopping": true, "grace_ms": grace_ms}),
        ))
    }
    async fn inspect_snapshot(
        State(state): State<AppState>,
        axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let dir = std::env::var("CONTROL_SNAPSHOT_DIR").unwrap_or_else(|_| "./snapshots".into());
        let ts = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let path = std::path::Path::new(&dir).join(format!("snapshot-{}.json", ts));
        let mut obj = serde_json::json!({
            "created_at": chrono::Utc::now().to_rfc3339(),
        });
        if q.get("include")
            .map(|s| s.contains("metrics"))
            .unwrap_or(false)
        {
            if let Ok(resp) = reqwest::get("http://127.0.0.1:3000/metrics").await {
                if let Ok(text) = resp.text().await {
                    obj["metrics_prom"] = serde_json::json!(text);
                }
            }
        }
        if q.get("include")
            .map(|s| s.contains("context"))
            .unwrap_or(false)
        {
            let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
            let mut index: std::collections::HashMap<String, Vec<String>> =
                std::collections::HashMap::new();
            if let Ok(rd) = std::fs::read_dir(&base) {
                for e in rd.flatten() {
                    if e.path().is_dir() {
                        let chat = e.file_name().to_string_lossy().to_string();
                        let mut files = Vec::new();
                        if let Ok(r2) = std::fs::read_dir(e.path()) {
                            for f in r2.flatten() {
                                if let Some(name) = f.file_name().to_str() {
                                    files.push(name.to_string());
                                }
                            }
                        }
                        files.sort();
                        index.insert(chat, files);
                    }
                }
            }
            obj["context_index"] = serde_json::to_value(index).unwrap_or(serde_json::json!({}));
        }
        let mut logs_file_out: Option<std::path::PathBuf> = None;
        if q.get("include")
            .map(|s| s.contains("logs"))
            .unwrap_or(false)
        {
            let log_path = std::path::Path::new("logs").join("backend.log");
            let lines = std::env::var("LOGS_TAIL_LINES")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(200);
            if let Ok(data) = std::fs::read_to_string(&log_path) {
                let level_filter = q.get("level").cloned();
                let since_ms = q.get("since_ts_ms").and_then(|v| v.parse::<i64>().ok());
                let mut tail: Vec<&str> = Vec::new();
                for ln in data.lines().rev() {
                    if tail.len() >= lines {
                        break;
                    }
                    if let Some(ref lev) = level_filter {
                        if !ln.contains(lev) {
                            continue;
                        }
                    }
                    if let Some(since) = since_ms {
                        // попытка фильтра по времени для JSON-логов с полем timestamp (RFC3339)
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(ln) {
                            if let Some(ts) = v.get("timestamp").and_then(|x| x.as_str()) {
                                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
                                    if dt.timestamp_millis() < since {
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    tail.push(ln);
                }
                tail.reverse();
                let joined = tail.join("\n");
                let masked = backend::context::context_storage::mask_preview(&joined, None, None)
                    .unwrap_or(joined);
                let lf = std::path::Path::new(&dir).join(format!("snapshot-{}-logs-tail.log", ts));
                let _ = std::fs::write(&lf, masked.as_bytes());
                logs_file_out = Some(lf.clone());
                obj["logs_tail_file"] = serde_json::json!(lf.to_string_lossy());
            } else {
                obj["logs_tail_file"] = serde_json::json!("not available");
            }
        }
        if let Some(req_id) = q.get("request_id") {
            if let Some(v) = state.hub.trace_dump(req_id) {
                let tf = std::path::Path::new(&dir)
                    .join(format!("snapshot-{}-trace-{}.json", ts, req_id));
                let _ =
                    std::fs::write(&tf, serde_json::to_string_pretty(&v).unwrap_or("{}".into()));
                obj["trace_file"] = serde_json::json!(tf.to_string_lossy());
            }
        }
        hearing::info("control: snapshot created");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(
            &path,
            serde_json::to_string_pretty(&obj).unwrap_or("{}".into()),
        );
        let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        obj["file"] = serde_json::json!(path.to_string_lossy());
        obj["public_url"] = serde_json::json!(format!("/snapshots/{}", fname));
        if q.get("zip").map(|v| v == "1").unwrap_or(false) {
            let zip_path = std::path::Path::new(&dir).join(format!("snapshot-{}.zip", ts));
            let f = std::fs::File::create(&zip_path)
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut zip = zip::ZipWriter::new(f);
            use zip::write::FileOptions;
            let opts = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
            // add JSON
            let json_str = std::fs::read_to_string(&path).unwrap_or("{}".into());
            let _ = zip.start_file("snapshot.json", opts.clone());
            let _ = zip.write_all(json_str.as_bytes());
            // add logs tail
            if let Some(lf) = logs_file_out.as_ref() {
                if let Ok(data) = std::fs::read(&lf) {
                    let _ = zip.start_file("logs-tail.log", opts.clone());
                    let _ = zip.write_all(&data);
                }
            }
            // add trace
            if let Some(tfv) = obj.get("trace_file").and_then(|v| v.as_str()) {
                if let Ok(data) = std::fs::read(tfv) {
                    let _ = zip.start_file("trace.json", opts.clone());
                    let _ = zip.write_all(&data);
                }
            }
            let _ = zip.finish();
            let zname = zip_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            obj["zip_file"] = serde_json::json!(zip_path.to_string_lossy());
            obj["zip_url"] = serde_json::json!(format!("/snapshots/{}", zname));
            let _ = std::fs::write(
                &path,
                serde_json::to_string_pretty(&obj).unwrap_or("{}".into()),
            );
        }
        metrics::counter!("snapshots_created_total").increment(1);
        Ok(Json(obj))
    }
    async fn trace_request(
        State(state): State<AppState>,
        Path(request_id): Path<String>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        if let Some(v) = state.hub.trace_dump(&request_id) {
            Ok(Json(v))
        } else {
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }
    app = app
        .route("/api/neira/control/pause", post(control_pause))
        .route("/api/neira/control/resume", post(control_resume))
        .route("/api/neira/control/status", get(control_status))
        .route("/api/neira/control/kill", post(control_kill))
        .route("/api/neira/inspect/snapshot", get(inspect_snapshot))
        .route("/api/neira/trace/{request_id}", get(trace_request));

    async fn queues_status(
        State(state): State<AppState>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let (qf, qs, ql) = state.backpressure.queue_lengths();
        let active = state.hub.active_streams();
        let bp = state.backpressure.backpressure_sum();
        Ok(Json(serde_json::json!({
            "active_streams": active,
            "backpressure": bp,
            "queues": {"fast": qf, "standard": qs, "long": ql}
        })))
    }
    app = app.route("/api/neira/queues/status", get(queues_status));

    // Cancel analysis endpoint
    #[derive(serde::Deserialize)]
    struct CancelAnalysis {
        auth: String,
        id: String,
    }
    async fn analysis_cancel(
        State(state): State<AppState>,
        Json(req): Json<CancelAnalysis>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        if !state.hub.check_auth(&req.auth) {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
        if !state
            .hub
            .check_scope(&req.auth, backend::interaction_hub::Scope::Write)
        {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let ok = state.hub.cancel_analysis(&req.id);
        Ok(Json(serde_json::json!({"cancelled": ok})))
    }
    app = app.route("/api/neira/analysis/cancel", post(analysis_cancel));

    // Logs tail endpoint with filters: /api/neira/logs/tail?lines=&level=&since_ts_ms=
    async fn logs_tail(
        axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        let log_path = std::path::Path::new("logs").join("backend.log");
        let lines = q
            .get("lines")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(200);
        let level = q.get("level").cloned();
        let since_ms = q.get("since_ts_ms").and_then(|v| v.parse::<i64>().ok());
        if let Ok(data) = std::fs::read_to_string(&log_path) {
            let mut out: Vec<String> = Vec::new();
            for ln in data.lines().rev() {
                if out.len() >= lines {
                    break;
                }
                if let Some(ref lev) = level {
                    if !ln.contains(lev) {
                        continue;
                    }
                }
                if let Some(since) = since_ms {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(ln) {
                        if let Some(ts) = v.get("timestamp").and_then(|x| x.as_str()) {
                            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
                                if dt.timestamp_millis() < since {
                                    continue;
                                }
                            }
                        }
                    }
                }
                out.push(ln.to_string());
            }
            out.reverse();
            return Ok(Json(
                serde_json::json!({"file": log_path.to_string_lossy(), "lines": out }),
            ));
        }
        Err(axum::http::StatusCode::NOT_FOUND)
    }
    app = app.route("/api/neira/logs/tail", get(logs_tail));

    // Limits recommendations based on current metrics
    async fn limits_recommendations() -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        // discover port from bind addr
        let bind = std::env::var("NEIRA_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
        let metrics_url = format!("http://{}/metrics", bind);
        let txt = match reqwest::get(&metrics_url).await {
            Ok(r) => r.text().await.unwrap_or_default(),
            Err(_) => String::new(),
        };
        let re_gauge = |name: &str, labels: Option<&str>| -> f64 {
            let pat = if let Some(lbl) = labels {
                format!(
                    r"(?m)^{}\{{[^}}]*{}[^}}]*\}}\s+([0-9]+(?:\.[0-9]+)?)$",
                    regex::escape(name),
                    lbl
                )
            } else {
                format!(r"(?m)^{}\s+([0-9]+(?:\.[0-9]+)?)$", regex::escape(name))
            };
            let rg = Regex::new(&pat).unwrap();
            if let Some(c) = rg.captures(&txt) {
                c.get(1)
                    .and_then(|m| m.as_str().parse::<f64>().ok())
                    .unwrap_or(0.0)
            } else {
                0.0
            }
        };
        let (soft_to, hard_to) = Watchdog::parse_metrics(&txt);
        let drains = re_gauge("pause_drain_events_total", None);
        let throttles = re_gauge("throttle_events_total", None);
        let sse_active = re_gauge("sse_active", None);

        // queues/backpressure via JSON
        let queues_url = format!("http://{}/api/neira/queues/status", bind);
        let (backpressure, qf, qs, ql) = match reqwest::get(&queues_url).await {
            Ok(r) => match r.json::<serde_json::Value>().await {
                Ok(v) => (
                    v.get("backpressure").and_then(|x| x.as_u64()).unwrap_or(0),
                    v.get("queues")
                        .and_then(|x| x.get("fast"))
                        .and_then(|x| x.as_u64())
                        .unwrap_or(0),
                    v.get("queues")
                        .and_then(|x| x.get("standard"))
                        .and_then(|x| x.as_u64())
                        .unwrap_or(0),
                    v.get("queues")
                        .and_then(|x| x.get("long"))
                        .and_then(|x| x.as_u64())
                        .unwrap_or(0),
                ),
                Err(_) => (0, 0, 0, 0),
            },
            Err(_) => (0, 0, 0, 0),
        };

        let mut recs: Vec<serde_json::Value> = Vec::new();
        if hard_to > 0.0 {
            recs.push(serde_json::json!({"key": "WATCHDOG_REASONING_HARD_MS", "action": "increase", "why": "Есть hard‑таймауты рассуждений"}));
        }
        if soft_to > 10.0 && hard_to == 0.0 {
            recs.push(serde_json::json!({"key": "AUTO_REQUEUE_ON_SOFT", "action": "enable", "why": "Много soft‑таймаутов — перевести длинные задачи в Long"}));
        }
        if throttles > 50.0 && backpressure > 50 {
            recs.push(serde_json::json!({"key": "BACKPRESSURE_HIGH_WATERMARK", "action": "increase", "why": "Частые троттлинги и высокое давление очередей"}));
        }
        if drains > 0.0 && sse_active > 0.0 {
            recs.push(serde_json::json!({"key": "REASONING_TOKEN_BUDGET", "action": "increase", "why": "При паузах часто выполняется дренаж SSE — ограничить длину потоков мягким бюджетом"}));
        }
        let signals = serde_json::json!({
            "watchdog_timeouts": {"soft": soft_to, "hard": hard_to},
            "pause_drain_events": drains,
            "throttle_events": throttles,
            "sse_active": sse_active,
            "queues": {"fast": qf, "standard": qs, "long": ql, "backpressure": backpressure}
        });
        Ok(Json(
            serde_json::json!({"signals": signals, "recommendations": recs}),
        ))
    }
    app = app.route(
        "/api/neira/limits/recommendations",
        get(limits_recommendations),
    );

    // Analysis streaming progress (SSE) — прогноз прогресса по времени/бюджету
    async fn analysis_stream(
        State(state): State<AppState>,
        headers: HeaderMap,
        Json(mut req): Json<AnalysisRequest>,
    ) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode> {
        // Anti-Idle: mark user activity
        anti_idle::mark_activity();
        if req.auth.trim().is_empty() {
            if let Some(h) = auth_from_headers(&headers) {
                req.auth = h;
            }
        }
        let token = tokio_util::sync::CancellationToken::new();
        // per-request budget via header fallback
        if req.budget_ms.is_none() {
            if let Some(h) = headers
                .get("x-reasoning-budget-ms")
                .and_then(|v| v.to_str().ok())
            {
                req.budget_ms = h.parse::<u64>().ok();
            }
        }
        let start = std::time::Instant::now();
        let auth = req.auth.clone();
        let id = req.id.clone();
        let input = req.input.clone();
        let hub = state.hub.clone();
        let t2 = token.clone();
        let mut handle =
            tokio::task::spawn(async move { hub.analyze(&id, &input, &auth, &t2).await });
        let hub_for_progress = state.hub.clone();
        state.hub.register_analysis_cancel(&req.id, token.clone());
        let stream = async_stream::stream! {
            yield Ok(Event::default().event("meta").data(serde_json::json!({"id": req.id, "budget_ms": req.budget_ms}).to_string()));
            loop {
                // progress every 1s
                tokio::select!{
                    _ = tokio::time::sleep(std::time::Duration::from_millis(1000)) => {
                        let elapsed = start.elapsed().as_millis() as u64;
                        let mut prog = serde_json::json!({"elapsed_ms": elapsed});
                        if let Some(b) = req.budget_ms { let pct = ((elapsed as f64)/(b.max(1) as f64)).min(1.0); prog["time_ratio"] = serde_json::json!(pct); }
                        anti_idle::mark_activity();
                        yield Ok(Event::default().event("progress").data(prog.to_string()));
                        if let Some(b) = req.budget_ms { if elapsed >= b { token.cancel(); } }
                    }
                    res = &mut handle => {
                        match res {
                            Ok(Some(result)) => {
                                yield Ok(Event::default().event("done").data(serde_json::json!({"status": result.status, "id": result.id}).to_string()));
                            }
                            Ok(None) => { yield Ok(Event::default().event("done").data(serde_json::json!({"status": "error"}).to_string())); }
                            Err(_) => { yield Ok(Event::default().event("done").data(serde_json::json!({"status": "panic"}).to_string())); }
                        }
                        state.hub.remove_analysis_cancel(&req.id);
                        break;
                    }
                }
            }
        };
        Ok(Sse::new(stream))
    }
    app = app.route("/api/neira/analysis/stream", post(analysis_stream));

    // Trace runtime toggle (admin)
    #[derive(serde::Deserialize)]
    struct TraceToggle {
        auth: String,
        enabled: Option<bool>,
    }
    async fn trace_toggle(
        State(state): State<AppState>,
        Json(req): Json<TraceToggle>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        if !state.hub.check_auth(&req.auth) {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
        if !state
            .hub
            .check_scope(&req.auth, backend::interaction_hub::Scope::Admin)
        {
            return Err(axum::http::StatusCode::FORBIDDEN);
        }
        let new_state = req.enabled.unwrap_or(!state.hub.is_trace_enabled());
        state.hub.set_trace_enabled(new_state);
        Ok(Json(serde_json::json!({"enabled": new_state})))
    }
    app = app.route("/api/neira/trace/toggle", post(trace_toggle));

    // Serve snapshots directory (download links)
    app = app.route(
        "/snapshots/{*path}",
        get(|Path(path): Path<String>| async move {
            let base =
                std::env::var("CONTROL_SNAPSHOT_DIR").unwrap_or_else(|_| "./snapshots".into());
            let full = std::path::Path::new(&base).join(path);
            match std::fs::read(&full) {
                Ok(bytes) => {
                    let ct = if full.extension().and_then(|s| s.to_str()) == Some("zip") {
                        "application/zip"
                    } else {
                        "application/json"
                    };
                    ([(axum::http::header::CONTENT_TYPE, ct)], bytes)
                }
                Err(_) => (
                    [(axum::http::header::CONTENT_TYPE, "text/plain")],
                    b"not found".to_vec(),
                ),
            }
        }),
    );

    // Introspection Status
    async fn introspection_status(
        State(state): State<AppState>,
    ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        metrics::counter!("introspection_status_requests_total").increment(1);
        let (qf, qs, ql) = state.backpressure.queue_lengths();
        let active = state.hub.active_streams();
        let bp = state.backpressure.backpressure_sum();
        let soft_ms: u64 = std::env::var("WATCHDOG_REASONING_SOFT_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30_000);
        let hard_ms: u64 = std::env::var("WATCHDOG_REASONING_HARD_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300_000);
        // Anti-Idle snapshot
        let anti_idle_enabled = anti_idle::is_enabled();
        let (idle_state, since) = anti_idle::idle_state(active);
        let t = *anti_idle::thresholds();
        metrics::gauge!("time_since_activity_seconds").set(since as f64);
        let caps = serde_json::json!({
            "trace_requests": state.hub.is_trace_enabled(),
            "inspect_snapshot": true,
            "control_pause_resume": std::env::var("CONTROL_ALLOW_PAUSE").map(|v| v=="1"||v.eq_ignore_ascii_case("true")).unwrap_or(true),
            "control_kill_switch": std::env::var("CONTROL_ALLOW_KILL").map(|v| v=="1"||v.eq_ignore_ascii_case("true")).unwrap_or(true),
            "dev_routes": std::env::var("DEV_ROUTES_ENABLED").map(|v| v=="1"||v.eq_ignore_ascii_case("true")).unwrap_or(false),
            "factory_adapter": state.hub.factory_is_adapter_enabled(),
            "organs_builder": state.hub.organ_builder_enabled()
        });
        let (factory_total, factory_active) = state.hub.factory_counts();
        Ok(Json(serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "paused": state.paused.load(std::sync::atomic::Ordering::Relaxed),
            "safe_mode": state.hub.is_safe_mode(),
            "capabilities": caps,
            "sse_active": active,
            "queues": {"fast": qf, "standard": qs, "long": ql},
            "backpressure": bp,
            "watchdogs": {"soft_ms": soft_ms, "hard_ms": hard_ms},
            "anti_idle": {"enabled": anti_idle_enabled, "idle_state": idle_state, "idle_label": match idle_state {0=>"active",1=>"short",2=>"long",_=>"deep"}, "since_seconds": since, "thresholds": {"idle": t.idle_secs, "long": t.long_secs, "deep": t.deep_secs}, "microtasks": {"dryrun_depth": anti_idle::dryrun_queue_depth() }}
            ,"factory": {"records_total": factory_total, "active": factory_active}
        })))
    }
    app = app.route("/api/neira/introspection/status", get(introspection_status));
    app = app.merge(anti_idle::router::<AppState>());

    // Runtime Extensibility (read-only): plugins and UI tools listing
    async fn list_plugins() -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        fn list_dir(path: &str) -> Vec<String> {
            let p = std::path::Path::new(path);
            if !p.exists() {
                return Vec::new();
            }
            match std::fs::read_dir(p) {
                Ok(rd) => rd
                    .flatten()
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect(),
                Err(_) => Vec::new(),
            }
        }
        let scripts_dir =
            std::env::var("PLUGINS_SCRIPTS_DIR").unwrap_or_else(|_| "plugins/scripts".into());
        let wasm_dir = std::env::var("PLUGINS_WASM_DIR").unwrap_or_else(|_| "plugins/wasm".into());
        let index_path =
            std::env::var("PLUGINS_INDEX_JSON").unwrap_or_else(|_| "plugins/index.json".into());
        let scripts = list_dir(&scripts_dir);
        let wasm = list_dir(&wasm_dir);
        let index = std::fs::read_to_string(&index_path)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok());
        Ok(Json(
            serde_json::json!({"scripts": scripts, "wasm": wasm, "index": index}),
        ))
    }
    async fn list_ui_tools() -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
        fn list_dir(path: &str) -> Vec<String> {
            let p = std::path::Path::new(path);
            if !p.exists() {
                return Vec::new();
            }
            match std::fs::read_dir(p) {
                Ok(rd) => rd
                    .flatten()
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect(),
                Err(_) => Vec::new(),
            }
        }
        let ui_dir = std::env::var("UI_TOOLS_DIR").unwrap_or_else(|_| "plugins/ui/tools".into());
        let alt_dir = "ui/tools";
        let mut tools = list_dir(&ui_dir);
        if tools.is_empty() {
            tools = list_dir(alt_dir);
        }
        Ok(Json(serde_json::json!({"tools": tools})))
    }
    app = app
        .route("/api/neira/plugins", get(list_plugins))
        .route("/api/neira/ui/tools", get(list_ui_tools));

    if std::env::var("DEV_ROUTES_ENABLED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        // register dev slow analysis node
        struct DevSlowNode;
        impl AnalysisNode for DevSlowNode {
            fn id(&self) -> &str {
                "dev.slow"
            }
            fn analysis_type(&self) -> &str {
                "dev"
            }
            fn status(&self) -> NodeStatus {
                NodeStatus::Active
            }
            fn links(&self) -> &[String] {
                &[]
            }
            fn confidence_threshold(&self) -> f32 {
                0.0
            }
            fn analyze(
                &self,
                input: &str,
                cancel_token: &tokio_util::sync::CancellationToken,
            ) -> AnalysisResult {
                let ms: u64 = input.trim().parse().ok().unwrap_or(5_000);
                let start = std::time::Instant::now();
                while start.elapsed().as_millis() < ms as u128 {
                    if cancel_token.is_cancelled() {
                        let mut r = AnalysisResult::new(self.id(), "cancelled", vec![]);
                        r.status = NodeStatus::Error;
                        return r;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                AnalysisResult::new(
                    self.id(),
                    format!("slept {} ms", ms),
                    vec!["dev-slow".into()],
                )
            }
            fn explain(&self) -> String {
                "Dev slow analysis for watchdog tests".into()
            }
        }
        registry.register_analysis_node(Arc::new(DevSlowNode));

        async fn dev_long_stream(
            State(state): State<AppState>,
            headers: HeaderMap,
        ) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode>
        {
            let auth = auth_from_headers(&headers).unwrap_or_default();
            if !state
                .hub
                .check_scope(&auth, backend::interaction_hub::Scope::Admin)
            {
                return Err(axum::http::StatusCode::FORBIDDEN);
            }
            let cancel = tokio_util::sync::CancellationToken::new();
            metrics::gauge!("sse_active").increment(1.0);
            let delay = std::env::var("SSE_DEV_DELAY_MS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(50);
            let count = std::env::var("SSE_DEV_TOKENS")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(200);
            let stream = stream! {
                for i in 0..count { if cancel.is_cancelled(){ break; } yield Ok(Event::default().event("message").data(format!("x{}", i))); tokio::time::sleep(std::time::Duration::from_millis(delay)).await; if i%10==0 { yield Ok(Event::default().event("progress").data(serde_json::json!({"tokens": i}).to_string())); } }
                yield Ok(Event::default().event("done").data("true")); metrics::gauge!("sse_active").decrement(1.0);
            };
            Ok(Sse::new(stream))
        }
        async fn dev_long_analysis(
            State(state): State<AppState>,
            axum::extract::Query(q): axum::extract::Query<
                std::collections::HashMap<String, String>,
            >,
        ) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
            let auth = q.get("auth").cloned().unwrap_or_default();
            if !state
                .hub
                .check_scope(&auth, backend::interaction_hub::Scope::Admin)
            {
                return Err(axum::http::StatusCode::FORBIDDEN);
            }
            let ms: u64 = q.get("ms").and_then(|v| v.parse().ok()).unwrap_or(5000);
            let token = tokio_util::sync::CancellationToken::new();
            let id = format!("dev.slow");
            match state
                .hub
                .analyze(&id, &format!("{}", ms), &auth, &token)
                .await
            {
                Some(res) => Ok(Json(
                    serde_json::to_value(&res).unwrap_or(serde_json::json!({})),
                )),
                None => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        app = app
            .route("/api/neira/dev/stream/long", get(dev_long_stream))
            .route("/api/neira/dev/analysis/long", get(dev_long_analysis));
    }
    if let Some(handle) = metrics_handle {
        app = app.route("/metrics", get(move || async move { handle.render() }));
    }
    let app = app.with_state(state);

    // Index compaction job (keywords TTL cleanup)
    let compact_every_ms = std::env::var("INDEX_COMPACT_INTERVAL_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(300_000);
    if compact_every_ms > 0 {
        tokio::spawn(async move {
            let ttl_days: i64 = std::env::var("INDEX_KW_TTL_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(90);
            let ttl_ms = ttl_days.max(0) as i64 * 86_400_000;
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(compact_every_ms)).await;
                let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
                if let Ok(rd) = std::fs::read_dir(&base) {
                    for e in rd.flatten() {
                        let idx = e.path().join("index.json");
                        if !idx.exists() {
                            continue;
                        }
                        if let Ok(s) = std::fs::read_to_string(&idx) {
                            if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&s) {
                                let now_ms = chrono::Utc::now().timestamp_millis();
                                if let Some(map) = v.as_object_mut() {
                                    let mut changed = false;
                                    for (_, entry) in map.iter_mut() {
                                        if let Some(obj) = entry.as_object_mut() {
                                            if let Some(ts) =
                                                obj.get("kw_updated_ms").and_then(|x| x.as_i64())
                                            {
                                                if ttl_ms > 0 && now_ms.saturating_sub(ts) > ttl_ms
                                                {
                                                    obj.insert(
                                                        "keywords".into(),
                                                        serde_json::Value::Array(Vec::new()),
                                                    );
                                                    obj.insert(
                                                        "kw_updated_ms".into(),
                                                        serde_json::json!(now_ms),
                                                    );
                                                    changed = true;
                                                }
                                            }
                                        }
                                    }
                                    if changed {
                                        let _ = std::fs::write(
                                            &idx,
                                            serde_json::to_string_pretty(&v)
                                                .unwrap_or_else(|_| s.clone()),
                                        );
                                        metrics::counter!("index_compact_runs").increment(1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    let bind_addr = std::env::var("NEIRA_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    // Optional periodic training
    if let Ok(ms) = std::env::var("TRAINING_INTERVAL_MS").and_then(|v| {
        v.parse::<u64>()
            .map_err(|_e| std::env::VarError::NotPresent)
    }) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
                let node =
                    backend::action::scripted_training_node::ScriptedTrainingNode::from_env();
                tokio::spawn(async move {
                    let _ = node.run().await;
                });
            }
        });
    }
    hearing::info(&format!(
        "Listening on http://{}",
        listener.local_addr().unwrap()
    ));

    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        shutdown_token.cancelled().await;
    });
    if let Err(err) = server.await {
        error!("server error: {err}");
    }
}
/* neira:meta
id: NEI-20250829-setup-meta-main
intent: docs
scope: backend/http
summary: |
  Точки входа HTTP (API), SSE с прогрессом и отменой, маскирование с пресетами,
  поиск по content с фильтрами и пагинацией, rate-limit заголовки, скоупы токенов, включён CORS.
links:
  - docs/backend-api.md
  - docs/reference/env.md
  - docs/reference/metrics.md
  - CAPABILITIES.md
env:
  - NERVOUS_SYSTEM_ENABLED
  - NERVOUS_SYSTEM_JSON_LOGS
  - INDEX_KW_TTL_DAYS
  - INDEX_COMPACT_INTERVAL_MS
  - SSE_WARN_AFTER_MS
  - PERSIST_REQUIRE_SESSION_ID
  - MASK_PRESETS_DIR
  - CHAT_RATE_LIMIT_PER_MIN
  - CHAT_RATE_KEY
endpoints:
  - POST /api/neira/chat
  - POST /api/neira/chat/stream
  - POST /api/neira/chat/stream/cancel
  - GET /api/neira/chat/{chat_id}/{session_id}/search
  - POST /api/neira/context/masking
  - GET /api/neira/context/masking/config
  - POST /api/neira/context/masking/dry_run
  - GET /metrics
metrics:
  - sse_active
  - sessions_active
  - sessions_autocreated_total
  - sessions_closed_total
  - requests_idempotent_hits
  - index_compact_runs
risks: low
safe_mode:
  affects_write: true
  requires_admin: true
i18n:
  reviewer_note: |
    Основной API и политики. При изменениях обновляй референсы и список метрик/флагов.
*/
