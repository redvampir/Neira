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
/* neira:meta
id: NEI-20261020-digestive-settings-cache
intent: refactor
summary: Кэшируются настройки DigestivePipeline с очисткой через reset_cache.
*/
/* neira:meta
id: NEI-20261124-digestive-memory-store
intent: feature
summary: После парсинга вход сохраняется в MemoryCell.
*/
/* neira:meta
id: NEI-20270307-digestive-fallback-schema
intent: feature
summary: Использован запасной JSON Schema при отсутствии основной.
*/
/* neira:meta
id: NEI-20270405-digestive-toxicity-filter
intent: feature
summary: Добавлены базовые фильтры токсичных слов перед TriggerDetector.
*/
/* neira:meta
id: NEI-20270420-digestive-strict-typing
intent: refactor
summary: Добавлены serde-атрибуты и PathBuf для строгой типизации DigestiveSettings.
*/
/* neira:meta
id: NEI-20270715-digestive-xml-flatten
intent: fix
summary: Раскрыты узлы `$text` в XML перед валидацией.
*/
use crate::cell_template::load_schema_from;
use crate::memory_cell::MemoryCell;
use crate::time_metrics::{record_parse_duration_ms, record_validation_duration_ms};
use jsonschema_valid::Config;
use once_cell::sync::{Lazy, OnceCell};
use quick_xml::de::from_str as from_xml;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use serde_yaml;
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Instant,
};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DigestiveSettings {
    #[serde(deserialize_with = "deserialize_schema_path")]
    pub schema_path: PathBuf,
}

fn deserialize_schema_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}

static SCHEMA_CACHE: Lazy<Mutex<HashMap<PathBuf, Arc<Config<'static>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static SETTINGS_CACHE: Lazy<Mutex<Option<DigestiveSettings>>> = Lazy::new(|| Mutex::new(None));

static MEMORY: OnceCell<Arc<MemoryCell>> = OnceCell::new();

static TOXIC_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    [
        r"(?i)идиот",
        r"(?i)оскорб",
        r"(?i)\b(?:terror|extremism)\b",
        r"(?i)\b(?:suicide|kill yourself)\b",
        r"(?:\u{fffd}|\?){2,}",
    ]
    .into_iter()
    .map(|pattern| Regex::new(pattern).expect("invalid toxicity pattern"))
    .collect()
});

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
pub(super) static CONFIG_READS: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
fn read_config(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

#[cfg(test)]
fn read_config(path: &Path) -> std::io::Result<String> {
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);
    fs::read_to_string(path)
}

impl DigestivePipeline {
    pub fn init() -> Result<(), PipelineError> {
        default_schema().map(|_| ()).map_err(PipelineError::Schema)
    }

    pub fn set_memory(memory: Arc<MemoryCell>) {
        let _ = MEMORY.set(memory);
    }

    pub fn ingest(raw_input: &str) -> Result<ParsedInput, PipelineError> {
        debug!("ingest input: {raw_input}");
        let start = Instant::now();
        let trimmed = raw_input.trim_start();
        let parsed = if let Ok(mut json) = serde_json::from_str::<Value>(raw_input) {
            info!("detected json input");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            sanitize_value(&mut json);
            validate(&json, None)?;
            ParsedInput::Json(json)
        } else if trimmed.starts_with('<') {
            match from_xml::<Value>(raw_input) {
                Ok(mut xml) => {
                    info!("detected xml input");
                    record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
                    let root_tag = extract_root_tag(trimmed);
                    sanitize_value(&mut xml);
                    ensure_root_wrapper(&mut xml, root_tag.as_deref());
                    flatten_xml_text(&mut xml);
                    ensure_root_wrapper(&mut xml, root_tag.as_deref());
                    validate(&xml, root_tag.as_deref())?;
                    ParsedInput::Json(xml)
                }
                Err(_) => {
                    warn!("unknown input format, treating as text");
                    record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
                    ParsedInput::Text(sanitize_text(raw_input))
                }
            }
        } else if let Ok(mut yaml) = serde_yaml::from_str::<Value>(raw_input) {
            info!("detected yaml input");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            sanitize_value(&mut yaml);
            validate(&yaml, None)?;
            ParsedInput::Json(yaml)
        } else {
            warn!("unknown input format, treating as text");
            record_parse_duration_ms(start.elapsed().as_secs_f64() * 1000.0);
            ParsedInput::Text(sanitize_text(raw_input))
        };
        if let Some(mem) = MEMORY.get() {
            mem.store_parsed_input(parsed.clone());
        }
        Ok(parsed)
    }

    pub fn sanitize(raw: &str) -> String {
        sanitize_text(raw)
    }

    /// Сбрасывает внутренние кэши схем и настроек.
    pub fn reset_cache() {
        SCHEMA_CACHE.lock().unwrap().clear();
        SETTINGS_CACHE.lock().unwrap().take();
    }
}

fn sanitize_text(text: &str) -> String {
    let mut result = text.to_string();
    for re in TOXIC_PATTERNS.iter() {
        result = re.replace_all(&result, "[censored]").into_owned();
    }
    result
}

fn sanitize_value(value: &mut Value) {
    match value {
        Value::String(s) => {
            *s = sanitize_text(s);
        }
        Value::Array(arr) => {
            for v in arr {
                sanitize_value(v);
            }
        }
        Value::Object(map) => {
            for v in map.values_mut() {
                sanitize_value(v);
            }
        }
        _ => {}
    }
}

