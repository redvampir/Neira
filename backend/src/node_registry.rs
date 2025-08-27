use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::Value;
use tracing::{error, info};

use crate::node_template::{validate_template, NodeTemplate};

/// Загружает `NodeTemplate` из файла JSON или YAML.
fn load_template(path: &Path) -> Result<NodeTemplate, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let value: Value = match path.extension().and_then(|s| s.to_str()) {
        Some("yaml") | Some("yml") => {
            let yaml: serde_yaml::Value =
                serde_yaml::from_str(&content).map_err(|e| format!("invalid YAML: {e}"))?;
            serde_json::to_value(yaml).map_err(|e| format!("YAML to JSON: {e}"))?
        }
        _ => serde_json::from_str(&content).map_err(|e| format!("invalid JSON: {e}"))?,
    };
    validate_template(&value).map_err(|errs| errs.join(", "))?;
    serde_json::from_value(value).map_err(|e| format!("deserialize NodeTemplate: {e}"))
}

/// Реестр узлов: хранит метаданные и следит за изменениями файлов.
pub struct NodeRegistry {
    root: PathBuf,
    nodes: Arc<RwLock<HashMap<String, NodeTemplate>>>,
    paths: Arc<RwLock<HashMap<PathBuf, String>>>,
    _watcher: RecommendedWatcher,
}

impl NodeRegistry {
    /// Создаёт реестр и запускает наблюдение за каталогом.
    pub fn new(dir: impl AsRef<Path>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        let nodes = Arc::new(RwLock::new(HashMap::new()));
        let paths = Arc::new(RwLock::new(HashMap::new()));

        // Начальная загрузка файлов
        for entry in fs::read_dir(&dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.is_file() {
                match load_template(&path) {
                    Ok(tpl) => {
                        paths.write().unwrap().insert(path.clone(), tpl.id.clone());
                        nodes.write().unwrap().insert(tpl.id.clone(), tpl);
                    }
                    Err(e) => error!("{}", e),
                }
            }
        }

        let nodes_w = nodes.clone();
        let paths_w = paths.clone();
        let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    for path in event.paths {
                        if !path.is_file() {
                            continue;
                        }
                        match event.kind {
                            EventKind::Remove(_) => {
                                if let Some(id) = paths_w.write().unwrap().remove(&path) {
                                    nodes_w.write().unwrap().remove(&id);
                                    info!("Removed node {}", id);
                                }
                            }
                            _ => match load_template(&path) {
                                Ok(tpl) => {
                                    paths_w
                                        .write()
                                        .unwrap()
                                        .insert(path.clone(), tpl.id.clone());
                                    nodes_w.write().unwrap().insert(tpl.id.clone(), tpl);
                                    info!("Loaded node template {}", path.display());
                                }
                                Err(e) => error!("{e}"),
                            },
                        }
                    }
                }
                Err(e) => error!("watch error: {e}"),
            },
            Config::default(),
        )
        .map_err(|e| e.to_string())?;

        watcher
            .watch(&dir, RecursiveMode::NonRecursive)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            root: dir,
            nodes,
            paths,
            _watcher: watcher,
        })
    }

    /// Регистрация или обновление узла из файла.
    pub fn register(&self, path: &Path) -> Result<(), String> {
        let tpl = load_template(path)?;
        self.paths
            .write()
            .unwrap()
            .insert(path.to_path_buf(), tpl.id.clone());
        self.nodes.write().unwrap().insert(tpl.id.clone(), tpl);
        Ok(())
    }

    /// Регистрация узла по структуре `NodeTemplate` с сохранением на диск.
    pub fn register_template(&self, tpl: NodeTemplate) -> Result<(), String> {
        let file = format!("{}-{}.json", tpl.id, tpl.version);
        let path = self.root.join(file);
        let content = serde_json::to_string(&tpl).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())?;
        self.register(&path)
    }

    /// Получение метаданных узла по идентификатору.
    pub fn get(&self, id: &str) -> Option<NodeTemplate> {
        self.nodes.read().unwrap().get(id).cloned()
    }
}
