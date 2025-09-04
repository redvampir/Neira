/* neira:meta
id: NEI-20270520-action-engine
intent: feature
summary: |
  Асинхронный движок для файловых, сетевых и системных операций с проверкой прав.
*/
use crate::security::{check_operation, Operation, SecurityError};
use reqwest;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::{fs, process::Command, sync::Mutex};

#[derive(Debug, Clone)]
pub enum ActionCommand {
    ReadFile { path: String },
    HttpGet { url: String },
    RunCommand { program: String, args: Vec<String> },
}

/* neira:meta
id: NEI-20271203-file-cache
intent: perf
summary: |
  Добавлен FileCache для кэширования чтений файлов.
*/
#[derive(Default)]
pub struct FileCache {
    inner: Mutex<HashMap<PathBuf, String>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, path: &Path) -> Option<String> {
        let cache = self.inner.lock().await;
        cache.get(path).cloned()
    }

    pub async fn insert(&self, path: PathBuf, contents: String) {
        let mut cache = self.inner.lock().await;
        cache.insert(path, contents);
    }
}

pub struct ActionEngine {
    cache: FileCache,
}

impl ActionEngine {
    pub fn new() -> Self {
        Self {
            cache: FileCache::new(),
        }
    }

    pub async fn execute(&self, cmd: ActionCommand) -> Result<String, ActionError> {
        let op = match &cmd {
            ActionCommand::ReadFile { path } => Operation::FileRead(path.clone()),
            ActionCommand::HttpGet { url } => Operation::NetworkRequest(url.clone()),
            ActionCommand::RunCommand { program, .. } => Operation::SystemCommand(program.clone()),
        };
        check_operation(&op)?;
        match cmd {
            ActionCommand::ReadFile { path } => {
                let path_buf = PathBuf::from(&path);
                if let Some(cached) = self.cache.get(&path_buf).await {
                    return Ok(cached);
                }
                let contents = fs::read_to_string(&path).await?;
                self.cache.insert(path_buf, contents.clone()).await;
                Ok(contents)
            }
            ActionCommand::HttpGet { url } => Ok(reqwest::get(&url).await?.text().await?),
            ActionCommand::RunCommand { program, args } => {
                let output = Command::new(program).args(args).output().await?;
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ActionError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Network(#[from] reqwest::Error),
    #[error(transparent)]
    Security(#[from] SecurityError),
}
