/* neira:meta
id: NEI-20251010-organ-builder-test
intent: test
summary: Проверяет переходы стадий органа, ручное обновление, удержание статуса `Failed` и очистку шаблонов по TTL.
*/
use std::path::PathBuf;

use backend::organ_builder::{OrganBuilder, OrganState};

#[tokio::test]
async fn organ_builder_progresses_and_updates() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    let dir = PathBuf::from("backend/tests/tmp_organs");
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.to_str().unwrap());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_eq!(builder.status(&id), Some(OrganState::Stable));
    builder.update_status(&id, OrganState::Failed);
    assert_eq!(builder.status(&id), Some(OrganState::Failed));
    assert!(dir.join(format!("{id}.json")).exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn organ_builder_failure_persists() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    let dir = PathBuf::from("backend/tests/tmp_organs");
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.to_str().unwrap());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    builder.update_status(&id, OrganState::Failed);
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_eq!(builder.status(&id), Some(OrganState::Failed));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn organ_builder_removes_template_after_ttl() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    std::env::set_var("ORGANS_BUILDER_TTL_SECS", "1");
    let dir = PathBuf::from("backend/tests/tmp_organs");
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.to_str().unwrap());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert!(dir.join(format!("{id}.json")).exists());
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    assert!(!dir.join(format!("{id}.json")).exists());
    assert_eq!(builder.status(&id), Some(OrganState::Stable));
    let _ = std::fs::remove_dir_all(&dir);
}
