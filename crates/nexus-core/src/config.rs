use crate::tier::NexusTier;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}

/// Root NEXUS configuration structure stored in ~/.config/nexus/config.toml
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NexusConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub appearance: AppearanceConfig,
    #[serde(default)]
    pub bar: BarConfig,
}

impl NexusConfig {
    /// Load config from file or return default if missing
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        if !path.as_ref().exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let config: NexusConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save current config to a target path
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(self)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    /// Default user config location
    pub fn default_config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".config/nexus/config.toml")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    #[serde(default)]
    pub tier: NexusTier,
    #[serde(default = "default_true")]
    pub auto_reload: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            tier: NexusTier::Core,
            auto_reload: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceConfig {
    #[serde(default = "default_accent")]
    pub accent_color: String,
    #[serde(default = "default_true")]
    pub dynamic_theming: bool,
    #[serde(default)]
    pub wallpaper_path: Option<String>,
    #[serde(default = "default_corner_radius")]
    pub corner_radius: u32,
    #[serde(default = "default_blur_strength")]
    pub blur_strength: u32,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            accent_color: default_accent(),
            dynamic_theming: true,
            wallpaper_path: None,
            corner_radius: default_corner_radius(),
            blur_strength: default_blur_strength(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BarConfig {
    #[serde(default = "default_bar_position")]
    pub position: String,
    #[serde(default = "default_bar_height")]
    pub height: u32,
    #[serde(default = "default_true")]
    pub show_workspaces: bool,
    #[serde(default = "default_true")]
    pub show_tray: bool,
    #[serde(default = "default_true")]
    pub show_clock: bool,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            position: default_bar_position(),
            height: default_bar_height(),
            show_workspaces: true,
            show_tray: true,
            show_clock: true,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_accent() -> String {
    "#38BDF8".to_string() // Cyan/Sky blue default
}

fn default_corner_radius() -> u32 {
    14
}

fn default_blur_strength() -> u32 {
    20
}

fn default_bar_position() -> String {
    "top".to_string()
}

fn default_bar_height() -> u32 {
    44
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_serialization() {
        let default_config = NexusConfig::default();
        let serialized = toml::to_string_pretty(&default_config).expect("Serialize to TOML");
        let deserialized: NexusConfig = toml::from_str(&serialized).expect("Deserialize from TOML");
        assert_eq!(default_config, deserialized);
    }

    #[test]
    fn test_partial_toml_deserialization() {
        let partial_toml = r#"
            [general]
            tier = "hyper"

            [appearance]
            corner_radius = 22
        "#;
        let config: NexusConfig = toml::from_str(partial_toml).expect("Deserialize partial TOML");
        assert_eq!(config.general.tier, NexusTier::Hyper);
        assert_eq!(config.appearance.corner_radius, 22);
        assert_eq!(config.bar.position, "top");
    }
}
