use crate::state::SharedState;
use nexus_core::config::NexusConfig;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use tokio::time::Duration;
use tracing::{error, info};

pub fn start_config_watcher(state: SharedState) {
    let config_path = NexusConfig::default_config_path();
    let parent_dir = match config_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return,
    };

    if !parent_dir.exists() {
        let _ = std::fs::create_dir_all(&parent_dir);
    }

    std::thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher = match RecommendedWatcher::new(tx, notify::Config::default()) {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to initialize config file watcher: {}", e);
                return;
            }
        };

        if let Err(e) = watcher.watch(Path::new(&parent_dir), RecursiveMode::NonRecursive) {
            error!("Failed to watch config directory {:?}: {}", parent_dir, e);
            return;
        }

        info!("Watching configuration file in {:?}", parent_dir);

        while let Ok(res) = rx.recv() {
            match res {
                Ok(Event {
                    kind: EventKind::Modify(_) | EventKind::Create(_),
                    paths,
                    ..
                }) => {
                    let affects_config = paths.iter().any(|p| p.ends_with("config.toml"));
                    if affects_config {
                        info!("Configuration file modified, reloading...");
                        // Debounce slightly
                        std::thread::sleep(Duration::from_millis(50));
                        if let Ok(new_cfg) = NexusConfig::load_from_file(&config_path) {
                            let state_clone = state.clone();
                            tokio::spawn(async move {
                                let mut lock = state_clone.write().await;
                                lock.config = new_cfg;
                                info!("Configuration hot-reloaded successfully.");
                            });
                        }
                    }
                }
                Err(e) => error!("Watch error: {}", e),
                _ => {}
            }
        }
    });
}
