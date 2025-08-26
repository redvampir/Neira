use backend::node_template::{load_schema_from, NodeTemplate};
use serde_json::json;
use std::path::Path;

fn main() {
    let example = json!({
        "id": "example-node",
        "analysis_type": "text",
        "links": ["node-a", "node-b"],
        "confidence_threshold": 0.75,
        "draft_content": "draft",
        "metadata": { "schema": "1.0", "author": "Alice" }
    });

    let schema = load_schema_from(Path::new("schemas/node-template.schema.json"));
    let validation = schema.validate(&example);
    match validation {
        Ok(_) => {
            let template: NodeTemplate = serde_json::from_value(example.clone()).expect("deserialize");
            println!("{:?}", template);
        }
        Err(errors) => {
            for error in errors {
                eprintln!("Ошибка проверки: {error}");
            }
        }
    }
}
