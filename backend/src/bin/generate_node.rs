use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use backend::node_template::{self, Metadata, NodeTemplate};
use serde_json::Value;

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
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--schema" => {
                schema_version = args.next();
            }
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

    let template = NodeTemplate {
        id: String::new(),
        analysis_type: String::new(),
        links: Vec::new(),
        confidence_threshold: None,
        draft_content: None,
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

