use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::net::TcpListener;
use tracing::{error, info};

use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::interaction_hub::InteractionHub;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use backend::node_template::NodeTemplate;

#[derive(Clone)]
struct AppState {
    hub: Arc<InteractionHub>,
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
    #[serde(default)]
    priority: Option<u8>,
}

#[derive(serde::Deserialize)]
struct ResumeRequest {
    id: String,
    auth: String,
}

async fn analyze_request(
    State(state): State<AppState>,
    Json(req): Json<AnalysisRequest>,
) -> Result<Json<AnalysisResult>, axum::http::StatusCode> {
    let token = tokio_util::sync::CancellationToken::new();
    let result = state
        .hub
        .analyze(
            &req.id,
            &req.input,
            req.priority.unwrap_or(0),
            &req.auth,
            &token,
        )
        .await
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    Ok(Json(result))
}

async fn resume_request(
    State(state): State<AppState>,
    Json(req): Json<ResumeRequest>,
) -> Result<Json<AnalysisResult>, axum::http::StatusCode> {
    state
        .hub
        .resume(&req.id, &req.auth)
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let templates_dir =
        std::env::var("NODE_TEMPLATES_DIR").unwrap_or_else(|_| "./templates".into());
    let _ = std::fs::create_dir_all(&templates_dir);
    let registry = Arc::new(NodeRegistry::new(&templates_dir).expect("registry"));
    let memory = Arc::new(MemoryNode::new());
    let hub = Arc::new(InteractionHub::new(registry.clone(), memory.clone()));
    hub.add_auth_token("secret");
    hub.add_trigger_keyword("echo");

    // Пример узла анализа
    struct EchoNode;
    impl AnalysisNode for EchoNode {
        fn id(&self) -> &str { "example.analysis" }
        fn analysis_type(&self) -> &str { "summary" }
        fn status(&self) -> NodeStatus { NodeStatus::Active }
        fn links(&self) -> &[String] { &[] }
        fn confidence_threshold(&self) -> f32 { 0.0 }
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
        fn explain(&self) -> String { "Echoes input".into() }
    }

    registry.register_analysis_node(Arc::new(EchoNode));

    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("metrics");

    let state = AppState { hub: hub.clone() };

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/nodes", post(register_node))
        .route("/nodes/:id", get(get_node_latest))
        .route("/nodes/:id/:version", get(get_node))
        .route("/api/neira/analysis", post(analyze_request))
        .route("/api/neira/analysis/resume", post(resume_request))
        .route("/metrics", get(move || async move { handle.render() }))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on http://{}", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, app).await {
        error!("server error: {err}");
    }
}
