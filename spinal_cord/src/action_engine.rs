/* neira:meta
id: NEI-20270520-action-engine
intent: feature
summary: |
  Асинхронный движок для файловых, сетевых и системных операций с проверкой прав.
*/
use crate::security::{check_operation, Operation, SecurityError};
use reqwest;
use thiserror::Error;
use tokio::{fs, process::Command};

#[derive(Debug, Clone)]
pub enum ActionCommand {
    ReadFile { path: String },
    HttpGet { url: String },
    RunCommand { program: String, args: Vec<String> },
}

pub struct ActionEngine;

impl ActionEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, cmd: ActionCommand) -> Result<String, ActionError> {
        let op = match &cmd {
            ActionCommand::ReadFile { path } => Operation::FileRead(path.clone()),
            ActionCommand::HttpGet { url } => Operation::NetworkRequest(url.clone()),
            ActionCommand::RunCommand { program, .. } => Operation::SystemCommand(program.clone()),
        };
        check_operation(&op)?;
        match cmd {
            ActionCommand::ReadFile { path } => Ok(fs::read_to_string(path).await?),
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
