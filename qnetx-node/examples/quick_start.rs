//! examples/quick_start.rs
//!
//! A minimal quickstart demonstrating how to initialize and run
//! a Qublis v2.0 QNetX validator node programmatically.
//!
//! Assumes you have a `config/default.toml` in your working directory.

use std::path::PathBuf;
use qublis_qnetx_node::{
    config::NodeConfig,
    bootstrap,
    node,
    error::NodeError,
};

#[tokio::main]
async fn main() -> Result<(), NodeError> {
    // 1. Define paths to the config file and data directory
    let config_path = PathBuf::from("config/default.toml");
    let base_path   = PathBuf::from("./data/node1");

    // 2. Load and validate node configuration
    let cfg = NodeConfig::load(&config_path)?;
    println!("Loaded node config: {:?}", cfg);

    // 3. Initialize the node (creates directories, writes config, generates keys)
    bootstrap::init(&cfg, &base_path)?;
    println!("Node initialized at {}", base_path.display());

    // 4. Run the validator node (P2P, consensus, NeuroFlux, telemetry, metrics)
    println!("Starting node...");
    node::run(&cfg, &base_path).await?;

    Ok(())
}
