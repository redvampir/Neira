/* neira:meta
id: NEI-20250602-150100-examplecell
intent: docs
summary: |
  Переименован тип анализа в JSON-примере на ExampleCell.
*/
use backend::cell_template::{validate_template, CellTemplate};
use serde_json::json;

fn main() {
    // Example template in JSON form
    let value = json!({
        "id": "example.template",
        "analysis_type": "ExampleCell",
        "metadata": {
            "schema": "1.0.0",
            "author": "Example"
        }
    });

    // Validate JSON against the schema and deserialize
    validate_template(&value).expect("template should validate");
    let template: CellTemplate = serde_json::from_value(value).expect("deserialize CellTemplate");
    println!("Loaded template id: {}", template.id);
}
