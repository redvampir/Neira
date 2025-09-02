use backend::cell_template::{load_schema_from, validate_template, CellTemplate};
use serde_json::json;
use std::path::Path;

#[test]
fn valid_template_is_accepted() {
    let value = json!({
        "id": "valid-cell",
        "version": "0.1.0",
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
    let template: CellTemplate = serde_json::from_value(value).expect("deserialize");
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
    let schema = load_schema_from(std::path::Path::new("schemas/v1/cell-template.schema.json"))
        .expect("load schema");
    let value = json!({
        "links": [],
        "metadata": {}
    });
    assert!(
        schema.validate(&value).is_err(),
        "schema validation should fail"
    );
    assert!(
        serde_json::from_value::<CellTemplate>(value).is_err(),
        "deserialization should fail"
    );
}

#[test]
fn invalid_links_type_fails() {
    let schema = load_schema_from(std::path::Path::new("schemas/v1/cell-template.schema.json"))
        .expect("load schema");
    let value = json!({
        "id": "cell",
        "version": "0.1.0",
        "analysis_type": "text",
        "links": "not-an-array",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_err(),
        "schema validation should fail"
    );
    assert!(
        serde_json::from_value::<CellTemplate>(value).is_err(),
        "deserialization should fail"
    );
}

#[test]
fn invalid_confidence_threshold_type_fails() {
    let schema = load_schema_from(std::path::Path::new("schemas/v1/cell-template.schema.json"))
        .expect("load schema");
    let value = json!({
        "id": "cell",
        "version": "0.1.0",
        "analysis_type": "text",
        "confidence_threshold": "high",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_err(),
        "schema validation should fail"
    );
    assert!(
        serde_json::from_value::<CellTemplate>(value).is_err(),
        "deserialization should fail"
    );
}

#[test]
fn empty_id_is_handled() {
    let schema = load_schema_from(std::path::Path::new("schemas/v1/cell-template.schema.json"))
        .expect("load schema");
    let value = json!({
        "id": "",
        "version": "0.1.0",
        "analysis_type": "text",
        "metadata": {"schema": "1.0.0"}
    });
    assert!(
        schema.validate(&value).is_ok(),
        "schema validation should pass for empty id"
    );
    let template: CellTemplate = serde_json::from_value(value).expect("deserialize");
    assert!(template.id.is_empty());
}

#[test]
fn explicit_path_loading_works() {
    let schema =
        load_schema_from(Path::new("schemas/v1/cell-template.schema.json")).expect("load schema");
    let value = json!({
        "id": "explicit",
        "version": "0.1.0",
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
    let value = json!({
        "id": "cell",
        "version": "0.1.0",
        "analysis_type": "text",
        "metadata": {"schema": "9.9.9"}
    });
    assert!(
        validate_template(&value).is_err(),
        "validation should fail for unknown schema version"
    );
}
