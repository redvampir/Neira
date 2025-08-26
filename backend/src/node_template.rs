use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env, fs};
use jsonschema_valid::{Config, schemas::Draft};
use tracing::error;

/// Дополнительные метаданные для `NodeTemplate`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Версия схемы.
    pub schema: String,
    /// Произвольные дополнительные поля.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Шаблон узла с параметрами анализа и ссылками.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTemplate {
    /// Уникальный идентификатор узла.
    pub id: String,
    /// Тип анализа, выполняемый узлом.
    pub analysis_type: String,
    /// Связанные узлы.
    #[serde(default)]
    pub links: Vec<String>,
    /// Порог уверенности для принятия решения.
    pub confidence_threshold: Option<f64>,
    /// Предварительное содержимое.
    pub draft_content: Option<String>,
    /// Метаданные шаблона.
    pub metadata: Metadata,
}

impl NodeTemplate {
    /// Сериализует структуру в `serde_json::Value`.
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("serialize NodeTemplate")
    }
}

static SCHEMA: OnceCell<Config<'static>> = OnceCell::new();

fn read_schema() -> Config<'static> {
    let path = env::var("NODE_TEMPLATE_SCHEMA_PATH").unwrap_or_else(|_| {
        let version = env::var("NODE_TEMPLATE_SCHEMA_VERSION").unwrap_or_else(|_| "1.0".into());
        format!("{}/../schemas/{version}/node-template.schema.json", env!("CARGO_MANIFEST_DIR"))
    });

    let schema_str =
        fs::read_to_string(&path)
            .unwrap_or_else(|_| include_str!("../../schemas/node-template.schema.json").to_string());

    let schema_json: Value = serde_json::from_str(&schema_str).expect("invalid schema JSON");
    Config::from_schema(Box::leak(Box::new(schema_json)), Some(Draft::Draft7))
        .expect("invalid JSON schema")
}

/// Загружает и кэширует JSON-схему `NodeTemplate`.
pub fn load_schema() -> &'static Config<'static> {
    SCHEMA.get_or_init(read_schema)
}

/// Проверяет соответствие JSON структуре `NodeTemplate`.
pub fn validate(value: &Value) -> Result<NodeTemplate, Vec<String>> {
    let schema = load_schema();
    if let Err(errors) = schema.validate(value) {
        let msgs: Vec<String> = errors.map(|e| e.to_string()).collect();
        for msg in &msgs {
            error!("Ошибка валидации: {msg}");
        }
        Err(msgs)
    } else {
        serde_json::from_value(value.clone()).map_err(|e| vec![e.to_string()])
    }
}
