/* neira:meta
id: NEI-20250829-175425-node-registry
intent: docs
summary: |
  Отслеживает файлы шаблонов узлов и регистрирует реализации в системе.
*/

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::Value;
use tracing::{error, info};

use crate::action::chat_node::ChatNode;
use crate::action::scripted_training_node::ScriptedTrainingNode;
use crate::action_node::ActionNode;
use crate::analysis_node::AnalysisNode;
use crate::memory_node::MemoryNode;
use crate::node_template::{
    validate_action_template, validate_template, ActionNodeTemplate, NodeTemplate,
};

/* neira:meta
id: NEI-20250309-125000-load-template-impl
intent: refactor
summary: |
  Объединяет чтение файла и валидацию шаблонов узлов в общую функцию.
*/
fn load_template_impl<T, F>(path: &Path, validate_fn: F) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
    F: Fn(&Value) -> Result<(), Vec<String>>,
{
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let value: Value = if matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("yaml") | Some("yml")
    ) {
        let yaml: serde_yaml::Value =
            serde_yaml::from_str(&content).map_err(|e| format!("invalid YAML: {e}"))?;
        serde_json::to_value(yaml).map_err(|e| format!("YAML to JSON: {e}"))?
    } else {
        serde_json::from_str(&content).map_err(|e| format!("invalid JSON: {e}"))?
    };
    validate_fn(&value).map_err(|errs| errs.join(", "))?;
    serde_json::from_value(value).map_err(|e| format!("deserialize: {e}"))
}

/// Загружает `NodeTemplate` из файла.
fn load_template(path: &Path) -> Result<NodeTemplate, String> {
    load_template_impl(path, validate_template)
}

/// Загружает `ActionNodeTemplate` из файла.
fn load_action_template(path: &Path) -> Result<ActionNodeTemplate, String> {
    load_template_impl(path, validate_action_template)
}

/// Реестр узлов: хранит метаданные и следит за изменениями файлов.
pub struct NodeRegistry {
    root: PathBuf,
    nodes: Arc<RwLock<HashMap<String, NodeTemplate>>>,
    paths: Arc<RwLock<HashMap<PathBuf, String>>>,
    action_templates: Arc<RwLock<HashMap<String, ActionNodeTemplate>>>,
    action_paths: Arc<RwLock<HashMap<PathBuf, String>>>,
    analysis_nodes: Arc<RwLock<HashMap<String, Arc<dyn AnalysisNode + Send + Sync>>>>,
    action_nodes: Arc<RwLock<Vec<Arc<dyn ActionNode>>>>,
    chat_nodes: Arc<RwLock<HashMap<String, Arc<dyn ChatNode + Send + Sync>>>>,
    _watcher: RecommendedWatcher,
}

