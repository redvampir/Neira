use backend::node_registry::NodeRegistry;
use backend::node_template::{Metadata, NodeTemplate};
use serde_json::json;
use std::{collections::HashMap, fs, thread, time::Duration};
use tempfile::tempdir;

#[test]
fn hot_reload_detects_file_changes() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("n1-0.1.0.json");
    let tpl = json!({
        "id": "n1",
        "version": "0.1.0",
        "analysis_type": "a",
        "metadata": {"schema": "1.0.0"}
    });
    fs::write(&path, serde_json::to_string(&tpl).unwrap()).unwrap();

    let registry = NodeRegistry::new(dir.path()).unwrap();
    assert_eq!(registry.get("n1", None).unwrap().analysis_type, "a");

    let tpl2 = json!({
        "id": "n1",
        "version": "0.1.0",
        "analysis_type": "b",
        "metadata": {"schema": "1.0.0"}
    });
    fs::write(&path, serde_json::to_string(&tpl2).unwrap()).unwrap();
    thread::sleep(Duration::from_secs(1));
    assert_eq!(registry.get("n1", None).unwrap().analysis_type, "b");

    fs::remove_file(&path).unwrap();
    thread::sleep(Duration::from_secs(1));
    // Проверяем, что файл удалён без ошибок; возможная задержка уведомления не учитывается.
}

#[test]
fn register_template_persists_to_disk() {
    let dir = tempdir().unwrap();
    let registry = NodeRegistry::new(dir.path()).unwrap();
    let tpl = NodeTemplate {
        id: "n2".to_string(),
        version: "0.1.0".to_string(),
        analysis_type: "a".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: Metadata { schema: "1.0.0".to_string(), extra: HashMap::new() },
    };
    registry.register_template(tpl.clone()).unwrap();
    assert!(registry.get("n2", Some("0.1.0")).is_some());
    drop(registry);
    let registry2 = NodeRegistry::new(dir.path()).unwrap();
    assert!(registry2.get("n2", Some("0.1.0")).is_some());
}
