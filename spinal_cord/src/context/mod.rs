/* neira:meta
id: NEI-20250101-000000-context-dir-helper
intent: code
summary: Добавлена функция context_dir с учётом переменной CONTEXT_DIR.
*/
use std::path::PathBuf;

pub mod context_storage;

/// Возвращает путь к директории хранения контекста.
///
/// Приоритет отдаётся переменной окружения `CONTEXT_DIR`.
/// Если переменная отсутствует, используется директория `context`.
pub fn context_dir() -> PathBuf {
    std::env::var_os("CONTEXT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("context"))
}
