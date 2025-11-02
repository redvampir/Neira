use async_stream::stream;
use axum::response::sse::{Event, Sse};
use axum::{http::StatusCode, Json};
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;

use backend::action::scripted_training_cell::ScriptedTrainingCell;
/* neira:meta
id: NEI-20250101-000002-training-context-dir
intent: refactor
summary: Тренировочные маршруты используют context_dir() вместо прямого чтения CONTEXT_DIR.
*/
use backend::context::context_dir;

#[derive(Deserialize)]
pub struct TrainingRunReq {
    pub script: Option<String>,
    pub dry_run: Option<bool>,
}

#[derive(Serialize)]
pub struct TrainingRunResp {
    pub started: bool,
}

pub async fn training_run(
    Json(req): Json<TrainingRunReq>,
) -> Result<Json<TrainingRunResp>, (StatusCode, String)> {
    if let Some(s) = req.script {
        std::env::set_var("TRAINING_SCRIPT", s);
    }
    if let Some(dr) = req.dry_run {
        std::env::set_var("TRAINING_DRY_RUN", if dr { "true" } else { "false" });
    }
    let cell = ScriptedTrainingCell::from_env();
    tokio::spawn(async move {
        let _ = cell.run().await;
    });
    Ok(Json(TrainingRunResp { started: true }))
}

pub async fn training_status() -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let progress = std::env::var("TRAINING_PROGRESS")
        .unwrap_or_else(|_| "context/training/progress.json".into());
    if let Ok(s) = std::fs::read_to_string(progress) {
        serde_json::from_str::<serde_json::Value>(&s)
            .map(Json)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    } else {
        Ok(Json(serde_json::json!({})))
    }
}

fn latest_training_file() -> Option<std::path::PathBuf> {
    let dir = context_dir().join("training");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("ndjson"))
        .collect();
    files.sort();
    files.pop()
}

#[derive(serde::Deserialize)]
pub struct StreamQuery {
    chat_id: Option<String>,
    session_id: Option<String>,
    interval_ms: Option<u64>,
    from_offset: Option<u64>,
}

pub async fn training_stream(
    axum::extract::Query(q): axum::extract::Query<StreamQuery>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let mut offset: u64 = q.from_offset.unwrap_or(0);
    let filter_chat = q.chat_id.unwrap_or_else(|| "training".into());
    let filter_sess = q.session_id.unwrap_or_else(|| "run".into());
    let mut current: Option<std::path::PathBuf> = latest_training_file().or_else(|| {
        let dir = context_dir().join(&filter_chat);
        let file = format!("{}.ndjson", filter_sess);
        let p = dir.join(file);
        if p.exists() {
            Some(p)
        } else {
            None
        }
    });
    let stream = stream! {
        let mut ticker = IntervalStream::new(tokio::time::interval(std::time::Duration::from_millis(q.interval_ms.unwrap_or(1000))));
        let mut id: u64 = 0;
        while ticker.next().await.is_some() {
            if current.is_none() { current = latest_training_file(); offset = 0; }
            if let Some(ref p) = current {
                if let Ok(meta) = std::fs::metadata(p) {
                    if meta.len() < offset { offset = 0; }
                    if meta.len() > offset {
                        if let Ok(mut f) = std::fs::File::open(p) {
                            use std::io::{Seek, SeekFrom, Read};
                            let _ = f.seek(SeekFrom::Start(offset));
                            let mut buf = String::new();
                            if f.read_to_string(&mut buf).is_ok() {
                                offset = meta.len();
                                for line in buf.lines() {
                                    id += 1;
                                    let ev = Event::default().id(id.to_string()).data(line);
                                    yield Ok(ev);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    Sse::new(stream)
}

/* neira:meta
id: NEI-20240513-training-routes-lints
intent: chore
summary: Убраны предупреждения Clippy в training_routes: убран лишний паттерн и to_string.
*/
