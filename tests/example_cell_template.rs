use backend::cell_template::{load_schema_from, CellTemplate};
use serde_json::json;
use std::path::Path;

#[test]
fn example_cell_template_validates() {
    let example = json!({
        "id": "example-cell",
        "version": "0.1.0",
        "analysis_type": "text",
        "links": ["cell-a", "cell-b"],
        "confidence_threshold": 0.75,
        "draft_content": "draft",
        "metadata": {
            "schema": "1.0.0",
            "author": "Alice",
            "tags": ["demo", "test"],
            "version": "0.1.0"
        }
    });

    let schema =
        load_schema_from(Path::new("schemas/v1/cell-template.schema.json")).expect("load schema");
    assert!(
        schema.validate(&example).is_ok(),
        "schema validation should pass"
    );
    let template: CellTemplate = serde_json::from_value(example).expect("deserialize");
    assert_eq!(
        template
            .metadata
            .extra
            .get("author")
            .and_then(|v| v.as_str()),
        Some("Alice")
    );
    assert_eq!(
        template.metadata.extra.get("tags"),
        Some(&json!(["demo", "test"]))
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
