/* neira:meta
id: NEI-20250829-175425-scripted-training
intent: docs
scope: backend/action
summary: |
  Выполняет сценарии обучения; пути и режим задаются через переменные окружения.
env:
  - TRAINING_SCRIPT
  - TRAINING_PROGRESS
  - TRAINING_DRY_RUN
*/

use std::path::PathBuf;
use std::sync::Arc;

use crate::action_cell::ActionCell;
use crate::context::context_storage::ContextStorage;
use crate::memory_cell::MemoryCell;
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};
use tracing::{error, info};
// metrics integration can be wired via `metrics` crate if desired

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScriptStep {
    #[serde(default = "default_method")]
    method: String,
    url: String,
    #[serde(default)]
    body: Option<serde_json::Value>,
    #[serde(default)]
    headers: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    expect_status: Option<u16>,
    #[serde(default)]
    expect_contains: Option<String>,
    #[serde(default)]
    timeout_ms: Option<u64>,
    #[serde(default)]
    dataset: Option<Vec<serde_json::Value>>, // data-driven rows
    #[serde(default)]
    assertions: Option<Vec<Assertion>>, // rich checks
    #[serde(default)]
    pre_hook: Option<Hook>,
    #[serde(default)]
    post_hook: Option<Hook>,
    #[serde(default)]
    retry: Option<Retry>,
}

