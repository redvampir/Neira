use backend::node_template::{load_schema, load_schema_from, validate_template, NodeTemplate};
use serde_json::json;
use std::path::Path;

#[test]
fn valid_template_is_accepted() {
    let value = json!({
        "id": "valid-node",
        "analysis_type": "text",
        "links": ["a", "b"],
        "confidence_threshold": 0.5,
        "metadata": {
            "schema": "1.0.0",
            "author": "Bob",
            "tags": ["tag1", "tag2"],
            "version": "0.1.0"
        }
    });
    assert!(
        validate_template(&value).is_ok(),
        "schema validation should pass"
    );
    let template: NodeTemplate = serde_json::from_value(value).expect("deserialize");
    assert_eq!(
        template
            .metadata
            .extra
            .get("author")
            .and_then(|v| v.as_str()),
        Some("Bob")
    );
    assert_eq!(
        template.metadata.extra.get("tags"),
        Some(&json!(["tag1", "tag2"]))
    );
    assert_eq!(
        template
            .metadata
            .extra
            .get("version")
            .and_then(|v| v.as_str()),
        Some("0.1.0")
    );
}

#[test]
fn missing_required_fields_are_rejected() {
    let schema = load_schema("1.0.0").expect("load schema");
    let value = json!({
        "links": [],
        "metadata": {}
    });
    assert!(
        schema.validate(&value).is_err(),
        "schema validation should fail"
    );
    assert!(
        serde_json::from_value::<NodeTemplate>(value).is_err(),
        "deserialization should fail"
    );
}

#[test]
fn invalid_links_type_fails() {
    let schema = load_schema("1.0.0").expect("load schema");
    let value = json!({
        "id": "node",
        "analysis_type": "text",
        "links": "not-an-array",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_err(),
        "schema validation should fail"
    );
    assert!(
        serde_json::from_value::<NodeTemplate>(value).is_err(),
        "deserialization should fail"
    );
}

#[test]
fn invalid_confidence_threshold_type_fails() {
    let schema = load_schema("1.0.0").expect("load schema");
    let value = json!({
        "id": "node",
        "analysis_type": "text",
        "confidence_threshold": "high",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_err(),
        "schema validation should fail"
    );
    assert!(
        serde_json::from_value::<NodeTemplate>(value).is_err(),
        "deserialization should fail"
    );
}

#[test]
fn empty_id_is_handled() {
    let schema = load_schema("1.0.0").expect("load schema");
    let value = json!({
        "id": "",
        "analysis_type": "text",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_ok(),
        "schema validation should pass for empty id"
    );
    let template: NodeTemplate = serde_json::from_value(value).expect("deserialize");
    assert!(template.id.is_empty());
}

#[test]
fn explicit_path_loading_works() {
    let schema =
        load_schema_from(Path::new("schemas/node-template/v1.0.0.json")).expect("load schema");
    let value = json!({
        "id": "explicit",
        "analysis_type": "text",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_ok(),
        "schema validation should pass"
    );
}

#[test]
fn unknown_schema_version_errors() {
    assert!(
        load_schema("9.9.9").is_err(),
        "loading unknown version should fail"
    );
    let value = json!({
        "id": "node",
        "analysis_type": "text",
        "metadata": {"schema": "9.9.9"}
    });
    assert!(
        validate_template(&value).is_err(),
        "validation should fail for unknown schema version"
    );
}
