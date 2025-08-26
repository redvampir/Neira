use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_schema_from(path: &Path) -> JSONSchema {
    let schema_str = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read schema {}: {e}", path.display()));
    let schema_json: Value = serde_json::from_str(&schema_str).expect("invalid schema JSON");
    JSONSchema::compile(&schema_json).expect("invalid JSON schema")
}

static SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let path = env::var("NODE_TEMPLATE_SCHEMA_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas/node-template.schema.json")
        });
    load_schema_from(&path)
});

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
    match SCHEMA.validate(value) {
        Ok(_) => Ok(()),
        Err(errors) => {
            let messages = errors
                .map(|error| format!("{}: {}", error.instance_path, error))
                .collect();
            Err(messages)
        }
    }
}

pub fn load_schema() -> &'static JSONSchema {
    &SCHEMA
}
