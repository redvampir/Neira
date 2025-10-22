/* neira:meta
id: NEI-20260301-anti-idle-module
intent: code
summary: |-
  Вынесен модуль Anti-Idle: хранит состояние, считает idle_* и
  предоставляет REST-ручку `/api/neira/anti_idle/toggle`.
*/
/* neira:meta
id: NEI-20250220-env-flag-anti-idle
intent: refactor
summary: Флаг ANTI_IDLE_ENABLED читается через env_flag.
*/
use axum::extract::FromRef;
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::synapse_hub::{Scope, SynapseHub};

#[derive(Clone, Copy)]
pub struct IdleThresholds {
    pub idle_secs: u64,
    pub long_secs: u64,
    pub deep_secs: u64,
}

static ENABLED: AtomicBool = AtomicBool::new(true);
static LAST_ACTIVITY: AtomicU64 = AtomicU64::new(0);
static THRESHOLDS: OnceLock<IdleThresholds> = OnceLock::new();
static EMA_ALPHA: OnceLock<f64> = OnceLock::new();
static DRYRUN_DEPTH: OnceLock<u64> = OnceLock::new();

pub fn init() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    LAST_ACTIVITY.store(now, Ordering::Relaxed);
    let enabled = crate::config::env_flag("ANTI_IDLE_ENABLED", true);
    ENABLED.store(enabled, Ordering::Relaxed);
    let _ = thresholds();
    let _ = ema_alpha();
    let _ = dryrun_queue_depth();
}

pub fn mark_activity() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    LAST_ACTIVITY.store(now, Ordering::Relaxed);
}

pub fn seconds_since_last_activity() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let last = LAST_ACTIVITY.load(Ordering::Relaxed);
    now.saturating_sub(last)
}

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn set_enabled(v: bool) {
    ENABLED.store(v, Ordering::Relaxed);
}

pub fn thresholds() -> &'static IdleThresholds {
    THRESHOLDS.get_or_init(|| {
        let idle_secs = std::env::var("IDLE_THRESHOLD_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);
        let long_secs = std::env::var("LONG_IDLE_THRESHOLD_MINUTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5)
            * 60;
        let deep_secs = std::env::var("DEEP_IDLE_THRESHOLD_MINUTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30)
            * 60;
        IdleThresholds {
            idle_secs,
            long_secs,
            deep_secs,
        }
    })
}

pub fn ema_alpha() -> f64 {
    *EMA_ALPHA.get_or_init(|| {
        std::env::var("IDLE_EMA_ALPHA")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.3)
    })
}

pub fn dryrun_queue_depth() -> u64 {
    *DRYRUN_DEPTH.get_or_init(|| {
        std::env::var("IDLE_DRYRUN_QUEUE_DEPTH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    })
}

pub fn idle_state(active_streams: usize) -> (u32, u64) {
    let since = seconds_since_last_activity();
    let t = thresholds();
    let state_idx = if active_streams > 0 || since < t.idle_secs {
        0
    } else if since < t.long_secs {
        1
    } else if since < t.deep_secs {
        2
    } else {
        3
    };
    (state_idx, since)
}

#[derive(Deserialize)]
struct ToggleRequest {
    auth: String,
    enabled: Option<bool>,
}

#[derive(Serialize)]
struct ToggleResponse {
    enabled: bool,
}

async fn toggle<S>(
    State(hub): State<Arc<SynapseHub>>,
    Json(req): Json<ToggleRequest>,
) -> Result<Json<ToggleResponse>, axum::http::StatusCode>
where
    Arc<SynapseHub>: FromRef<S>,
{
    if !hub.check_auth(&req.auth) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    if !hub.check_scope(&req.auth, Scope::Admin) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    let new_state = req.enabled.unwrap_or(!is_enabled());
    set_enabled(new_state);
    Ok(Json(ToggleResponse { enabled: new_state }))
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<SynapseHub>: FromRef<S>,
{
    Router::new().route("/api/neira/anti_idle/toggle", post(toggle::<S>))
}
