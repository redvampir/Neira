use std::{fs, path::Path, process::Command};

use backend::node_template::validate_template;
use serde_json::Value;

fn main() {
    // Run the generator to produce a template JSON.
    let output = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            "backend/Cargo.toml",
            "--bin",
            "generate_node",
            "--quiet",
            "--",
            "--schema",
            "v1",
        ])
        .output()
        .expect("failed to run generator");
    assert!(output.status.success(), "generator failed");

    // Parse the generated JSON and remove `null` fields.
    let mut value: Value = serde_json::from_slice(&output.stdout).expect("parse json");
    if let Some(obj) = value.as_object_mut() {
        obj.retain(|_, v| !v.is_null());
    }

    // Validate against the schema and save to a file.
    validate_template(&value).expect("template should validate");
    let path = Path::new("node-template.json");
    fs::write(path, serde_json::to_string_pretty(&value).unwrap()).expect("write file");

    println!("Generated and validated {}", path.display());
}
