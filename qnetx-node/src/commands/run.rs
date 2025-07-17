//! Implementation of the `run` subcommand for the QNetX validator node.
//!
//! Loads configuration, starts telemetry & metrics servers, and launches
//! the core node runtime (P2P networking, consensus, NeuroFlux, etc.).

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::{
    config::NodeConfig,
    error::NodeError,
    metrics,
    node,
    telemetry,
};

/// Execute the `run` command.
///
/// # Arguments
///
/// * `config_path` – path to the node's TOML configuration file.
/// * `base_path`   – directory where node data (state, keys, etc.) are stored.
pub async fn execute(config_path: PathBuf, base_path: PathBuf) -> Result<(), NodeError> {
    // 1. Load and validate configuration
    let cfg = NodeConfig::load(&config_path)?;

    // 2. Start telemetry HTTP endpoint for Prometheus scrapes
    telemetry::start(&cfg.telemetry)?;

    // 3. Start metrics export (in-memory collector); bind if needed
    metrics::start(&cfg.metrics)?;

    // 4. Launch the core node runtime
    node::run(&cfg, &base_path).await?;

    Ok(())
}
