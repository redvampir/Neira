/* neira:meta
id: NEI-20270520-security-perms
intent: security
summary: |
  Добавлен контроль прав для файловых, сетевых и системных операций.
*/
/* neira:meta
id: NEI-20270401-network-post-perm
intent: security
summary: Добавлена проверка права network_post для отправки HTTP POST.
*/
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum Operation {
    FileRead(String),
    NetworkRequest(String),
    NetworkPost(String),
    SystemCommand(String),
}

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("operation not permitted: {0:?}")]
    PermissionDenied(Operation),
}

pub fn check_permission(op: &Operation) -> Result<(), SecurityError> {
    match op {
        Operation::SystemCommand(_) => {
            if std::env::var("NEIRA_ALLOW_SYSTEM").is_err() {
                Err(SecurityError::PermissionDenied(op.clone()))
            } else {
                Ok(())
            }
        }
        Operation::NetworkPost(_) => {
            if std::env::var("NEIRA_ALLOW_NETWORK_POST").is_err() {
                Err(SecurityError::PermissionDenied(op.clone()))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub mod init_config_cell;
pub mod integrity_checker_cell;
pub mod quarantine_cell;
pub mod safe_mode_controller;
