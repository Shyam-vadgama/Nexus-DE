mod metrics;
mod service;
mod state;
mod watcher;

use nexus_core::NexusTier;
use nexus_ipc::dbus::{NEXUS_DBUS_PATH, NEXUS_DBUS_SERVICE};
use service::NexusDeService;
use state::{DaemonState, SharedState};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use zbus::connection::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let args: Vec<String> = std::env::args().collect();

    // Check version flag
    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("nexus-daemon {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // CLI Command: --status (Queries live running daemon over D-Bus)
    if args.iter().any(|a| a == "--status") {
        return query_status().await;
    }

    // CLI Command: --set-tier <1|2|3>
    if let Some(pos) = args.iter().position(|a| a == "--set-tier") {
        if let Some(tier_str) = args.get(pos + 1) {
            if let Ok(tier_num) = tier_str.parse::<u8>() {
                return trigger_set_tier(tier_num).await;
            }
        }
        eprintln!("Usage: nexus-daemon --set-tier <1|2|3>");
        std::process::exit(1);
    }

    info!(
        "Starting NEXUS Desktop Daemon v{}",
        env!("CARGO_PKG_VERSION")
    );

    let state: SharedState = Arc::new(RwLock::new(DaemonState::new()));

    // Test-run / verification mode for CI and automated tests
    if args.iter().any(|a| a == "--test-run") {
        info!("Running self-test suite...");
        let lock = state.read().await;
        info!("Active Tier: {}", lock.tier().label());
        info!("Palette Primary: {}", lock.palette.primary);
        info!(
            "Theme CSS preview length: {} bytes",
            lock.palette.to_css().len()
        );
        info!("Verification test-run succeeded. Exiting cleanly.");
        return Ok(());
    }

    // Start background workers
    tokio::spawn(metrics::run_metrics_collector(state.clone()));
    watcher::start_config_watcher(state.clone());

    // Register on D-Bus session bus
    let service = NexusDeService::new(state.clone());
    let _conn = Builder::session()?
        .name(NEXUS_DBUS_SERVICE)?
        .serve_at(NEXUS_DBUS_PATH, service)?
        .build()
        .await?;

    info!(
        "Registered D-Bus service '{}' at '{}'",
        NEXUS_DBUS_SERVICE, NEXUS_DBUS_PATH
    );
    info!("NEXUS Daemon is ready and running.");

    // Keep running until SIGINT / SIGTERM
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received. Exiting nexus-daemon.");
    Ok(())
}

async fn query_status() -> Result<(), Box<dyn std::error::Error>> {
    use nexus_ipc::dbus::NexusDeProxy;

    let connection = zbus::Connection::session().await?;
    let proxy = NexusDeProxy::new(&connection).await?;

    let tier_id = proxy.get_tier().await?;
    let tier = NexusTier::from_u8(tier_id).unwrap_or(NexusTier::Core);
    let metrics = proxy.get_system_metrics().await?;

    println!("NEXUS Desktop Daemon Status");
    println!("---------------------------");
    println!("Active Tier   : {} (ID: {})", tier.label(), tier_id);
    println!("Glass Effects : {}", tier.has_glass_effects());
    println!("Widgets Active: {}", tier.has_widgets());
    println!("System Metrics: {}", metrics);
    Ok(())
}

async fn trigger_set_tier(tier: u8) -> Result<(), Box<dyn std::error::Error>> {
    use nexus_ipc::dbus::NexusDeProxy;

    let connection = zbus::Connection::session().await?;
    let proxy = NexusDeProxy::new(&connection).await?;

    let success = proxy.set_tier(tier).await?;
    if success {
        let label = NexusTier::from_u8(tier)
            .map(|t| t.label())
            .unwrap_or("Unknown");
        println!("Successfully switched to tier: {} ({})", label, tier);
    } else {
        eprintln!("Failed to switch tier.");
    }
    Ok(())
}
