/* neira:meta
id: NEI-20250829-175425-cell-template
intent: docs
scope: spinal_cord/core
summary: |
  Загружает и валидирует шаблоны ячеек по JSON‑схеме.
env:
  - CELL_TEMPLATE_SCHEMAS_DIR
*/

use jsonschema_valid::{self, Config};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tracing::{error, info};

pub fn load_schema_from(path: &Path) -> Result<Config<'static>, String> {
    let schema_str = fs::read_to_string(path).map_err(|e| {
        let msg = format!("failed to read schema {}: {e}", path.display());
        error!("{msg}");
        msg
    })?;
    let schema_json: Value = serde_json::from_str(&schema_str).map_err(|e| {
        let msg = format!("invalid schema JSON {}: {e}", path.display());
        error!("{msg}");
        msg
    })?;
    let schema_ref: &'static Value = Box::leak(Box::new(schema_json));
    let cfg = Config::from_schema(schema_ref, None).map_err(|e| {
        let msg = format!("invalid JSON schema {}: {e}", path.display());
        error!("{msg}");
        msg
    })?;
    cfg.validate_schema().map_err(|errors| {
        let messages: Vec<String> = errors.map(|err| format!("{}", err)).collect();
        let msg = format!("invalid JSON schema {}: {:?}", path.display(), messages);
        error!("{msg}");
        msg
    })?;
    info!("Loaded JSON schema {}", path.display());
    Ok(cfg)
}

static SCHEMAS: Lazy<Mutex<HashMap<String, &'static Config<'static>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/* neira:meta
id: NEI-20250214-152000-action-schema-cache
intent: feature
summary: |
  Кэш конфигураций JSON‑схем для шаблонов ячеек действий.
*/
static ACTION_SCHEMAS: Lazy<Mutex<HashMap<String, &'static Config<'static>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn parse_version(version: &str) -> Result<String, String> {
    let trimmed = version.trim_start_matches('v');
    let major = trimmed
        .split('.')
        .next()
        .ok_or_else(|| format!("invalid schema version {version}"))?;
    if major.chars().all(|c| c.is_ascii_digit()) {
        Ok(format!("v{}", major))
    } else {
        Err(format!("invalid schema version {version}"))
    }
}

fn load_schema(version: &str) -> Result<&'static Config<'static>, String> {
    let mut map = SCHEMAS.lock().map_err(|e| e.to_string())?;
    if let Some(cfg) = map.get(version) {
        return Ok(*cfg);
    }
    let base = env::var("CELL_TEMPLATE_SCHEMAS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas"));
    let path = base.join(version).join("cell-template.schema.json");
    let cfg = load_schema_from(&path)?;
    let cfg_static: &'static Config<'static> = Box::leak(Box::new(cfg));
    info!("Using CellTemplate schema {}", version);
    map.insert(version.to_string(), cfg_static);
    Ok(cfg_static)
}

fn load_action_schema(version: &str) -> Result<&'static Config<'static>, String> {
    let mut map = ACTION_SCHEMAS.lock().map_err(|e| e.to_string())?;
    if let Some(cfg) = map.get(version) {
        return Ok(*cfg);
    }
    let base = env::var("CELL_TEMPLATE_SCHEMAS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas"));
    let path = base.join(version).join("action-cell-template.schema.json");
    let cfg = load_schema_from(&path)?;
    let cfg_static: &'static Config<'static> = Box::leak(Box::new(cfg));
    info!("Using ActionCellTemplate schema {}", version);
    map.insert(version.to_string(), cfg_static);
    Ok(cfg_static)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub schema: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CellTemplate {
    pub id: String,
    pub version: String,
    pub analysis_type: String,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_content: Option<String>,
    pub metadata: Metadata,
}

impl CellTemplate {
    pub fn to_json(&self) -> Value {
        let value = serde_json::to_value(self).expect("serialize CellTemplate");
        #[cfg(debug_assertions)]
        {
            if let Err(errors) = validate_template(&value) {
                panic!("serialized CellTemplate failed validation: {:?}", errors);
            }
        }
        value
    }
}

/* neira:meta
id: NEI-20250214-152500-action-cell-template
intent: feature
summary: |
  Структура шаблона ячейки действия и преобразование в JSON.
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionCellTemplate {
    pub id: String,
    pub version: String,
    pub action_type: String,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_content: Option<String>,
    pub metadata: Metadata,
}

impl ActionCellTemplate {
    pub fn to_json(&self) -> Value {
        let value = serde_json::to_value(self).expect("serialize ActionCellTemplate");
        #[cfg(debug_assertions)]
        {
            if let Err(errors) = validate_action_template(&value) {
                panic!(
                    "serialized ActionCellTemplate failed validation: {:?}",
                    errors
                );
            }
        }
        value
    }
}

/* neira:meta
id: NEI-20250214-154500-validate-with-loader
intent: refactor
summary: |
  Вынос общей логики валидации в validate_with_loader.
*/
fn validate_with_loader<F>(value: &Value, load_schema_fn: F) -> Result<(), Vec<String>>
where
    F: Fn(&str) -> Result<&'static Config<'static>, String>,
{
    let version = value
        .get("metadata")
        .and_then(|m| m.get("schema"))
        .and_then(|s| s.as_str())
        .ok_or_else(|| {
            let msg = "metadata.schema is required".to_string();
            error!("{msg}");
            vec![msg]
        })?;
    let dir = parse_version(version).map_err(|msg| {
        error!("{msg}");
        vec![msg]
    })?;
    let schema = load_schema_fn(&dir).map_err(|e| {
        error!("{e}");
        vec![e]
    })?;
    match schema.validate(value) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let messages: Vec<String> = errors
                .map(|err| {
                    let path = if err.instance_path.is_empty() {
                        "/".to_string()
                    } else {
                        let segments: Vec<String> =
                            err.instance_path.iter().rev().cloned().collect();
                        format!("/{}", segments.join("/"))
                    };
                    format!("{}: {}", path, err.msg)
                })
                .collect();
            Err(messages)
        }
    }
}

pub fn validate_template(value: &Value) -> Result<(), Vec<String>> {
    match validate_with_loader(value, load_schema) {
        Ok(()) => {
            info!("CellTemplate validation succeeded");
            Ok(())
        }
        Err(errors) => {
            error!("CellTemplate validation failed: {:?}", errors);
            Err(errors)
        }
    }
}

/* neira:meta
id: NEI-20250214-153000-validate-action-template
intent: feature
summary: |
  Валидация ActionCellTemplate по соответствующей JSON‑схеме.
*/
pub fn validate_action_template(value: &Value) -> Result<(), Vec<String>> {
    match validate_with_loader(value, load_action_schema) {
        Ok(()) => {
            info!("ActionCellTemplate validation succeeded");
            Ok(())
        }
        Err(errors) => {
            error!("ActionCellTemplate validation failed: {:?}", errors);
            Err(errors)
        }
    }
}
