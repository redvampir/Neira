/* neira:meta
id: NEI-20250323-151200-action-template-list
intent: test
summary: Проверяет регистрацию и перечисление шаблонов узлов действия.
*/
use backend::cell_registry::CellRegistry;
use backend::cell_template::{ActionCellTemplate, CellTemplate};
use std::collections::HashSet;
use std::fs;

#[test]
fn registry_registers_action_templates() {
    let dir = tempfile::tempdir().unwrap();
    let registry = CellRegistry::new(dir.path()).unwrap();
    let tpl = ActionCellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
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
    let registry = CellRegistry::new(dir.path()).unwrap();
    let tpl1 = ActionCellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    let tpl2 = ActionCellTemplate {
        id: "action.another.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "another".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
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
id: NEI-20250501-reregister-same-path
intent: test
summary: Проверяет, что повторная регистрация по тому же пути заменяет шаблон.
*/
#[test]
fn reregister_same_path_replaces_template() {
    let dir = tempfile::tempdir().unwrap();
    let registry = CellRegistry::new(dir.path()).unwrap();
    let mut tpl = ActionCellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    registry.register_action_template(tpl.clone()).unwrap();
    tpl.action_type = "updated".to_string();
    registry.register_action_template(tpl).unwrap();
    let tpl = registry
        .get_action_template("action.example.v1")
        .expect("template exists");
    assert_eq!(tpl.action_type, "updated");
}

/* neira:meta
id: NEI-20250501-different-path-error
intent: test
summary: Проверяет, что повторная регистрация с другим путём возвращает ошибку.
*/
#[test]
fn registering_same_id_different_path_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let registry = CellRegistry::new(dir.path()).unwrap();
    let tpl1 = ActionCellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    let tpl2 = ActionCellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.2.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    registry.register_action_template(tpl1).unwrap();

    // перед повторной регистрацией в каталоге один файл
    let before_files = fs::read_dir(dir.path())
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(before_files.len(), 1);

    assert!(registry.register_action_template(tpl2).is_err());

    // после ошибки файл не создаётся и шаблон не изменяется
    let files = fs::read_dir(dir.path())
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(files.len(), 1);
    let tpl = registry
        .get_action_template("action.example.v1")
        .expect("template exists");
    assert_eq!(tpl.version, "0.1.0");
}

/* neira:meta
id: NEI-20250501-different-type-error
intent: test
summary: Проверяет, что повторная регистрация с другим типом шаблона запрещена.
*/
#[test]
fn registering_same_id_different_type_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let registry = CellRegistry::new(dir.path()).unwrap();
    let action_tpl = ActionCellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        action_type: "example".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    let cell_tpl = CellTemplate {
        id: "action.example.v1".to_string(),
        version: "0.1.0".to_string(),
        analysis_type: "analysis".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: backend::cell_template::Metadata {
            schema: "v1".to_string(),
            extra: Default::default(),
        },
    };
    registry.register_action_template(action_tpl).unwrap();
    assert!(registry.register_template(cell_tpl).is_err());
}
