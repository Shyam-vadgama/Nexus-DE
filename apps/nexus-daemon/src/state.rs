use nexus_core::config::NexusConfig;
use nexus_core::theme::NexusPalette;
use nexus_core::NexusTier;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_pct: f32,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_used_mb: 0,
            memory_total_mb: 1,
            memory_pct: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct DaemonState {
    pub config: NexusConfig,
    pub palette: NexusPalette,
    pub metrics: SystemMetrics,
}

impl DaemonState {
    pub fn new() -> Self {
        let config_path = NexusConfig::default_config_path();
        let config = NexusConfig::load_from_file(&config_path).unwrap_or_default();

        let palette = if let Some(ref wallpaper) = config.appearance.wallpaper_path {
            NexusPalette::from_wallpaper(wallpaper).unwrap_or_else(|_| {
                NexusPalette::from_hex(&config.appearance.accent_color).unwrap_or_default()
            })
        } else {
            NexusPalette::from_hex(&config.appearance.accent_color).unwrap_or_default()
        };

        Self {
            config,
            palette,
            metrics: SystemMetrics::default(),
        }
    }

    pub fn tier(&self) -> NexusTier {
        self.config.general.tier
    }

    pub fn set_tier(&mut self, tier: NexusTier) {
        self.config.general.tier = tier;
        let config_path = NexusConfig::default_config_path();
        let _ = self.config.save_to_file(config_path);
    }
}

pub type SharedState = Arc<RwLock<DaemonState>>;
