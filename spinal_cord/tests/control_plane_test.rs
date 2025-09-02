/* neira:meta
id: NEI-20250214-control-plane-startup-wait
intent: test
summary: усилили ожидание запуска бэкенда и фиксируем ранний выход процесса.
*/
/* neira:meta
id: NEI-20260413-control-plane-rename
intent: test
summary: Путь CONTROL_SNAPSHOT_DIR обновлён на spinal_cord/snapshots_test.
*/
use std::process::{Command, Stdio};
use std::time::Duration;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pause_resume_snapshot_kill() {
    // choose a test port
    let port = 3011;
    let base = format!("http://127.0.0.1:{}", port);

    // path to compiled binary is provided by Cargo as env var in tests
    let bin = option_env!("CARGO_BIN_EXE_backend").unwrap_or("target/debug/backend");

    let mut child = Command::new(bin)
        .env("NEIRA_ADMIN_TOKEN", "admin123")
        .env("NERVOUS_SYSTEM_ENABLED", "true")
        .env("CONTROL_SNAPSHOT_DIR", "spinal_cord/snapshots_test")
        .env("CONTROL_ALLOW_KILL", "true")
        .env("CONTROL_ALLOW_PAUSE", "true")
        .env("NEIRA_BIND_ADDR", format!("127.0.0.1:{}", port))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("spawn backend");

    // wait for server
    let client = reqwest::Client::new();
    let mut ok = false;
    for _ in 0..100 {
        if let Ok(resp) = client.get(format!("{}/", base)).send().await {
            if resp.status().as_u16() == 200 {
                ok = true;
                break;
            }
        }
        if let Some(status) = child.try_wait().expect("child wait") {
            panic!("backend exited early: {status}");
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    assert!(ok, "server did not start");

    // pause
    let pause = client
        .post(format!("{}/api/neira/control/pause", base))
        .json(&serde_json::json!({"auth":"admin123","reason":"maint","request_id":"r1"}))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    assert_eq!(pause.get("paused").and_then(|v| v.as_bool()), Some(true));

    // chat should be 503
    let resp = client.post(format!("{}/api/neira/chat", base))
        .json(&serde_json::json!({"cell_id":"echo.chat","chat_id":"t","message":"hello","auth":"admin123","persist":false}))
        .send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 503);

    // resume
    let resume = client
        .post(format!("{}/api/neira/control/resume", base))
        .json(&serde_json::json!({"auth":"admin123","request_id":"r2"}))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    assert_eq!(resume.get("paused").and_then(|v| v.as_bool()), Some(false));

    // chat works
    let chat = client.post(format!("{}/api/neira/chat", base))
        .json(&serde_json::json!({"cell_id":"echo.chat","chat_id":"t","message":"hello world","auth":"admin123","persist":false}))
        .send().await.unwrap()
        .json::<serde_json::Value>().await.unwrap();
    assert!(chat.get("response").is_some());

    // snapshot
    let snap = client
        .get(format!(
            "{}/api/neira/inspect/snapshot?include=metrics,context",
            base
        ))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    let file = snap.get("file").and_then(|v| v.as_str()).unwrap();
    assert!(std::path::Path::new(file).exists());

    // kill
    let _ = client
        .post(format!("{}/api/neira/control/kill", base))
        .json(&serde_json::json!({"auth":"admin123","grace_ms": 500, "request_id":"r3"}))
        .send()
        .await
        .unwrap();

    // wait for exit
    let mut exited = false;
    for _ in 0..20 {
        if let Some(status) = child.try_wait().unwrap() {
            exited = status.success() || !status.success();
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    assert!(exited, "backend did not exit after kill");
}
