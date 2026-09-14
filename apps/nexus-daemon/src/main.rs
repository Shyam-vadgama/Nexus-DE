use nexus_core::config::NexusConfig;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!(
        "Starting NEXUS Desktop Daemon v{}",
        env!("CARGO_PKG_VERSION")
    );

    let config_path = NexusConfig::default_config_path();
    info!("Loading config from: {}", config_path.display());
    let config = NexusConfig::load_from_file(&config_path).unwrap_or_default();
    info!("Active Tier: {}", config.general.tier.label());

    // Check if running in one-shot dry-run / verification mode
    if std::env::args().any(|arg| arg == "--test-run" || arg == "--version") {
        info!("Verification run successful. Exiting cleanly.");
        return Ok(());
    }

    info!("nexus-daemon initialized successfully.");
    Ok(())
}
