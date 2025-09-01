use backend::node_registry::NodeRegistry;
use backend::node_template::ActionNodeTemplate;

#[test]
fn registry_registers_action_templates() {
    let dir = tempfile::tempdir().unwrap();
    let registry = NodeRegistry::new(dir.path()).unwrap();
    let tpl = ActionNodeTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::node_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    registry.register_action_template(tpl).unwrap();
    assert!(registry.get_action_template("action.example.v1").is_some());
}
