use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;

pub struct IdempotentStore {
    path: PathBuf,
    ttl_secs: u64,
    map: RwLock<HashMap<String, (String, i64)>>,
}

impl IdempotentStore {
    pub fn new(dir: impl AsRef<Path>, ttl_secs: u64) -> Self {
        let dir = dir.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("idempotent.jsonl");
        let mut map: HashMap<String, (String, i64)> = HashMap::new();
        let now = now_secs() as i64;
        if let Ok(file) = OpenOptions::new().read(true).create(true).open(&path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if line.trim().is_empty() { continue; }
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
                    if let (Some(k), Some(val), Some(exp)) = (
                        v.get("k").and_then(|x| x.as_str()),
                        v.get("v").and_then(|x| x.as_str()),
                        v.get("exp").and_then(|x| x.as_i64()),
                    ) {
                        if exp >= now { map.insert(k.to_string(), (val.to_string(), exp)); }
                    }
                }
            }
        }
        Self { path, ttl_secs, map: RwLock::new(map) }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let now = now_secs() as i64;
        if let Some((v, exp)) = self.map.read().get(key).cloned() {
            if exp >= now { return Some(v); }
        }
        None
    }

    pub fn put(&self, key: &str, value: &str) {
        let exp = now_secs() as i64 + self.ttl_secs as i64;
        self.map.write().insert(key.to_string(), (value.to_string(), exp));
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&self.path) {
            let rec = serde_json::json!({"k": key, "v": value, "exp": exp});
            let _ = writeln!(f, "{}", rec.to_string());
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
