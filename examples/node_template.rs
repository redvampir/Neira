use backend::node_template::{load_schema, validate_template, NodeTemplate};
use serde_json::json;

fn main() {
    // Load schema to ensure it exists
    let _schema = load_schema().expect("schema should load");

    // Example template in JSON form
    let value = json!({
        "id": "example.template",
        "analysis_type": "ExampleNode",
        "metadata": {
            "schema": "1.0.0",
            "author": "Example"
        }
    });

    // Validate JSON against the schema and deserialize
    validate_template(&value).expect("template should validate");
    let template: NodeTemplate = serde_json::from_value(value).expect("deserialize NodeTemplate");
    println!("Loaded template id: {}", template.id);
}
