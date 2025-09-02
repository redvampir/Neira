use backend::cell_template::{validate_template, CellTemplate};
use serde_json::json;

#[test]
fn valid_template_passes_validation() {
    let value = json!({
        "id": "example-node",
        "version": "0.1.0",
        "analysis_type": "text",
        "metadata": {
            "schema": "1.0.0",
            "author": "Carol",
            "tags": ["demo"],
            "version": "0.1.0"
        }
    });
    validate_template(&value).expect("should be valid");
    let template: CellTemplate = serde_json::from_value(value).expect("deserialize");
    assert!(template.metadata.extra.contains_key("author"));
    assert!(template.metadata.extra.contains_key("tags"));
    assert!(template.metadata.extra.contains_key("version"));
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
