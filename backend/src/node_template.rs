use jsonschema::JSONSchema;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info};

pub fn load_schema_from(path: &Path) -> Result<JSONSchema, String> {
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
    let schema = JSONSchema::compile(&schema_json).map_err(|e| {
        let msg = format!("invalid JSON schema {}: {e}", path.display());
        error!("{msg}");
        msg
    })?;
    info!("Loaded JSON schema {}", path.display());
    Ok(schema)
}

static SCHEMA: OnceCell<JSONSchema> = OnceCell::new();
const SCHEMA_VERSION: &str = "1.0.0";

pub fn load_schema() -> Result<&'static JSONSchema, String> {
    let schema = SCHEMA
        .get_or_try_init(|| {
            let base = env::var("NODE_TEMPLATE_SCHEMA_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas/node-template")
                });
            let path = base.join(format!("v{SCHEMA_VERSION}.json"));
            load_schema_from(&path)
        })
        .map_err(|e| {
            error!("{e}");
            e
        })?;
    info!("Using NodeTemplate schema v{SCHEMA_VERSION}");
    Ok(schema)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub schema: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    if version != SCHEMA_VERSION {
        let msg = format!("unknown schema version {version}");
        error!("{msg}");
        return Err(vec![msg]);
    }
    let schema = load_schema().map_err(|e| {
        error!("{e}");
        vec![e]
    })?;
    let result = schema.validate(value);
    match result {
        Ok(_) => {
            info!("NodeTemplate validation succeeded");
            Ok(())
        }
        Err(errors) => {
            let messages: Vec<String> = errors
                .map(|error| format!("{}: {}", error.instance_path, error))
                .collect();
            error!("NodeTemplate validation failed: {:?}", messages);
            Err(messages)
        }
    }
}
