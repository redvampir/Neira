/* neira:meta
id: NEI-20270520-action-engine-test
intent: test
summary: Проверяет работу ActionEngine и контроль прав.
*/
use backend::action_engine::{ActionCommand, ActionEngine, ActionError};
use serial_test::serial;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
#[serial]
async fn file_read_executes() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "hi").unwrap();
    let path = tmp.path().to_string_lossy().to_string();
    let engine = ActionEngine::new();
    let cmd = ActionCommand::ReadFile { path };
    let res = engine.execute(cmd).await.unwrap();
    assert!(res.contains("hi"));
}

#[tokio::test]
#[serial]
async fn system_command_denied_without_env() {
    std::env::remove_var("NEIRA_ALLOW_SYSTEM");
    let engine = ActionEngine::new();
    let cmd = ActionCommand::RunCommand {
        program: "echo".into(),
        args: vec!["ok".into()],
    };
    let err = engine.execute(cmd).await.unwrap_err();
    matches!(err, ActionError::Security(_));
}

#[tokio::test]
#[serial]
async fn system_command_allowed_with_env() {
    std::env::set_var("NEIRA_ALLOW_SYSTEM", "1");
    let engine = ActionEngine::new();
    let cmd = ActionCommand::RunCommand {
        program: "echo".into(),
        args: vec!["ok".into()],
    };
    let res = engine.execute(cmd).await.unwrap();
    assert_eq!(res, "ok");
}
