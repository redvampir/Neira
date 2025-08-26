use backend::node_template::{NodeTemplate, load_schema};
use serde_json::json;

#[test]
fn valid_template_is_accepted() {
    let schema = load_schema();
    let value = json!({
        "id": "valid-node",
        "analysis_type": "text",
        "links": ["a", "b"],
        "confidence_threshold": 0.5,
        "metadata": {"schema": "1.0"}
    });
    assert!(schema.validate(&value).is_ok(), "schema validation should pass");
    let _template: NodeTemplate = serde_json::from_value(value).expect("deserialize");
}

#[test]
fn missing_required_fields_are_rejected() {
    let schema = load_schema();
    let value = json!({
        "links": [],
        "metadata": {}
    });
    assert!(schema.validate(&value).is_err(), "schema validation should fail");
    assert!(serde_json::from_value::<NodeTemplate>(value).is_err(), "deserialization should fail");
}

#[test]
fn invalid_links_type_fails() {
    let schema = load_schema();
    let value = json!({
        "id": "node",
        "analysis_type": "text",
        "links": "not-an-array",
        "metadata": {"schema": "1.0"}
    });
    assert!(schema.validate(&value).is_err(), "schema validation should fail");
    assert!(serde_json::from_value::<NodeTemplate>(value).is_err(), "deserialization should fail");
}

#[test]
fn invalid_confidence_threshold_type_fails() {
    let schema = load_schema();
    let value = json!({
        "id": "node",
        "analysis_type": "text",
        "confidence_threshold": "high",
        "metadata": {"schema": "1.0"}
    });
    assert!(schema.validate(&value).is_err(), "schema validation should fail");
    assert!(serde_json::from_value::<NodeTemplate>(value).is_err(), "deserialization should fail");
}

#[test]
fn empty_id_is_handled() {
    let schema = load_schema();
    let value = json!({
        "id": "",
        "analysis_type": "text",
        "metadata": {"schema": "1.0"}
    });
    assert!(schema.validate(&value).is_ok(), "schema validation should pass for empty id");
    let template: NodeTemplate = serde_json::from_value(value).expect("deserialize");
    assert!(template.id.is_empty());
}
