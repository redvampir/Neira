use backend::node_template::{validate_template, NodeTemplate};
use serde_json::json;

#[test]
fn valid_template_passes_validation() {
    let value = json!({
        "id": "example-node",
        "analysis_type": "text",
        "metadata": {"schema": "1.0"}
    });
    validate_template(&value).expect("should be valid");
    let _template: NodeTemplate = serde_json::from_value(value).expect("deserialize");
}

#[test]
fn invalid_template_reports_errors() {
    let value = json!({
        "analysis_type": "text",
        "metadata": {}
    });
    match validate_template(&value) {
        Ok(_) => panic!("expected validation errors"),
        Err(errors) => {
            assert!(!errors.is_empty());
            println!("Ошибки: {errors:?}");
        }
    }
}