fn default_method() -> String {
    "GET".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScriptFile {
    name: String,
    #[serde(default)]
    description: Option<String>,
    steps: Vec<ScriptStep>,
    #[serde(default)]
    vars: Option<std::collections::HashMap<String, String>>, // script-level variables
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TrainProgress {
    script: String,
    last_completed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Retry {
    attempts: u32,
    backoff_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Hook {
    #[serde(rename = "sleep_ms")]
    Sleep { ms: u64 },
    #[serde(rename = "set_env")]
    SetEnv {
        vars: std::collections::HashMap<String, String>,
    },
    #[serde(rename = "shell")]
    Shell { cmd: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Assertion {
    path: String, // JSONPath
    #[serde(default)]
    equals: Option<serde_json::Value>,
    #[serde(default)]
    contains: Option<String>,
    #[serde(default)]
    gt: Option<f64>,
    #[serde(default)]
    lt: Option<f64>,
}

pub struct ScriptedTrainingCell {
    id: String,
    script_path: PathBuf,
    progress_path: PathBuf,
    dry_run: bool,
}

impl ScriptedTrainingCell {
    pub fn from_env() -> Self {
        let script = std::env::var("TRAINING_SCRIPT")
            .unwrap_or_else(|_| "examples/training_script.yaml".into());
        let progress = std::env::var("TRAINING_PROGRESS")
            .unwrap_or_else(|_| "context/training/progress.json".into());
        let dry_run = std::env::var("TRAINING_DRY_RUN")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Self {
            id: "scripted.training".into(),
            script_path: script.into(),
            progress_path: progress.into(),
            dry_run,
        }
    }

    async fn load_script(&self) -> Result<ScriptFile, String> {
        let content = std::fs::read_to_string(&self.script_path).map_err(|e| e.to_string())?;
        if self.script_path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&content).map_err(|e| e.to_string())
        } else {
            serde_yaml::from_str(&content).map_err(|e| e.to_string())
        }
    }

    fn load_progress(&self) -> TrainProgress {
        if let Ok(s) = std::fs::read_to_string(&self.progress_path) {
            serde_json::from_str(&s).unwrap_or_default()
        } else {
            TrainProgress {
                script: self.script_path.display().to_string(),
                last_completed: 0,
            }
        }
    }

    fn save_progress(&self, progress: &TrainProgress) {
        if let Some(dir) = self.progress_path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(s) = serde_json::to_string_pretty(progress) {
            let _ = std::fs::write(&self.progress_path, s);
        }
    }

    fn substitute_vars(input: &str, env: &std::collections::HashMap<String, String>) -> String {
        let mut out = String::new();
        let mut i = 0;
        let bytes = input.as_bytes();
        while i < bytes.len() {
            if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                if let Some(end) = input[i + 2..].find('}') {
                    let key = &input[i + 2..i + 2 + end];
                    // ${FILE:/path} or ${VAR_FILE:/path}
                    if let Some(path) = key
                        .strip_prefix("FILE:")
                        .or_else(|| key.strip_prefix("VAR_FILE:"))
                    {
                        match std::fs::read_to_string(path) {
                            Ok(mut s) => {
                                s.truncate(s.trim_end().len());
                                out.push_str(s.trim());
                            }
                            Err(_) => {}
                        }
                    } else {
                        out.push_str(env.get(key).map(|s| s.as_str()).unwrap_or(""));
                    }
                    i += end + 3; // skip ${...}
                    continue;
                }
            }
            out.push(bytes[i] as char);
            i += 1;
        }
        out
    }

    fn apply_vars_json(
        v: &serde_json::Value,
        env: &std::collections::HashMap<String, String>,
    ) -> serde_json::Value {
        match v {
            serde_json::Value::String(s) => {
                serde_json::Value::String(Self::substitute_vars(s, env))
            }
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.iter().map(|x| Self::apply_vars_json(x, env)).collect())
            }
            serde_json::Value::Object(m) => {
                let mut o = serde_json::Map::new();
                for (k, v) in m {
                    o.insert(k.clone(), Self::apply_vars_json(v, env));
                }
                serde_json::Value::Object(o)
            }
            _ => v.clone(),
        }
    }

    async fn run_step(
        &self,
        client: &reqwest::Client,
        base_env: &std::collections::HashMap<String, String>,
        step: &ScriptStep,
    ) -> Result<(), String> {
        let datasets = step
            .dataset
            .clone()
            .unwrap_or_else(|| vec![serde_json::json!({})]);
        for row in datasets {
            metrics::counter!("scripted_training_cell_requests_total").increment(1);
            // Build env: script/env + row fields
            let mut env = base_env.clone();
            if let Some(obj) = row.as_object() {
                for (k, v) in obj {
                    let s = match v.as_str() {
                        Some(s) => s.to_string(),
                        None => v.to_string(),
                    };
                    env.insert(k.clone(), s);
                }
            }
            // Hooks: pre
            if let Some(h) = &step.pre_hook {
                if !self.dry_run {
                    Self::run_hook(h).await;
                }
            }

            // Prepare request with substitutions
            let url = Self::substitute_vars(&step.url, &env);
            let mut headers = step.headers.clone().unwrap_or_default();
            for v in headers.values_mut() {
                *v = Self::substitute_vars(v, &env);
            }
            let body = step.body.as_ref().map(|b| Self::apply_vars_json(b, &env));
            let expect_contains = step
                .expect_contains
                .as_ref()
                .map(|s| Self::substitute_vars(s, &env));

            let run_once = || async {
                let req = match step.method.to_uppercase().as_str() {
                    "GET" => client.get(&url),
                    "POST" => client.post(&url),
                    "PUT" => client.put(&url),
                    "PATCH" => client.patch(&url),
                    "DELETE" => client.delete(&url),
                    m => return Err(format!("unsupported method {m}")),
                };
                let mut req = req;
                for (k, v) in &headers {
                    req = req.header(k, v);
                }
                if let Some(b) = &body {
                    req = req.json(b);
                }
                let timeout_dur = Duration::from_millis(step.timeout_ms.unwrap_or(15_000));
                let start = std::time::Instant::now();
                let resp = timeout(timeout_dur, req.send())
                    .await
                    .map_err(|_| "request timeout".to_string())
                    .and_then(|r| r.map_err(|e| e.to_string()))?;
                let elapsed = start.elapsed();
                if let Some(expect) = step.expect_status {
                    if resp.status().as_u16() != expect {
                        return Err(format!(
                            "unexpected status: {} != {}",
                            resp.status(),
                            expect
                        ));
                    }
                }
                let text = resp.text().await.map_err(|e| e.to_string())?;
                if let Some(substr) = &expect_contains {
                    if !text.contains(substr) {
                        return Err(Self::err_with_attachment(
                            "response does not contain expected substring",
                            &text,
                        ));
                    }
                }
                // JSON assertions
                if let Some(asserts) = &step.assertions {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Err(e) = Self::eval_asserts(&json, asserts) {
                            return Err(Self::err_with_attachment(&e, &text));
                        }
                    }
                }
                info!(
                    "step {} {} ok in {} ms",
                    step.method,
                    url,
                    elapsed.as_millis()
                );
                Ok::<(), String>(())
            };

            let mut attempts = step.retry.as_ref().map(|r| r.attempts).unwrap_or(1);
            let backoff = step.retry.as_ref().map(|r| r.backoff_ms).unwrap_or(1000);
            loop {
                match run_once().await {
                    Ok(_) => break,
                    Err(_e) if attempts > 1 => {
                        attempts -= 1;
                        let wait = backoff
                            * (step.retry.as_ref().map(|r| r.attempts).unwrap_or(1) - attempts)
                                as u64;
                        tokio::time::sleep(Duration::from_millis(wait)).await;
                        continue;
                    }
                    Err(e) => {
                        metrics::counter!("scripted_training_cell_errors_total").increment(1);
                        return Err(e);
                    }
                }
            }

            // Hooks: post
            if let Some(h) = &step.post_hook {
                if !self.dry_run {
                    Self::run_hook(h).await;
                }
            }
        }
        if self.dry_run {
            // already logged dry-run at the start of each row
        }
        Ok(())
    }

    fn err_with_attachment(msg: &str, body: &str) -> String {
        let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
        let dir = std::path::Path::new(&base).join("training");
        let _ = std::fs::create_dir_all(&dir);
        let ts = chrono::Utc::now().timestamp_millis();
        let path_rel = format!("training/failure_{}.txt", ts);
        let full = dir.join(format!("failure_{}.txt", ts));
        let mut snippet = body.to_string();
        if snippet.len() > 4096 {
            snippet.truncate(4096);
        }
        let _ = std::fs::write(&full, snippet);
        format!("{} [attachment:/context/{}]", msg, path_rel)
    }

    fn eval_asserts(json: &serde_json::Value, asserts: &[Assertion]) -> Result<(), String> {
        for a in asserts {
            let res = jsonpath_lib::select(json, &a.path).map_err(|e| e.to_string())?;
            if res.is_empty() {
                return Err(format!("jsonpath '{}' no match", a.path));
            }
            if let Some(eq) = &a.equals {
                if !res.contains(&eq) {
                    return Err(format!("jsonpath '{}' equals failed", a.path));
                }
            }
            if let Some(sub) = &a.contains {
                let mut ok = false;
                for v in &res {
                    if let Some(s) = v.as_str() {
                        if s.contains(sub) {
                            ok = true;
                            break;
                        }
                    }
                }
                if !ok {
                    return Err(format!("jsonpath '{}' contains failed", a.path));
                }
            }
            if a.gt.is_some() || a.lt.is_some() {
                // compare numeric
                let mut ok = false;
                for v in &res {
                    if let Some(n) = v.as_f64() {
                        if let Some(gt) = a.gt {
                            if n <= gt {
                                continue;
                            }
                        }
                        if let Some(lt) = a.lt {
                            if n >= lt {
                                continue;
                            }
                        }
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    return Err(format!("jsonpath '{}' range failed", a.path));
                }
            }
        }
        Ok(())
    }

    async fn run_hook(h: &Hook) {
        match h {
            Hook::Sleep { ms } => tokio::time::sleep(Duration::from_millis(*ms)).await,
            Hook::SetEnv { vars } => {
                for (k, v) in vars {
                    std::env::set_var(k, v);
                }
            }
            Hook::Shell { cmd } => {
                let allow = std::env::var("TRAINING_ALLOW_SHELL")
                    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                if !allow {
                    return;
                }
                #[cfg(target_os = "windows")]
                let mut c = std::process::Command::new("cmd");
                #[cfg(not(target_os = "windows"))]
                let mut c = std::process::Command::new("sh");
                #[cfg(target_os = "windows")]
                {
                    c.arg("/C").arg(cmd);
                }
                #[cfg(not(target_os = "windows"))]
                {
                    c.arg("-c").arg(cmd);
                }
                let _ = c.status();
            }
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        let script = self.load_script().await?;
        let mut progress = self.load_progress();
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| e.to_string())?;
        // Build base env from script.vars + .env
        let mut base_env: std::collections::HashMap<String, String> = std::env::vars().collect();
        if let Some(vars) = &script.vars {
            for (k, v) in vars {
                base_env.insert(k.clone(), v.clone());
            }
        }
        // Context storage (optional logging)
        let storage = crate::context::context_storage::FileContextStorage::new("context");
        let chat_id = "training";
        let session_id = "run";
        info!("starting scripted training: {}", script.name);
        let mut results: Vec<(usize, String, String, bool, u128, Option<String>)> = Vec::new();
        for (i, step) in script.steps.iter().enumerate() {
            if i < progress.last_completed {
                continue;
            }
            if self.dry_run {
                info!("[dry-run] {} {}", step.method, step.url);
            }
            let started = std::time::Instant::now();
            match self.run_step(&client, &base_env, step).await {
                Ok(_) => {
                    let dur = started.elapsed().as_millis();
                    progress.last_completed = i + 1;
                    self.save_progress(&progress);
                    info!("completed step {} of {}", i + 1, script.steps.len());
                    // metrics: record step count and duration here if enabled
                    let _ = storage.save_message(
                        chat_id,
                        session_id,
                        &crate::context::context_storage::ChatMessage {
                            role: crate::context::context_storage::Role::System,
                            content: format!("step {} ok: {} {}", i + 1, step.method, step.url),
                            timestamp_ms: chrono::Utc::now().timestamp_millis(),
                            source: Some("training".into()),
                            message_id: None,
                            thread_id: None,
                            parent_id: None,
                        },
                    );
                    results.push((
                        i + 1,
                        step.method.clone(),
                        step.url.clone(),
                        true,
                        dur,
                        None,
                    ));
                }
                Err(e) => {
                    let dur = started.elapsed().as_millis();
                    error!("step {} failed: {}", i + 1, e);
                    // metrics: record step count and duration here if enabled
                    let _ = storage.save_message(
                        chat_id,
                        session_id,
                        &crate::context::context_storage::ChatMessage {
                            role: crate::context::context_storage::Role::System,
                            content: format!("step {} failed: {}", i + 1, e),
                            timestamp_ms: chrono::Utc::now().timestamp_millis(),
                            source: Some("training".into()),
                            message_id: None,
                            thread_id: None,
                            parent_id: None,
                        },
                    );
                    results.push((
                        i + 1,
                        step.method.clone(),
                        step.url.clone(),
                        false,
                        dur,
                        Some(e),
                    ));
                    break;
                }
            }
        }
        Self::write_reports(&script.name, &results).ok();
        Ok(())
    }

    fn write_reports(
        name: &str,
        results: &[(usize, String, String, bool, u128, Option<String>)],
    ) -> Result<(), String> {
        let base = std::env::var("CONTEXT_DIR").unwrap_or_else(|_| "context".into());
        let dir = std::path::Path::new(&base).join("training");
        let _ = std::fs::create_dir_all(&dir);
        let tests = results.len();
        let failures = results.iter().filter(|r| !r.3).count();
        let time_sec: f64 = results.iter().map(|r| r.4 as f64 / 1000.0).sum();
        // JUnit XML
        let mut xml = String::new();
        xml.push_str(&format!(
            "<testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" time=\"{:.3}\">\n",
            name, tests, failures, time_sec
        ));
        for (idx, method, url, ok, dur, err) in results {
            let case_name = format!("{} {}", method, url);
            xml.push_str(&format!(
                "  <testcase name=\"step {}: {}\" time=\"{:.3}\">\n",
                idx,
                xml_escape(&case_name),
                *dur as f64 / 1000.0
            ));
            if !ok {
                let msg = xml_escape(err.as_deref().unwrap_or("failure"));
                xml.push_str(&format!("    <failure message=\"{}\"/>\n", msg));
            }
            xml.push_str("  </testcase>\n");
        }
        xml.push_str("</testsuite>\n");
        std::fs::write(dir.join("report.xml"), xml).map_err(|e| e.to_string())?;
        // HTML
        let mut html = String::new();
        html.push_str(
            "<html><head><meta charset=\"utf-8\"><title>Training Report</title></head><body>",
        );
        html.push_str(&format!("<h1>{}</h1>", html_escape(name)));
        html.push_str(&format!(
            "<p>Tests: {} Failures: {} Time: {:.3}s</p>",
            tests, failures, time_sec
        ));
        html.push_str("<table border=1 cellspacing=0 cellpadding=4><tr><th>#</th><th>Method</th><th>URL</th><th>Status</th><th>Time, ms</th><th>Error</th><th>Attachment</th></tr>");
        for (idx, method, url, ok, dur, err) in results {
            let (err_txt, link) = if let Some(e) = err {
                if let Some(pos) = e.find("[attachment:") {
                    let (msg, rest) = e.split_at(pos);
                    let mut href = String::new();
                    if let Some(start) = rest.find("/context/") {
                        let s2 = &rest[start..];
                        if let Some(end) = s2.find("]") {
                            href = s2[..end].to_string();
                        }
                    }
                    (msg.trim().to_string(), href)
                } else {
                    (e.clone(), String::new())
                }
            } else {
                (String::new(), String::new())
            };
            html.push_str(&format!("<tr><td>{}</td><td>{}</td><td>{}</td><td style=\"color:{}\">{}</td><td>{}</td><td>{}</td></tr>",
                idx,
                html_escape(method),
                html_escape(url),
                if *ok {"green"} else {"red"},
                if *ok {"OK"} else {"FAIL"},
                dur,
                html_escape(&err_txt)
            ));
            if !link.is_empty() {
                html.push_str(&format!(
                    "<tr><td colspan=7><a href=\"{}\" target=\"_blank\">attachment</a></td></tr>",
                    html_escape(&link)
                ));
            }
        }
        html.push_str("</table></body></html>");
        std::fs::write(dir.join("report.html"), html).map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('"', "&quot;")
}
fn html_escape(s: &str) -> String {
    xml_escape(s)
}

impl ActionCell for ScriptedTrainingCell {
    fn id(&self) -> &str {
        &self.id
    }

    fn preload(&self, triggers: &[String], _memory: &Arc<MemoryCell>) {
        // kick off training when a "train" trigger is present
        if triggers.iter().any(|t| t.eq_ignore_ascii_case("train")) {
            let cell = Self { ..self.clone() };
            tokio::spawn(async move {
                if let Err(e) = cell.run().await {
                    error!("training run error: {}", e);
                }
            });
        }
    }
}

impl Clone for ScriptedTrainingCell {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            script_path: self.script_path.clone(),
            progress_path: self.progress_path.clone(),
            dry_run: self.dry_run,
        }
    }
}

impl Default for ScriptedTrainingCell {
    fn default() -> Self {
        Self::from_env()
    }
}
