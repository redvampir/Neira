use std::fs::File;

use backend::cell_template::{validate_template, CellTemplate};
use serde_yaml::Value as YamlValue;

fn main() {
    // Read the YAML template from a file.
    let file = File::open("examples/cell_template.yaml").expect("open YAML file");
    let yaml: YamlValue = serde_yaml::from_reader(file).expect("read YAML");

    // Convert YAML value to JSON value.
    let value = serde_json::to_value(yaml).expect("convert to JSON");

    // Validate JSON against the schema and deserialize.
    validate_template(&value).expect("template should validate");
    let template: CellTemplate = serde_json::from_value(value).expect("deserialize CellTemplate");
    println!("Loaded template id: {}", template.id);
}
