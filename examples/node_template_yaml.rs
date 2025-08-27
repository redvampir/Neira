use std::fs::File;

use backend::node_template::{validate_template, NodeTemplate};
use serde_yaml::Value as YamlValue;

fn main() {
    // Read the YAML template from a file.
    let file = File::open("examples/node_template.yaml").expect("open YAML file");
    let yaml: YamlValue = serde_yaml::from_reader(file).expect("read YAML");

    // Convert YAML value to JSON value.
    let value = serde_json::to_value(yaml).expect("convert to JSON");

    // Validate JSON against the schema and deserialize.
    validate_template(&value).expect("template should validate");
    let template: NodeTemplate = serde_json::from_value(value).expect("deserialize NodeTemplate");
    println!("Loaded template id: {}", template.id);
}
