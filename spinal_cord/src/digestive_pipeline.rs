/* neira:meta
id: NEI-20260530-digestive-pipeline
intent: feature
summary: |
  Добавлен DigestivePipeline: преобразует сырой ввод в ParsedInput с проверкой JSON-схемы.
*/
/* neira:meta
id: NEI-20260601-digestive-xml-yaml
intent: feature
summary: |
  DigestivePipeline распознаёт YAML и XML, конвертируя их в ParsedInput::Json.
*/
/* neira:meta
id: NEI-20260710-quick-xml
intent: chore
summary: Использован quick-xml вместо serde_xml_rs для разбора XML.
*/
/* neira:meta
id: NEI-20260725-digestive-config-path
intent: refactor
summary: Путь к JSON Schema берётся из файла конфигурации.
*/
/* neira:meta
id: NEI-20260920-digestive-tracing
intent: chore
summary: Добавлены tracing-логи входа, формата и результата валидации.
*/
/* neira:meta
id: NEI-20261005-digestive-metrics
intent: feature
summary: Замерен время парсинга и проверки схемы с отправкой в time_metrics.
*/
/* neira:meta
id: NEI-20261015-digestive-cache
intent: refactor
summary: Добавлен глобальный кэш JSON Schema.
*/
use crate::cell_template::load_schema_from;
use crate::time_metrics::{record_parse_duration_ms, record_validation_duration_ms};
use jsonschema_valid::Config;
use once_cell::sync::Lazy;
use quick_xml::de::from_str as from_xml;
use serde::Deserialize;
use serde_json::Value;
use serde_yaml;
use std::{
    collections::HashMap,
    env,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Instant,
};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
pub enum ParsedInput {
    Json(Value),
    Text(String),
}

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("schema load failed: {0}")]
    Schema(String),
}

pub struct DigestivePipeline;

#[derive(Deserialize)]
struct DigestiveSettings {
    schema_path: String,
}

static SCHEMA_CACHE: Lazy<Mutex<HashMap<PathBuf, Arc<Config<'static>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

impl DigestivePipeline {
    pub fn init() -> Result<(), PipelineError> {
        default_schema().map(|_| ()).map_err(PipelineError::Schema)
    }

    pub fn ingest(raw_input: &str) -> Result<ParsedInput, PipelineError> {
        debug!("ingest input: {raw_input}");
        let start = Instant::now();
        if let Ok(json) = serde_json::from_str::<Value>(raw_input) {
            info!("detected json input");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            validate(&json)?;
            Ok(ParsedInput::Json(json))
        } else if let Ok(yaml) = serde_yaml::from_str::<Value>(raw_input) {
            info!("detected yaml input");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            validate(&yaml)?;
            Ok(ParsedInput::Json(yaml))
        } else if let Ok(xml) = from_xml::<Value>(raw_input) {
            info!("detected xml input");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            validate(&xml)?;
            Ok(ParsedInput::Json(xml))
        } else {
            warn!("unknown input format, treating as text");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            Ok(ParsedInput::Text(raw_input.to_string()))
        }
    }

    /// Сбрасывает внутренний кэш схем.
    pub fn reset_cache() {
        SCHEMA_CACHE.lock().unwrap().clear();
    }
}

fn validate(value: &Value) -> Result<(), PipelineError> {
    let cfg = default_schema().map_err(PipelineError::Schema)?;
    let start = Instant::now();
    let res = cfg.validate(value);
    record_validation_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
    if let Err(errors) = res {
        let msg = errors.map(|e| e.to_string()).collect::<Vec<_>>().join("; ");
        warn!("validation failed: {msg}");
        Err(PipelineError::Validation(msg))
    } else {
        debug!("validation passed");
        Ok(())
    }
}

fn default_schema() -> Result<Arc<Config<'static>>, String> {
    let cfg_path = env::var("DIGESTIVE_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/digestive.toml")
        });
    let raw = fs::read_to_string(&cfg_path).map_err(|e| e.to_string())?;
    let settings: DigestiveSettings = toml::from_str(&raw).map_err(|e| e.to_string())?;
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(settings.schema_path);
    load_schema_cached(&path)
}

fn load_schema_cached(path: &Path) -> Result<Arc<Config<'static>>, String> {
    let mut cache = SCHEMA_CACHE.lock().unwrap();
    if let Some(cfg) = cache.get(path) {
        return Ok(cfg.clone());
    }
    let cfg = load_schema_from(path)?;
    let arc = Arc::new(cfg);
    cache.insert(path.to_path_buf(), arc.clone());
    Ok(arc)
}
