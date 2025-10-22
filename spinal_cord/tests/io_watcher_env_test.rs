use backend::config::Config;

#[test]
fn io_watcher_env_fallback_and_override() {
    std::env::remove_var("PROBES_IO_WATCHER_ENABLED");
    std::env::remove_var("IO_WATCHER_ENABLED");

    // Legacy variable works when new one is absent
    std::env::set_var("IO_WATCHER_ENABLED", "1");
    let cfg = Config::from_env();
    assert!(
        cfg.probes
            .get("io_watcher")
            .expect("io watcher probe")
            .enabled
    );
    std::env::remove_var("IO_WATCHER_ENABLED");

    // New variable overrides legacy value
    std::env::set_var("PROBES_IO_WATCHER_ENABLED", "0");
    std::env::set_var("IO_WATCHER_ENABLED", "1");
    let cfg = Config::from_env();
    assert!(
        !cfg.probes
            .get("io_watcher")
            .expect("io watcher probe")
            .enabled
    );
    std::env::remove_var("PROBES_IO_WATCHER_ENABLED");
    std::env::remove_var("IO_WATCHER_ENABLED");
}
