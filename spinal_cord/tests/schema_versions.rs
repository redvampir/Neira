use backend::cell_template::validate_template;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

#[test]
fn load_multiple_schema_versions() {
    // Prepare temporary directory with two schema versions
    let tmp = tempfile::tempdir().expect("tempdir");
    let base = tmp.path();
    let original =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas/v1/cell-template.schema.json");
    for ver in &["v1", "v2"] {
        let dir = base.join(ver);
        fs::create_dir_all(&dir).expect("create version dir");
        let content = fs::read_to_string(&original).expect("read schema");
        fs::write(dir.join("cell-template.schema.json"), &content).expect("write schema");
    }
    std::env::set_var("CELL_TEMPLATE_SCHEMAS_DIR", base);

    let v1 = json!({
        "id": "n1",
        "version": "0.1.0",
        "analysis_type": "a",
        "metadata": {"schema": "1.0.0"}
    });
    validate_template(&v1).expect("v1 schema should load");

    let v2 = json!({
        "id": "n2",
        "version": "0.1.0",
        "analysis_type": "a",
        "metadata": {"schema": "2.0.0"}
    });
    validate_template(&v2).expect("v2 schema should load");
}
