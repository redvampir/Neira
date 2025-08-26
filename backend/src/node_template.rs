use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use jsonschema::JSONSchema;

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

pub fn load_schema() -> JSONSchema {
    let schema_str = include_str!("../../schemas/node-template.schema.json");
    let schema_json: Value = serde_json::from_str(schema_str).expect("invalid schema JSON");
    JSONSchema::compile(&schema_json).expect("invalid JSON schema")
}
