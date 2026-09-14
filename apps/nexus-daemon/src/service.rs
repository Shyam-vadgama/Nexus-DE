use crate::state::SharedState;
use nexus_core::theme::NexusPalette;
use nexus_core::NexusTier;
use zbus::interface;
use zbus::object_server::SignalContext;

pub struct NexusDeService {
    state: SharedState,
}

impl NexusDeService {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

#[interface(name = "org.nexus.DE")]
impl NexusDeService {
    /// Get active desktop tier (1: Minimal, 2: Core, 3: Hyper)
    async fn get_tier(&self) -> u8 {
        let lock = self.state.read().await;
        lock.tier().to_u8()
    }

    /// Set active desktop tier and emit signal
    async fn set_tier(
        &self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        tier: u8,
    ) -> zbus::fdo::Result<bool> {
        let new_tier = NexusTier::from_u8(tier)
            .ok_or_else(|| zbus::fdo::Error::InvalidArgs(format!("Invalid tier: {}", tier)))?;

        {
            let mut lock = self.state.write().await;
            lock.set_tier(new_tier);
        }

        Self::tier_changed(&ctxt, tier).await?;
        Ok(true)
    }

    /// Get current configuration as JSON string
    async fn get_config(&self) -> String {
        let lock = self.state.read().await;
        serde_json::to_string(&lock.config).unwrap_or_else(|_| "{}".to_string())
    }

    /// Get current Material You dynamic palette as JSON string
    async fn get_palette(&self) -> String {
        let lock = self.state.read().await;
        serde_json::to_string(&lock.palette).unwrap_or_else(|_| "{}".to_string())
    }

    /// Set active wallpaper, extract palette, save, and emit signal
    async fn set_wallpaper(
        &self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        path: String,
    ) -> zbus::fdo::Result<bool> {
        let new_palette = NexusPalette::from_wallpaper(&path)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Failed to parse wallpaper: {}", e)))?;

        let palette_json = serde_json::to_string(&new_palette).unwrap_or_default();

        {
            let mut lock = self.state.write().await;
            lock.config.appearance.wallpaper_path = Some(path);
            lock.palette = new_palette;
            let config_path = nexus_core::config::NexusConfig::default_config_path();
            let _ = lock.config.save_to_file(config_path);
        }

        Self::theme_changed(&ctxt, &palette_json).await?;
        Ok(true)
    }

    /// Get real-time system metrics as JSON string
    async fn get_system_metrics(&self) -> String {
        let lock = self.state.read().await;
        serde_json::to_string(&lock.metrics).unwrap_or_else(|_| "{}".to_string())
    }

    /// Signal emitted when the desktop tier changes
    #[zbus(signal)]
    pub async fn tier_changed(ctxt: &SignalContext<'_>, tier: u8) -> zbus::Result<()>;

    /// Signal emitted when the color palette changes
    #[zbus(signal)]
    pub async fn theme_changed(ctxt: &SignalContext<'_>, palette_json: &str) -> zbus::Result<()>;

    /// Signal emitted when system metrics update
    #[zbus(signal)]
    pub async fn metrics_updated(ctxt: &SignalContext<'_>, metrics_json: &str) -> zbus::Result<()>;

    /// Signal emitted when configuration is reloaded
    #[zbus(signal)]
    pub async fn config_reloaded(ctxt: &SignalContext<'_>) -> zbus::Result<()>;
}
