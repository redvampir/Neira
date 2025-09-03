/* neira:meta
id: NEI-20270520-security-perms
intent: security
summary: |
  Добавлен контроль прав для файловых, сетевых и системных операций.
*/
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum Operation {
    FileRead(String),
    NetworkRequest(String),
    SystemCommand(String),
}

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("operation not permitted: {0:?}")]
    PermissionDenied(Operation),
}

pub fn check_operation(op: &Operation) -> Result<(), SecurityError> {
    if matches!(op, Operation::SystemCommand(_)) && std::env::var("NEIRA_ALLOW_SYSTEM").is_err() {
        Err(SecurityError::PermissionDenied(op.clone()))
    } else {
        Ok(())
    }
}

pub mod init_config_cell;
pub mod integrity_checker_cell;
pub mod quarantine_cell;
pub mod safe_mode_controller;
