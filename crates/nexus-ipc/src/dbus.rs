use zbus::proxy;

/// D-Bus Service Name
pub const NEXUS_DBUS_SERVICE: &str = "org.nexus.DE";

/// D-Bus Object Path
pub const NEXUS_DBUS_PATH: &str = "/org/nexus/DE";

/// D-Bus Main Interface Name
pub const NEXUS_DBUS_INTERFACE: &str = "org.nexus.DE";

/// Zbus Proxy for the org.nexus.DE D-Bus interface.
/// Used by nexus-settings, Quickshell shell bridges, and CLI tools.
#[proxy(
    default_service = "org.nexus.DE",
    default_path = "/org/nexus/DE",
    interface = "org.nexus.DE"
)]
pub trait NexusDe {
    /// Get currently active desktop tier (1: Minimal, 2: Core, 3: Hyper)
    fn get_tier(&self) -> zbus::Result<u8>;

    /// Set active desktop tier
    fn set_tier(&self, tier: u8) -> zbus::Result<bool>;

    /// Get current configuration as JSON string
    fn get_config(&self) -> zbus::Result<String>;

    /// Get current Material You palette as JSON string
    fn get_palette(&self) -> zbus::Result<String>;

    /// Apply wallpaper and trigger dynamic palette regeneration
    fn set_wallpaper(&self, path: &str) -> zbus::Result<bool>;

    /// Get real-time system metrics (CPU, RAM, battery) as JSON string
    fn get_system_metrics(&self) -> zbus::Result<String>;

    /// Signal emitted when the desktop tier changes
    #[zbus(signal)]
    fn tier_changed(&self, tier: u8) -> zbus::Result<()>;

    /// Signal emitted when color theme changes
    #[zbus(signal)]
    fn theme_changed(&self, palette_json: &str) -> zbus::Result<()>;

    /// Signal emitted when system metrics update
    #[zbus(signal)]
    fn metrics_updated(&self, metrics_json: &str) -> zbus::Result<()>;

    /// Signal emitted when config is reloaded
    #[zbus(signal)]
    fn config_reloaded(&self) -> zbus::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbus_constants() {
        assert_eq!(NEXUS_DBUS_SERVICE, "org.nexus.DE");
        assert_eq!(NEXUS_DBUS_PATH, "/org/nexus/DE");
        assert_eq!(NEXUS_DBUS_INTERFACE, "org.nexus.DE");
    }
}
