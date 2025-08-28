use std::sync::Arc;

use async_stream::stream;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::sse::{Event, Sse},
    routing::{delete, get, post},
    Json, Router,
};
use backend::context::context_storage::set_runtime_mask_config;
use dotenvy::dotenv;
use futures_core::stream::Stream;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::convert::Infallible;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::TcpListener;
use tracing::{error, info};

use backend::action::chat_node::EchoChatNode;
use backend::action::metrics_collector_node::MetricsCollectorNode;
use backend::action::diagnostics_node::DiagnosticsNode;
use backend::action_node::PreloadAction;
use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::context::context_storage::FileContextStorage;
use backend::interaction_hub::InteractionHub;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use backend::node_template::NodeTemplate;
mod http {
    pub mod training_routes;
}

#[derive(Clone)]
struct AppState {
    hub: Arc<InteractionHub>,
    storage: Arc<FileContextStorage>,
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
struct AnalysisRequest {
    id: String,
    input: String,
    auth: String,
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
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    let token = tokio_util::sync::CancellationToken::new();
    let result = state
        .hub
        .analyze(&req.id, &req.input, &req.auth, &token)
        .await
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    Ok(Json(result))
}

async fn resume_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut req): Json<ResumeRequest>,
) -> Result<Json<AnalysisResult>, axum::http::StatusCode> {
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
) -> Result<Json<ChatResponse>, (axum::http::StatusCode, String)> {
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    let used_context = req.session_id.is_some();
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
        )
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    Ok(Json(ChatResponse {
        response: out.response,
        used_context,
        session_id: out.session_id,
        idempotent: out.idempotent,
    }))
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
    if req.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            req.auth = h;
        }
    }
    if !state.hub.check_auth(&req.auth) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
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
    if q.auth.trim().is_empty() {
        if let Some(h) = auth_from_headers(&headers) {
            q.auth = h;
        }
    }
    if !state.hub.check_auth(&q.auth) {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
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
    let used_context = req.session_id.is_some();
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
        )
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    let stream = stream! {
        use std::time::Instant;
        // first send metadata event
        let meta = serde_json::json!({
            "used_context": used_context,
            "session_id": out.session_id,
            "idempotent": out.idempotent,
            "source": req.source,
            "thread_id": req.thread_id,
        });
        yield Ok(Event::default().event("meta").data(meta.to_string()));
        // then stream chunked response by words
        let mut sent = 0usize;
        let mut chars = 0usize;
        let start = Instant::now();
        for w in out.response.split_whitespace() {
            yield Ok(Event::default().event("message").data(w.to_string()));
            sent += 1;
            chars += w.len();
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
    };
    Ok(Sse::new(stream))
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
}

async fn search_chat(
    Path((chat_id, session_id)): Path<(String, String)>,
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<(axum::http::HeaderMap, String), (axum::http::StatusCode, String)> {
    let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
    let dir = std::path::Path::new(&base).join(&chat_id);
    let mut out = String::new();
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
                    let hay = if params.prefix { lt } else { lt };
                    if regex.is_match(hay) {
                        out.push_str(lt);
                        out.push('\n');
                    }
                }
            }
        }
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
    body: String,
) -> Result<(), (axum::http::StatusCode, String)> {
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
    set_runtime_mask_config(req.enabled, req.regex, req.roles)
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

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let logs_dir = "logs";
    let _ = std::fs::create_dir_all(logs_dir);

    let file_appender = tracing_appender::rolling::daily(logs_dir, "backend.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let templates_dir =
        std::env::var("NODE_TEMPLATES_DIR").unwrap_or_else(|_| "./templates".into());
    let _ = std::fs::create_dir_all(&templates_dir);
    let registry = Arc::new(NodeRegistry::new(&templates_dir).expect("registry"));
    let memory = Arc::new(MemoryNode::new());
    let (metrics, metrics_rx) = MetricsCollectorNode::channel();
    let (diagnostics, _dev_rx) = DiagnosticsNode::new(metrics_rx, 5);
    let hub = Arc::new(InteractionHub::new(
        registry.clone(),
        memory.clone(),
        metrics,
        diagnostics,
    ));
    hub.add_auth_token("secret");
    hub.add_trigger_keyword("echo");
    registry.register_action_node(Arc::new(PreloadAction::default()));
    registry.register_scripted_training_node();
    // Register a default chat node
    registry.register_chat_node(Arc::new(EchoChatNode::default()));

    // Context storage
    let storage = Arc::new(FileContextStorage::new("context"));
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

    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("metrics");

    let state = AppState {
        hub: hub.clone(),
        storage: storage.clone(),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
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
        .route("/nodes", post(register_node))
        .route("/nodes/:id", get(get_node_latest))
        .route("/nodes/:id/:version", get(get_node))
        .route("/api/neira/analysis", post(analyze_request))
        .route("/api/neira/analysis/resume", post(resume_request))
        .route("/api/neira/chat", post(chat_request))
        .route("/api/neira/chat/stream", post(chat_stream))
        .route("/api/neira/chat/session/new", post(new_session))
        .route(
            "/api/neira/chat/:chat_id/:session_id",
            delete(delete_session),
        )
        .route(
            "/api/neira/chat/:chat_id/:session_id/rename",
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
        .route(
            "/context/*path",
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
        .route("/api/neira/chat/:chat_id/export", get(export_chat))
        .route(
            "/api/neira/chat/:chat_id/import/:session_id",
            post(import_chat),
        )
        .route("/api/neira/chat/:chat_id/index", get(get_chat_index))
        .route(
            "/api/neira/chat/:chat_id/:session_id",
            get(get_chat_session),
        )
        .route(
            "/api/neira/chat/:chat_id/:session_id/search",
            get(search_chat),
        )
        .route("/metrics", get(move || async move { handle.render() }))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
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
    info!("Listening on http://{}", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, app).await {
        error!("server error: {err}");
    }
}
