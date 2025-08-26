use std::env;
use std::fs;
use std::path::PathBuf;

use backend::node_template;
use serde_json::Value;

fn main() {
    tracing_subscriber::fmt::init();
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| "usage: cargo run --bin validate_template <path>".to_string())?;
    let path = PathBuf::from(path);
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let value: Value = match path.extension().and_then(|e| e.to_str()) {
        Some("yaml") | Some("yml") => {
            serde_yaml::from_str(&content).map_err(|e| format!("invalid YAML: {e}"))?
        }
        _ => serde_json::from_str(&content).map_err(|e| format!("invalid JSON: {e}"))?,
    };
    node_template::validate_template(&value).map_err(|errs| errs.join("\n"))?;
    println!("Template is valid");
    Ok(())
}
