use backend::cell_template::{validate_template, CellTemplate, Metadata};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn generate_validate_deserialize_template() {
    let mut extra = HashMap::new();
    extra.insert("author".to_string(), json!("Alice"));

    let template = CellTemplate {
        id: "generated-cell".to_string(),
        version: "0.1.0".to_string(),
        analysis_type: "text".to_string(),
        links: vec!["a".to_string(), "b".to_string()],
        confidence_threshold: Some(0.5),
        draft_content: Some("draft".to_string()),
        metadata: Metadata {
            schema: "1.0.0".to_string(),
            extra,
        },
    };

    let value = template.to_json();
    validate_template(&value).expect("generated template should validate");
    let parsed: CellTemplate = serde_json::from_value(value).expect("deserialize CellTemplate");

    assert_eq!(parsed.id, template.id);
    assert_eq!(parsed.version, template.version);
    assert_eq!(parsed.analysis_type, template.analysis_type);
    assert_eq!(parsed.links, template.links);
    assert_eq!(parsed.confidence_threshold, template.confidence_threshold);
    assert_eq!(parsed.draft_content, template.draft_content);
    assert_eq!(parsed.metadata.schema, template.metadata.schema);
    assert_eq!(parsed.metadata.extra.get("author"), Some(&json!("Alice")));
}