impl NodeRegistry {
    /// Создаёт реестр и запускает наблюдение за каталогом.
    pub fn new(dir: impl AsRef<Path>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        let nodes = Arc::new(RwLock::new(HashMap::new()));
        let paths = Arc::new(RwLock::new(HashMap::new()));
        let action_templates = Arc::new(RwLock::new(HashMap::new()));
        let action_paths = Arc::new(RwLock::new(HashMap::new()));
        let analysis_nodes = Arc::new(RwLock::new(HashMap::new()));
        let action_nodes = Arc::new(RwLock::new(Vec::new()));
        let chat_nodes = Arc::new(RwLock::new(HashMap::new()));

        // Начальная загрузка файлов
        for entry in fs::read_dir(&dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.is_file() {
                if let Ok(tpl) = load_template(&path) {
                    paths.write().unwrap().insert(path.clone(), tpl.id.clone());
                    nodes.write().unwrap().insert(tpl.id.clone(), tpl);
                } else if let Ok(tpl) = load_action_template(&path) {
                    action_paths
                        .write()
                        .unwrap()
                        .insert(path.clone(), tpl.id.clone());
                    action_templates
                        .write()
                        .unwrap()
                        .insert(tpl.id.clone(), tpl);
                } else {
                    error!("failed to load template {}", path.display());
                }
            }
        }

        let nodes_w = nodes.clone();
        let paths_w = paths.clone();
        let action_tpls_w = action_templates.clone();
        let action_paths_w = action_paths.clone();
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
                                } else if let Some(id) =
                                    action_paths_w.write().unwrap().remove(&path)
                                {
                                    action_tpls_w.write().unwrap().remove(&id);
                                    info!("Removed action node {}", id);
                                }
                            }
                            _ => {
                                if let Ok(tpl) = load_template(&path) {
                                    paths_w
                                        .write()
                                        .unwrap()
                                        .insert(path.clone(), tpl.id.clone());
                                    nodes_w.write().unwrap().insert(tpl.id.clone(), tpl);
                                    info!("Loaded node template {}", path.display());
                                } else if let Ok(tpl) = load_action_template(&path) {
                                    action_paths_w
                                        .write()
                                        .unwrap()
                                        .insert(path.clone(), tpl.id.clone());
                                    action_tpls_w.write().unwrap().insert(tpl.id.clone(), tpl);
                                    info!("Loaded action node template {}", path.display());
                                } else {
                                    error!("failed to load template {}", path.display());
                                }
                            }
                        }
                    }
                }
                Err(e) => error!("watch error: {e}"),
            },
            Config::default(),
        )
        .map_err(|e| e.to_string())?;

        /* neira:meta
        id: NEI-20250310-node-registry-recursive
        intent: fix
        summary: Включено рекурсивное наблюдение за каталогом шаблонов узлов.
        */
        watcher
            .watch(&dir, RecursiveMode::Recursive)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            root: dir,
            nodes,
            paths,
            action_templates,
            action_paths,
            analysis_nodes,
            action_nodes,
            chat_nodes,
            _watcher: watcher,
        })
    }

    pub fn register_scripted_training_node(&self) {
        self.register_action_node(Arc::new(ScriptedTrainingNode::default()));
        info!("Registered scripted training node");
    }

    pub fn register_init_node(&self, node: Arc<dyn ActionNode>, memory: &Arc<MemoryNode>) {
        node.preload(&[], memory);
        self.action_nodes.write().unwrap().insert(0, node);
    }

    /// Регистрация или обновление узла из файла.
    pub fn register(&self, path: &Path) -> Result<(), String> {
        if let Ok(tpl) = load_template(path) {
            self.paths
                .write()
                .unwrap()
                .insert(path.to_path_buf(), tpl.id.clone());
            self.nodes.write().unwrap().insert(tpl.id.clone(), tpl);
        } else {
            let tpl = load_action_template(path)?;
            self.action_paths
                .write()
                .unwrap()
                .insert(path.to_path_buf(), tpl.id.clone());
            self.action_templates
                .write()
                .unwrap()
                .insert(tpl.id.clone(), tpl);
        }
        Ok(())
    }

    /// Регистрация узла по структуре `NodeTemplate` с сохранением на диск.
    /* neira:meta
    id: NEI-20250210-register-template-validate
    intent: bugfix
    summary: Проверяет шаблон узла перед сохранением на диск.
    */
    pub fn register_template(&self, tpl: NodeTemplate) -> Result<(), String> {
        let value = tpl.to_json();
        validate_template(&value).map_err(|errs| errs.join(", "))?;
        let file = format!("{}-{}.json", tpl.id, tpl.version);
        let path = self.root.join(file);
        let content = serde_json::to_string(&tpl).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())?;
        self.register(&path)
    }

    /* neira:meta
    id: NEI-20250214-154000-register-action-template
    intent: feature
    summary: Регистрирует шаблон узла действия и сохраняет его на диск.
    */
    pub fn register_action_template(&self, tpl: ActionNodeTemplate) -> Result<(), String> {
        let value = tpl.to_json();
        validate_action_template(&value).map_err(|errs| errs.join(", "))?;
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

    /// Получение шаблона узла действия по идентификатору.
    pub fn get_action_template(&self, id: &str) -> Option<ActionNodeTemplate> {
        self.action_templates.read().unwrap().get(id).cloned()
    }

    /// Регистрация реализации `AnalysisNode`.
    pub fn register_analysis_node(&self, node: Arc<dyn AnalysisNode + Send + Sync>) {
        self.analysis_nodes
            .write()
            .unwrap()
            .insert(node.id().to_string(), node);
    }

    /// Получение реализации `AnalysisNode` по идентификатору.
    pub fn get_analysis_node(&self, id: &str) -> Option<Arc<dyn AnalysisNode + Send + Sync>> {
        self.analysis_nodes.read().unwrap().get(id).cloned()
    }

    pub fn register_action_node(&self, node: Arc<dyn ActionNode>) {
        self.action_nodes.write().unwrap().push(node);
    }

    pub fn action_nodes(&self) -> Vec<Arc<dyn ActionNode>> {
        self.action_nodes.read().unwrap().clone()
    }

    pub fn register_chat_node(&self, node: Arc<dyn ChatNode + Send + Sync>) {
        self.chat_nodes
            .write()
            .unwrap()
            .insert(node.id().to_string(), node);
    }

    pub fn get_chat_node(&self, id: &str) -> Option<Arc<dyn ChatNode + Send + Sync>> {
        self.chat_nodes.read().unwrap().get(id).cloned()
    }
}
