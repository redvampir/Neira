use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use semver::Version;
use serde_json::Value;
use tracing::{error, info};
use metrics::{increment_counter, gauge};

use crate::node_template::{validate_template, NodeTemplate};

/// Загружает `NodeTemplate` из файла JSON или YAML.
fn load_template(path: &Path) -> Result<NodeTemplate, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let value: Value = match path.extension().and_then(|s| s.to_str()) {
        Some("yaml") | Some("yml") => {
            let yaml: serde_yaml::Value =
                serde_yaml::from_str(&content).map_err(|e| format!("invalid YAML: {e}"))?;
            serde_json::to_value(yaml).map_err(|e| format!("YAML to JSON: {e}"))?
        }
        _ => serde_json::from_str(&content)
            .map_err(|e| format!("invalid JSON: {e}"))?,
    };
    validate_template(&value).map_err(|errs| errs.join(", "))?;
    serde_json::from_value(value).map_err(|e| format!("deserialize NodeTemplate: {e}"))
}

/// Реестр узлов: хранит метаданные и следит за изменениями файлов.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum NodeState {
    Draft,
    Active,
    Deprecated,
    Archived,
    Error,
}

pub struct NodeRegistry {
    versions: Arc<RwLock<HashMap<String, BTreeMap<Version, NodeTemplate>>>>,
    states: Arc<RwLock<HashMap<String, NodeState>>>,
    paths: Arc<RwLock<HashMap<PathBuf, (String, Version)>>>,
    dir: PathBuf,
    _watcher: RecommendedWatcher,
}

impl NodeRegistry {
    /// Создаёт реестр и запускает наблюдение за каталогом.
    pub fn new(dir: impl AsRef<Path>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        let versions: Arc<RwLock<HashMap<String, BTreeMap<Version, NodeTemplate>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let states = Arc::new(RwLock::new(HashMap::new()));
        let paths = Arc::new(RwLock::new(HashMap::new()));

        // Начальная загрузка файлов
        for entry in fs::read_dir(&dir)
            .map_err(|e| format!("read_dir {}: {e}", dir.display()))?
        {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.is_file() {
                match load_template(&path) {
                    Ok(tpl) => {
                        let ver = Version::parse(&tpl.version)
                            .map_err(|e| e.to_string())?;
                        let id = tpl.id.clone();
                        paths
                            .write()
                            .unwrap()
                            .insert(path.clone(), (id.clone(), ver.clone()));
                        versions
                            .write()
                            .unwrap()
                            .entry(id.clone())
                            .or_default()
                            .insert(ver, tpl);
                        states
                            .write()
                            .unwrap()
                            .entry(id)
                            .or_insert(NodeState::Active);
                    }
                    Err(e) => error!("{}", e),
                }
            }
        }

        let versions_w = versions.clone();
        let states_w = states.clone();
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
                                if let Some((id, ver)) =
                                    paths_w.write().unwrap().remove(&path)
                                {
                                    if let Some(map) = versions_w.write().unwrap().get_mut(&id) {
                                        map.remove(&ver);
                                        if map.is_empty() {
                                            versions_w.write().unwrap().remove(&id);
                                            states_w.write().unwrap().remove(&id);
                                        }
                                    }
                                    increment_counter!("node_registry_removed_total");
                                    gauge!("node_registry_templates", versions_w.read().unwrap().values().map(|m| m.len()).sum::<usize>() as f64);
                                    info!("Removed node {} {}", id, ver);
                                }
                            }
                            _ => match load_template(&path) {
                                Ok(tpl) => {
                                    let ver = Version::parse(&tpl.version)
                                        .map_err(|e| error!("parse version: {e}"))
                                        .ok();
                                    if let Some(ver) = ver {
                                        let id = tpl.id.clone();
                                        paths_w
                                            .write()
                                            .unwrap()
                                            .insert(path.clone(), (id.clone(), ver.clone()));
                                        versions_w
                                            .write()
                                            .unwrap()
                                            .entry(id.clone())
                                            .or_default()
                                            .insert(ver.clone(), tpl);
                                        states_w
                                            .write()
                                            .unwrap()
                                            .entry(id)
                                            .or_insert(NodeState::Active);
                                        increment_counter!("node_registry_loaded_total");
                                        gauge!("node_registry_templates", versions_w.read().unwrap().values().map(|m| m.len()).sum::<usize>() as f64);
                                        info!("Loaded node template {}", path.display());
                                    }
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

        gauge!("node_registry_templates", versions.read().unwrap().values().map(|m| m.len()).sum::<usize>() as f64);

        Ok(Self {
            versions,
            states,
            paths,
            dir,
            _watcher: watcher,
        })
    }

    /// Регистрация или обновление узла из файла.
    pub fn register(&self, path: &Path) -> Result<(), String> {
        let tpl = load_template(path)?;
        let ver = Version::parse(&tpl.version).map_err(|e| e.to_string())?;
        let id = tpl.id.clone();
        self.paths
            .write()
            .unwrap()
            .insert(path.to_path_buf(), (id.clone(), ver.clone()));
        self.versions
            .write()
            .unwrap()
            .entry(id.clone())
            .or_default()
            .insert(ver.clone(), tpl);
        self.states
            .write()
            .unwrap()
            .entry(id)
            .or_insert(NodeState::Active);
        increment_counter!("node_registry_loaded_total");
        gauge!("node_registry_templates", self.versions.read().unwrap().values().map(|m| m.len()).sum::<usize>() as f64);
        Ok(())
    }

    /// Регистрация шаблона из памяти с сохранением на диск.
    pub fn register_template(&self, tpl: NodeTemplate) -> Result<(), String> {
        let ver = Version::parse(&tpl.version).map_err(|e| e.to_string())?;
        let filename = format!("{}-{}.json", tpl.id, tpl.version);
        let path = self.dir.join(&filename);
        let json = serde_json::to_string_pretty(&tpl.to_json())
            .map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())?;
        let id = tpl.id.clone();
        self.paths
            .write()
            .unwrap()
            .insert(path.clone(), (id.clone(), ver.clone()));
        self.versions
            .write()
            .unwrap()
            .entry(id.clone())
            .or_default()
            .insert(ver.clone(), tpl);
        self.states
            .write()
            .unwrap()
            .entry(id)
            .or_insert(NodeState::Active);
        increment_counter!("node_registry_loaded_total");
        gauge!("node_registry_templates", self.versions.read().unwrap().values().map(|m| m.len()).sum::<usize>() as f64);
        Ok(())
    }

    /// Получение метаданных узла по идентификатору и версии. Если версия не указана, возвращается последняя.
    pub fn get(&self, id: &str, version: Option<&str>) -> Option<NodeTemplate> {
        let map = self.versions.read().unwrap();
        let versions = map.get(id)?;
        match version {
            Some(v) => Version::parse(v).ok().and_then(|ver| versions.get(&ver).cloned()),
            None => versions.iter().next_back().map(|(_, tpl)| tpl.clone()),
        }
    }

    pub fn set_state(&self, id: &str, state: NodeState) {
        self.states.write().unwrap().insert(id.to_string(), state);
    }

    pub fn get_state(&self, id: &str) -> Option<NodeState> {
        self.states.read().unwrap().get(id).copied()
    }
}

