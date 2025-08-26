use backend::node_template::{validate_template, Metadata, NodeTemplate};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn to_json_produces_valid_structure() {
    let mut extra = HashMap::new();
    extra.insert("author".to_string(), json!("Alice"));

    let template = NodeTemplate {
        id: "node-1".to_string(),
        analysis_type: "text".to_string(),
        links: vec!["a".to_string(), "b".to_string()],
        confidence_threshold: Some(0.8),
        draft_content: Some("draft".to_string()),
        metadata: Metadata {
            schema: "1.0.0".to_string(),
            extra,
        },
    };

    let value = template.to_json();
    validate_template(&value).expect("to_json result should validate");
}
