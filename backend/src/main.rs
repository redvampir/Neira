use std::sync::Arc;

use axum::{routing::{get, post}, Router, extract::{State, Path}, Json};
use tokio::net::TcpListener;
use tracing::{error, info};
use metrics_exporter_prometheus::PrometheusBuilder;

use backend::node_registry::NodeRegistry;
use backend::node_template::NodeTemplate;

#[derive(Clone)]
struct AppState {
    registry: Arc<NodeRegistry>,
}

async fn register_node(
    State(state): State<AppState>,
    Json(tpl): Json<NodeTemplate>,
) -> Result<String, (axum::http::StatusCode, String)> {
    state
        .registry
        .register_template(tpl)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    Ok("registered".to_string())
}

async fn get_node(
    State(state): State<AppState>,
    Path((id, version)): Path<(String, String)>,
) -> Result<Json<NodeTemplate>, axum::http::StatusCode> {
    state
        .registry
        .get(&id, Some(&version))
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

async fn get_node_latest(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<NodeTemplate>, axum::http::StatusCode> {
    state
        .registry
        .get(&id, None)
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let templates_dir = std::env::var("NODE_TEMPLATES_DIR").unwrap_or_else(|_| "./templates".into());
    let _ = std::fs::create_dir_all(&templates_dir);
    let registry = Arc::new(NodeRegistry::new(&templates_dir).expect("registry"));

    let handle = PrometheusBuilder::new().install_recorder().expect("metrics");

    let state = AppState { registry: registry.clone() };

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/nodes", post(register_node))
        .route("/nodes/:id", get(get_node_latest))
        .route("/nodes/:id/:version", get(get_node))
        .route("/metrics", get(move || async move { handle.render() }))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on http://{}", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, app).await {
        error!("server error: {err}");
    }
}
