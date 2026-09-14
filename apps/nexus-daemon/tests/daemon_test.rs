use nexus_core::config::NexusConfig;
use nexus_core::NexusTier;

#[test]
fn test_config_tier_switching() {
    let mut config = NexusConfig::default();
    assert_eq!(config.general.tier, NexusTier::Core);

    config.general.tier = NexusTier::Hyper;
    assert_eq!(config.general.tier, NexusTier::Hyper);
    assert!(config.general.tier.has_glass_effects());
    assert!(config.general.tier.has_control_center());

    config.general.tier = NexusTier::Minimal;
    assert_eq!(config.general.tier, NexusTier::Minimal);
    assert!(!config.general.tier.has_glass_effects());
    assert!(!config.general.tier.has_control_center());
}

#[tokio::test]
async fn test_daemon_cli_run() {
    // Test that invoking the binary with --test-run completes with success
    let output = tokio::process::Command::new(env!("CARGO_BIN_EXE_nexus-daemon"))
        .arg("--test-run")
        .output()
        .await
        .expect("Failed to run nexus-daemon --test-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}\n{}", stdout, stderr);
    assert!(combined.contains("Verification test-run succeeded"));
}
