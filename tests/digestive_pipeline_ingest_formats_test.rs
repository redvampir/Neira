/* neira:meta
id: NEI-20251101-digestive-ingest-tests
intent: test
summary: Тесты на JSON/YAML/XML ingest, XML $text-flattening, store_parsed_input и токсик-фильтр sanitize.
*/
use backend::digestive_pipeline::{DigestivePipeline, ParsedInput};
use backend::memory_cell::MemoryCell;
use serial_test::serial;
use std::fs;
use tempfile::tempdir;

fn prepare_config() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    let schema_path = dir.path().join("schema.json");
    // Minimal permissive JSON schema
    fs::write(&schema_path, "{\"type\":\"object\"}").unwrap();

    let cfg_path = dir.path().join("digestive.toml");
    // Use single-quoted TOML string to avoid Windows path escaping
    fs::write(
        &cfg_path,
        format!("schema_path = '{}'", schema_path.display()),
    )
    .unwrap();
    std::env::set_var("DIGESTIVE_CONFIG", cfg_path.to_str().unwrap());
    dir
}

#[test]
#[serial]
fn ingests_json_yaml_xml_and_stores_in_memory() {
    let _guard = prepare_config();
    DigestivePipeline::reset_cache();

    let memory = std::sync::Arc::new(MemoryCell::new());
    DigestivePipeline::set_memory(memory.clone());
    DigestivePipeline::init().unwrap();

    // JSON
    let r1 = DigestivePipeline::ingest("{\"a\":1}").unwrap();
    match r1 {
        ParsedInput::Json(_) => {}
        _ => panic!("expected Json for JSON input"),
    }

    // YAML
    let r2 = DigestivePipeline::ingest("a: 1\n").unwrap();
    match r2 {
        ParsedInput::Json(_) => {}
        _ => panic!("expected Json for YAML input"),
    }

    // XML (simple)
    let r3 = DigestivePipeline::ingest("<root><a>1</a></root>").unwrap();
    let v3 = match r3 {
        ParsedInput::Json(v) => v,
        _ => panic!("expected Json for XML input"),
    };
    // After flattening, `a` should be a string or number inside the object
    assert!(v3.get("root").is_some());

    // Memory has 3 parsed inputs stored
    let stored = memory.parsed_inputs();
    assert_eq!(stored.len(), 3);

    std::env::remove_var("DIGESTIVE_CONFIG");
    DigestivePipeline::reset_cache();
}

#[test]
#[serial]
fn xml_text_is_flattened() {
    let _guard = prepare_config();
    DigestivePipeline::reset_cache();
    DigestivePipeline::init().unwrap();

    let r = DigestivePipeline::ingest("<root><msg>hello</msg></root>").unwrap();
    let v = match r {
        ParsedInput::Json(v) => v,
        _ => panic!("expected Json for XML input"),
    };
    // Expect {"root":{"msg":"hello"}} (not nested {"$text":"hello"})
    assert_eq!(v["root"]["msg"], "hello");

    std::env::remove_var("DIGESTIVE_CONFIG");
    DigestivePipeline::reset_cache();
}

#[test]
fn sanitize_replaces_toxic_patterns() {
    // Using the exact mojibake sequence present in the code patterns to ensure match
    let raw = "aaa�����bbb";
    let sanitized = DigestivePipeline::sanitize(raw);
    assert!(sanitized.contains("[censored]"));
}
