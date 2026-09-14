use crate::state::SharedState;
use std::time::Duration;
use sysinfo::System;
use tokio::time::sleep;
use tracing::debug;

pub async fn run_metrics_collector(state: SharedState) {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu = sys.global_cpu_usage();
        let total_mem = sys.total_memory() / (1024 * 1024); // MB
        let used_mem = sys.used_memory() / (1024 * 1024); // MB
        let mem_pct = if total_mem > 0 {
            (used_mem as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        };

        {
            let mut lock = state.write().await;
            lock.metrics.cpu_usage = cpu;
            lock.metrics.memory_used_mb = used_mem;
            lock.metrics.memory_total_mb = total_mem;
            lock.metrics.memory_pct = mem_pct;
        }

        debug!(
            "Metrics: CPU: {:.1}%, Memory: {}MB / {}MB ({:.1}%)",
            cpu, used_mem, total_mem, mem_pct
        );

        sleep(Duration::from_secs(2)).await;
    }
}
