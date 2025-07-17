//! Implementation of the `status` subcommand for the QNetX validator node.
//!
//! Loads configuration, opens the existing node data directory, and queries
//! the running node (via JSON-RPC or local API) for current status metrics.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;
use crate::{
    config::NodeConfig,
    error::NodeError,
    node,
};

/// Execute the `status` command.
///
/// # Arguments
///
/// * `config_path` – path to the node's TOML configuration file.
/// * `base_path`   – directory where node data (state, keys, etc.) is stored.
pub fn execute(config_path: PathBuf, base_path: PathBuf) -> Result<(), NodeError> {
    // 1. Load and validate configuration
    let cfg = NodeConfig::load(&config_path)?;

    // 2. Query node status
    //    This could be via direct in-process API if the node is linked,
    //    or via JSON-RPC if running as a separate process.
    let status = node::status(&cfg, &base_path)?;

    // 3. Pretty-print the status to stdout
    println!("Node Status:");
    println!("  Name           : {}", cfg.node_name);
    println!("  Listening on   : {}", cfg.listen_addr);
    println!("  Peers connected: {}", status.peers_connected);
    println!("  Tip count      : {}", status.tip_count);
    println!("  Measured TPS   : {:.2}", status.measured_tps);
    println!("  Avg. latency   : {:.2} ms", status.avg_latency_ms);
    println!("  Fork rate      : {:.2}%", status.fork_rate * 100.0);
    println!("  Uptime         : {} seconds", status.uptime_secs);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use crate::config::NodeConfig;
    use crate::node::NodeStatus;

    /// Creates a dummy status in process.
    fn setup_dummy_status(dir: &PathBuf) {
        // For testing, we simulate a status storage file.
        let status_file = dir.join("status.json");
        let status = serde_json::to_string(&NodeStatus {
            peers_connected: 4,
            tip_count: 12,
            measured_tps: 1500.0,
            avg_latency_ms: 85.0,
            fork_rate: 0.01,
            uptime_secs: 3600,
        }).unwrap();
        fs::create_dir_all(dir).unwrap();
        let mut f = File::create(&status_file).unwrap();
        f.write_all(status.as_bytes()).unwrap();
    }

    #[test]
    fn execute_prints_status() {
        let tmp = tempdir().unwrap();
        let base_path = tmp.path().join("node");
        let cfg_toml = tmp.path().join("config.toml");

        // Write minimal config
        let cfg = NodeConfig {
            node_name: "test-node".into(),
            listen_addr: "/ip4/0.0.0.0/tcp/30333".into(),
            jsonrpc_addr: None,
            bootstrap_peers: vec![],
            consensus: Default::default(),
            metrics: Default::default(),
            telemetry: Default::default(),
            logging: Default::default(),
            base_path: base_path.clone(),
            dev_mode: true,
        };
        let toml_str = toml::to_string(&cfg).unwrap();
        fs::write(&cfg_toml, toml_str).unwrap();

        // Setup dummy status
        setup_dummy_status(&base_path);

        // Capture stdout
        let output = std::panic::catch_unwind(|| {
            execute(cfg_toml.clone(), base_path.clone()).unwrap();
        });
        assert!(output.is_ok());
        // (Further assertions could capture and check printed lines)
    }
}
