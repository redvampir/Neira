use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_schema_from(path: &Path) -> Result<JSONSchema, String> {
    let schema_str = fs::read_to_string(path)
        .map_err(|e| format!("failed to read schema {}: {e}", path.display()))?;
    let schema_json: Value =
        serde_json::from_str(&schema_str).map_err(|e| format!("invalid schema JSON {}: {e}", path.display()))?;
    JSONSchema::compile(&schema_json)
        .map_err(|e| format!("invalid JSON schema {}: {e}", path.display()))
}

pub fn load_schema(version: &str) -> Result<JSONSchema, String> {
    let base = env::var("NODE_TEMPLATE_SCHEMA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas/node-template")
        });
    let path = base.join(format!("v{version}.json"));
    load_schema_from(&path)
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

pub fn validate_template(value: &Value) -> Result<(), Vec<String>> {
    let version = value
        .get("metadata")
        .and_then(|m| m.get("schema"))
        .and_then(|s| s.as_str())
        .ok_or_else(|| vec!["metadata.schema is required".to_string()])?;
    let schema = load_schema(version).map_err(|e| vec![e])?;
    let result = schema.validate(value);
    match result {
        Ok(_) => Ok(()),
        Err(errors) => {
            let messages = errors
                .map(|error| format!("{}: {}", error.instance_path, error))
                .collect();
            Err(messages)
        }
    }
}
