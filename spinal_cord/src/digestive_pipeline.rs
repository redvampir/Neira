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
use crate::cell_template::load_schema_from;
use jsonschema_valid::Config;
use once_cell::sync::Lazy;
use quick_xml::de::from_str as from_xml;
use serde_json::Value;
use serde_yaml;
use std::{env, path::PathBuf};
use thiserror::Error;

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

static DEFAULT_SCHEMA: Lazy<Result<Config<'static>, String>> = Lazy::new(|| {
    let path = env::var("DIGESTIVE_SCHEMA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas/analysis-result.schema.json")
        });
    load_schema_from(&path)
});

impl DigestivePipeline {
    pub fn ingest(raw_input: &str) -> Result<ParsedInput, PipelineError> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_input) {
            validate(&json)?;
            Ok(ParsedInput::Json(json))
        } else if let Ok(yaml) = serde_yaml::from_str::<Value>(raw_input) {
            validate(&yaml)?;
            Ok(ParsedInput::Json(yaml))
        } else if let Ok(xml) = from_xml::<Value>(raw_input) {
            validate(&xml)?;
            Ok(ParsedInput::Json(xml))
        } else {
            Ok(ParsedInput::Text(raw_input.to_string()))
        }
    }
}

fn validate(value: &Value) -> Result<(), PipelineError> {
    let cfg = DEFAULT_SCHEMA
        .as_ref()
        .map_err(|e| PipelineError::Schema(e.clone()))?;
    if let Err(errors) = cfg.validate(value) {
        let msg = errors.map(|e| e.to_string()).collect::<Vec<_>>().join("; ");
        Err(PipelineError::Validation(msg))
    } else {
        Ok(())
    }
}
