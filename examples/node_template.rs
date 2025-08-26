use backend::node_template::validate;
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

    match validate(&example) {
        Ok(template) => println!("{}", template.to_json()),
        Err(errors) => {
            for error in errors {
                eprintln!("Ошибка проверки: {error}");
            }
        }
    }
}
