use backend::node_template::validate_action_template;
use serde_json::json;

#[test]
fn valid_action_template_passes_validation() {
    let value = json!({
        "id": "action.example.v1",
        "version": "0.1.0",
        "action_type": "example",
        "metadata": { "schema": "v1" }
    });
    validate_action_template(&value).expect("valid action template");
}

#[test]
fn invalid_action_template_reports_errors() {
    let value = json!({
        "id": "broken"
    });
    assert!(validate_action_template(&value).is_err());
}
