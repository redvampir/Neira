use backend::node_template::{load_schema_from, NodeTemplate};
use serde_json::json;
use std::path::Path;

#[test]
fn example_node_template_validates() {
    let example = json!({
        "id": "example-node",
        "analysis_type": "text",
        "links": ["node-a", "node-b"],
        "confidence_threshold": 0.75,
        "draft_content": "draft",
        "metadata": { "schema": "1.0.0", "author": "Alice" }
    });

    let schema = load_schema_from(Path::new("schemas/node-template/v1.0.0.json")).expect("load schema");
    assert!(schema.validate(&example).is_ok(), "schema validation should pass");
    let _template: NodeTemplate = serde_json::from_value(example).expect("deserialize");
}
