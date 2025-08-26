use backend::node_template::{load_schema, validate};
use serde_json::json;
use std::env;

#[test]
fn valid_node_template() {
    let value = json!({
        "id": "example-node",
        "analysis_type": "text",
        "links": ["node-a"],
        "confidence_threshold": 0.5,
        "draft_content": "draft",
        "metadata": {"schema": "1.0", "author": "Bob"}
    });
    let template = validate(&value).expect("validation should pass");
    assert_eq!(template.id, "example-node");
    let roundtrip = template.to_json();
    assert_eq!(roundtrip, value);
}

#[test]
fn invalid_node_template() {
    let value = json!({
        "analysis_type": "text",
        "metadata": {"schema": "1.0"}
    });
    let errors = validate(&value).expect_err("validation should fail");
    assert!(!errors.is_empty());
}

#[test]
fn schema_is_cached() {
    let first = load_schema() as *const _;
    let second = load_schema() as *const _;
    assert_eq!(first, second);
}

#[test]
fn schema_version_env() {
    env::set_var("NODE_TEMPLATE_SCHEMA_VERSION", "1.0");
    let schema = load_schema();
    assert!(schema
        .validate(&json!({
            "id": "v1",
            "analysis_type": "text",
            "metadata": {"schema": "1.0"}
        }))
        .is_ok());
    env::remove_var("NODE_TEMPLATE_SCHEMA_VERSION");
}
