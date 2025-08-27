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
    let base = env::var("NODE_TEMPLATE_SCHEMAS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas"));
    let path = base.join(version).join("node-template.schema.json");
    let cfg = load_schema_from(&path)?;
    let cfg_static: &'static Config<'static> = Box::leak(Box::new(cfg));
    info!("Using NodeTemplate schema {}", version);
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
pub struct NodeTemplate {
    pub id: String,
    pub analysis_type: String,
    #[serde(default)]
    pub links: Vec<String>,
    pub confidence_threshold: Option<f64>,
    pub draft_content: Option<String>,
    pub metadata: Metadata,
}

impl NodeTemplate {
    pub fn to_json(&self) -> Value {
        let value = serde_json::to_value(self).expect("serialize NodeTemplate");
        #[cfg(debug_assertions)]
        {
            if let Err(errors) = validate_template(&value) {
                panic!("serialized NodeTemplate failed validation: {:?}", errors);
            }
        }
        value
    }
}

pub fn validate_template(value: &Value) -> Result<(), Vec<String>> {
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
    let schema = load_schema(&dir).map_err(|e| {
        error!("{e}");
        vec![e]
    })?;
    match schema.validate(value) {
        Ok(()) => {
            info!("NodeTemplate validation succeeded");
            Ok(())
        }
        Err(errors) => {
            let messages: Vec<String> = errors
                .map(|err| {
                    let path = if err.instance_path.is_empty() {
                        "/".to_string()
                    } else {
                        let segments: Vec<String> = err.instance_path.iter().rev().cloned().collect();
                        format!("/{}", segments.join("/"))
                    };
                    format!("{}: {}", path, err.msg)
                })
                .collect();
            error!("NodeTemplate validation failed: {:?}", messages);
            Err(messages)
        }
    }
}
