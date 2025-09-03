/* neira:meta
id: NEI-20260725-digestive-config-test
intent: test
summary: Проверяет, что DigestivePipeline использует путь схемы из конфигурации.
*/
use backend::digestive_pipeline::{DigestivePipeline, PipelineError};
use tempfile::tempdir;

#[test]
fn validates_with_overridden_schema() {
    let dir = tempdir().expect("temp dir");
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "required": ["extra"]
    }"#;
    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, schema).expect("write schema");

    let cfg_content = format!("schema_path = \"{}\"", schema_path.display());
    let cfg_path = dir.path().join("digestive.toml");
    std::fs::write(&cfg_path, cfg_content).expect("write config");
    std::env::set_var("DIGESTIVE_CONFIG", &cfg_path);

    DigestivePipeline::init().expect("init");
    let raw = "{\"id\":1}";
    let err = DigestivePipeline::ingest(raw).expect_err("should fail");
    assert!(matches!(err, PipelineError::Validation(_)));
}
