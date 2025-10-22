/* neira:meta
id: NEI-20250101-000005-context-dir-test
intent: test
summary: Проверяет дефолтный путь и переопределение CONTEXT_DIR.
*/
use backend::context::context_dir;
use serial_test::serial;
use std::env;
use std::path::PathBuf;

#[test]
#[serial]
fn context_dir_default() {
    let prev = env::var_os("CONTEXT_DIR");
    env::remove_var("CONTEXT_DIR");
    let dir = context_dir();
    assert_eq!(dir, PathBuf::from("context"));
    if let Some(val) = prev {
        env::set_var("CONTEXT_DIR", val);
    } else {
        env::remove_var("CONTEXT_DIR");
    }
}

#[test]
#[serial]
fn context_dir_overridden() {
    let prev = env::var_os("CONTEXT_DIR");
    env::set_var("CONTEXT_DIR", "/tmp/ctx_override");
    let dir = context_dir();
    assert_eq!(dir, PathBuf::from("/tmp/ctx_override"));
    if let Some(val) = prev {
        env::set_var("CONTEXT_DIR", val);
    } else {
        env::remove_var("CONTEXT_DIR");
    }
}
