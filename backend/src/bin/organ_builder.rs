/* neira:meta
id: NEI-20250214-organ-builder-cli
intent: code
summary: CLI для управления сборкой органов: build, status и cancel.
*/
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(err) = run().await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let cmd = args.next().ok_or_else(usage)?;
    match cmd.as_str() {
        "build" => {
            let path = args.next().ok_or_else(usage)?;
            let base = args.next().unwrap_or_else(default_base);
            let content =
                fs::read_to_string(&path).map_err(|e| format!("failed to read {path}: {e}"))?;
            let template: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| format!("invalid JSON: {e}"))?;
            let body = serde_json::json!({"organ_template": template});
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("{base}/organs/build"))
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("request failed: {e}"))?;
            if resp.status().is_success() {
                let text = resp
                    .text()
                    .await
                    .map_err(|e| format!("response read failed: {e}"))?;
                println!("{text}");
            } else {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_else(|_| String::new());
                return Err(format!("status {status}: {text}"));
            }
        }
        "status" => {
            let id = args.next().ok_or_else(usage)?;
            let base = args.next().unwrap_or_else(default_base);
            let client = reqwest::Client::new();
            let resp = client
                .get(format!("{base}/organs/{id}/status"))
                .send()
                .await
                .map_err(|e| format!("request failed: {e}"))?;
            if resp.status().is_success() {
                let text = resp
                    .text()
                    .await
                    .map_err(|e| format!("response read failed: {e}"))?;
                println!("{text}");
            } else {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_else(|_| String::new());
                return Err(format!("status {status}: {text}"));
            }
        }
        "cancel" => {
            let id = args.next().ok_or_else(usage)?;
            let base = args.next().unwrap_or_else(default_base);
            let client = reqwest::Client::new();
            let resp = client
                .delete(format!("{base}/organs/{id}/build"))
                .send()
                .await
                .map_err(|e| format!("request failed: {e}"))?;
            if resp.status().is_success() {
                println!("cancelled");
            } else {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_else(|_| String::new());
                return Err(format!("status {status}: {text}"));
            }
        }
        other => return Err(format!("unknown command: {other}")),
    }
    Ok(())
}

fn usage() -> String {
    "usage: cargo run -p backend --bin organ_builder -- <command> <arg> [base_url]".to_string()
}

fn default_base() -> String {
    env::var("NEIRA_API_BASE").unwrap_or_else(|_| "http://127.0.0.1:3000".into())
}
