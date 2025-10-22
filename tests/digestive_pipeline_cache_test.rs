/* neira:meta
id: NEI-20261015-digestive-cache-test
intent: test
summary: Проверяет, что DigestivePipeline читает JSON Schema с диска один раз.
*/
use backend::digestive_pipeline::DigestivePipeline;
use serial_test::serial;
use std::fs;
use tempfile::tempdir;

#[test]
#[serial]
fn caches_schema_after_first_read() {
    DigestivePipeline::reset_cache();

    let dir = tempdir().unwrap();
    let schema_path = dir.path().join("schema.json");
    fs::write(&schema_path, "{\"type\":\"object\"}").unwrap();

    let cfg_path = dir.path().join("digestive.toml");
    fs::write(
        &cfg_path,
        format!("schema_path = \"{}\"", schema_path.display()),
    )
    .unwrap();

    std::env::set_var("DIGESTIVE_CONFIG", cfg_path.to_str().unwrap());
    DigestivePipeline::init().unwrap();
    DigestivePipeline::ingest("{\"foo\":\"bar\"}").unwrap();

    fs::write(&schema_path, "not json").unwrap();
    DigestivePipeline::ingest("{\"foo\":\"baz\"}").unwrap();

    std::env::remove_var("DIGESTIVE_CONFIG");
    DigestivePipeline::reset_cache();
}