fn flatten_xml_text(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if map.len() == 1 && map.contains_key("$text") {
                if let Some(mut inner) = map.remove("$text") {
                    flatten_xml_text(&mut inner);
                    *value = inner;
                }
            } else {
                for v in map.values_mut() {
                    flatten_xml_text(v);
                }
            }
        }
        Value::Array(arr) => {
            if arr.len() == 1 {
                let mut single = arr.pop().unwrap();
                flatten_xml_text(&mut single);
                if matches!(single, Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null) {
                    *value = single;
                    return;
                }
                arr.push(single);
            }
            for v in arr.iter_mut() {
                flatten_xml_text(v);
            }
        }
        _ => {}
    }
}


fn ensure_root_wrapper(value: &mut Value, root: Option<&str>) {
    let Some(name) = root else {
        return;
    };

    if matches!(value, Value::Object(map) if map.contains_key(name)) {
        return;
    }

    let mut wrapper = Map::new();
    wrapper.insert(name.to_string(), value.take());
    *value = Value::Object(wrapper);
}

fn extract_root_tag(input: &str) -> Option<String> {
    let mut rest = input.trim_start();

    loop {
        if let Some(stripped) = rest.strip_prefix("<?") {
            if let Some(idx) = stripped.find("?>") {
                rest = stripped[idx + 2..].trim_start();
                continue;
            } else {
                return None;
            }
        } else if let Some(stripped) = rest.strip_prefix("<!") {
            if let Some(idx) = stripped.find('>') {
                rest = stripped[idx + 1..].trim_start();
                continue;
            } else {
                return None;
            }
        }
        break;
    }

    if !rest.starts_with('<') {
        return None;
    }

    let mut name = String::new();
    for ch in rest[1..].chars() {
        if ch.is_whitespace() || ch == '>' || ch == '/' {
            break;
        }
        name.push(ch);
    }

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn validate(value: &Value, root_tag: Option<&str>) -> Result<(), PipelineError> {
    let cfg = default_schema().map_err(PipelineError::Schema)?;
    let start = Instant::now();

    let mut validation_ok = false;
    let mut failure_message: Option<String> = None;

    match cfg.validate(value) {
        Ok(_) => {
            validation_ok = true;
        }
        Err(primary_errors) => {
            let primary_msg = format_validation_errors(primary_errors);
            if let Some(name) = root_tag {
                if let Value::Object(map) = value {
                    if let Some(inner) = map.get(name) {
                        match cfg.validate(inner) {
                            Ok(_) => {
                                validation_ok = true;
                            }
                            Err(fallback_errors) => {
                                failure_message =
                                    Some(format_validation_errors(fallback_errors));
                            }
                        }
                    }
                }
            }
            if !validation_ok && failure_message.is_none() {
                failure_message = Some(primary_msg);
            }
        }
    }

    record_validation_duration_ms(start.elapsed().as_secs_f64() * 1000.0);

    if validation_ok {
        debug!("validation passed");
        Ok(())
    } else {
        let msg = failure_message.unwrap_or_else(|| "validation failed".to_string());
        warn!("validation failed: {msg}");
        Err(PipelineError::Validation(msg))
    }
}

fn format_validation_errors<E>(errors: E) -> String
where
    E: IntoIterator<Item = jsonschema_valid::ValidationError>,
{
    errors
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("; ")
}

fn default_schema() -> Result<Arc<Config<'static>>, String> {
    let settings = {
        let mut cache = SETTINGS_CACHE.lock().unwrap();
        if let Some(s) = cache.clone() {
            s
        } else {
            let cfg_path = env::var("DIGESTIVE_CONFIG")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/digestive.toml")
                });
            let raw = read_config(&cfg_path).map_err(|e| e.to_string())?;
            let parsed: DigestiveSettings = toml::from_str(&raw).map_err(|e| e.to_string())?;
            *cache = Some(parsed.clone());
            parsed
        }
    };
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let main_path = base_dir.join(&settings.schema_path);
    let schema_path = if main_path.exists() {
        main_path
    } else {
        let fallback = base_dir.join("../schemas/default.schema.json");
        warn!(
            "schema not found at {}, falling back to {}",
            main_path.display(),
            fallback.display()
        );
        fallback
    };
    load_schema_cached(&schema_path)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::Ordering;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn reads_config_once() {
        DigestivePipeline::reset_cache();
        CONFIG_READS.store(0, Ordering::Relaxed);

        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("schema.json");
        fs::write(&schema_path, "{\"type\":\"object\"}").unwrap();

        let cfg_path = dir.path().join("digestive.toml");
        fs::write(
            &cfg_path,
            format!("schema_path = \"{}\"", toml_escape_path(&schema_path)),
        )
        .unwrap();

        std::env::set_var("DIGESTIVE_CONFIG", cfg_path.to_str().unwrap());
        super::default_schema().unwrap();
        super::default_schema().unwrap();
        assert_eq!(CONFIG_READS.load(Ordering::Relaxed), 1);

        std::env::remove_var("DIGESTIVE_CONFIG");
        DigestivePipeline::reset_cache();
    }

    #[test]
    #[serial]
    fn uses_fallback_schema_when_main_missing() {
        DigestivePipeline::reset_cache();

        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join("digestive.toml");
        fs::write(
            &cfg_path,
            format!(
                "schema_path = \"{}\"",
                toml_escape_path(Path::new("missing.json"))
            ),
        )
        .unwrap();

        std::env::set_var("DIGESTIVE_CONFIG", cfg_path.to_str().unwrap());
        super::default_schema().expect("fallback schema loads");

        let cache = SCHEMA_CACHE.lock().unwrap();
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let main = base.join("missing.json");
        let fallback = base.join("../schemas/default.schema.json");
        assert!(cache.contains_key(&fallback));
        assert!(!cache.contains_key(&main));
        drop(cache);

        std::env::remove_var("DIGESTIVE_CONFIG");
        DigestivePipeline::reset_cache();
    }

    fn toml_escape_path(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "\\\\")
    }
}
