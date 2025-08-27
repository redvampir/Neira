use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use backend::node_template::{self, Metadata, NodeTemplate};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

fn main() {
    tracing_subscriber::fmt::init();
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mut schema_version: Option<String> = None;
    let mut interactive = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--schema" => {
                schema_version = args.next();
            }
            "--interactive" => interactive = true,
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let version = schema_version
        .ok_or_else(|| "usage: cargo run --bin generate_node -- --schema <version>".to_string())?;

    let dir = parse_version(&version)?;
    let base = env::var("NODE_TEMPLATE_SCHEMAS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas"));
    let path = base.join(&dir).join("node-template.schema.json");
    node_template::load_schema_from(&path)?;

    let schema_str = std::fs::read_to_string(&path)
        .map_err(|e| format!("failed to read schema {}: {e}", path.display()))?;
    let schema_json: Value = serde_json::from_str(&schema_str)
        .map_err(|e| format!("invalid schema JSON {}: {e}", path.display()))?;
    let props = schema_json
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| "invalid schema: no properties".to_string())?;

    let id =
        get_default::<String>(props, "id").or_else(|| prompt_string(interactive, "id"));
    let analysis_type = get_default::<String>(props, "analysis_type")
        .or_else(|| prompt_string(interactive, "analysis_type"));
    let links = get_default::<Vec<String>>(props, "links").unwrap_or_default();
    let confidence_threshold = get_default::<f64>(props, "confidence_threshold")
        .or_else(|| prompt_f64(interactive, "confidence_threshold"));
    let draft_content = get_default::<String>(props, "draft_content")
        .or_else(|| prompt_string(interactive, "draft_content"));

    let template = NodeTemplate {
        id: id.unwrap_or_default(),
        analysis_type: analysis_type.unwrap_or_default(),
        links,
        confidence_threshold,
        draft_content,
        metadata: Metadata {
            schema: version,
            extra: HashMap::<String, Value>::new(),
        },
    };

    let json = serde_json::to_string_pretty(&template)
        .map_err(|e| format!("failed to serialize template: {e}"))?;
    println!("{json}");
    Ok(())
}

fn parse_version(version: &str) -> Result<String, String> {
    let trimmed = version.trim_start_matches('v');
    let major = trimmed
        .split('.')
        .next()
        .ok_or_else(|| format!("invalid schema version {version}"))?;
    if major.chars().all(|c| c.is_ascii_digit()) {
        Ok(format!("v{major}"))
    } else {
        Err(format!("invalid schema version {version}"))
    }
}

fn get_default<T: DeserializeOwned>(props: &Map<String, Value>, key: &str) -> Option<T> {
    let value = props.get(key)?.get("default")?.clone();
    serde_json::from_value(value).ok()
}

fn prompt_string(interactive: bool, name: &str) -> Option<String> {
    if !interactive {
        return None;
    }
    print!("{name}: ");
    io::stdout().flush().ok()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    let trimmed = buf.trim().to_string();
    if trimmed.is_empty() { None } else { Some(trimmed) }
}

fn prompt_f64(interactive: bool, name: &str) -> Option<f64> {
    let input = prompt_string(interactive, name)?;
    input.parse().ok()
}

