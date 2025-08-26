use backend::node_template::{validate_template, NodeTemplate};
use serde_json::json;

fn main() {
    let example = json!({
        "id": "example-node",
        "analysis_type": "text",
        "links": ["node-a", "node-b"],
        "confidence_threshold": 0.75,
        "draft_content": "draft",
        "metadata": { "schema": "1.0", "author": "Alice" }
    });

    let template: NodeTemplate = serde_json::from_value(example.clone()).expect("deserialize");
    match validate_template(&example) {
        Ok(_) => println!("{:?}", template),
        Err(errors) => {
            for error in errors {
                eprintln!("Ошибка проверки: {error}");
            }
        }
    }
}
