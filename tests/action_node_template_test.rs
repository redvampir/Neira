/* neira:meta
id: NEI-20250323-151200-action-template-list
intent: test
summary: Проверяет регистрацию и перечисление шаблонов узлов действия.
*/
use backend::node_registry::NodeRegistry;
use backend::node_template::ActionNodeTemplate;
use std::collections::HashSet;

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

#[test]
fn registry_lists_action_templates() {
    let dir = tempfile::tempdir().unwrap();
    let registry = NodeRegistry::new(dir.path()).unwrap();
    let tpl1 = ActionNodeTemplate {
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
    let tpl2 = ActionNodeTemplate {
        id: "action.another.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "another".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::node_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    registry.register_action_template(tpl1).unwrap();
    registry.register_action_template(tpl2).unwrap();
    let listed = registry.list_action_templates();
    let ids: HashSet<_> = listed.into_iter().map(|t| t.id).collect();
    assert!(ids.contains("action.example.v1"));
    assert!(ids.contains("action.another.v1"));
}

/* neira:meta
id: NEI-20250418-duplicate-action-template
intent: test
summary: |-
  Проверяет, что повторная регистрация шаблона с тем же id возвращает ошибку.
*/
#[test]
fn duplicate_action_template_returns_error() {
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
    registry.register_action_template(tpl.clone()).unwrap();
    assert!(registry.register_action_template(tpl).is_err());
}
