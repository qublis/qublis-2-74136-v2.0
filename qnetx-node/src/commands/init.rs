//! Implementation of the `init` subcommand for the QNetX validator node.
//!
//! Loads configuration, initializes on-disk directories, writes config, and
//! generates identity keys.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::{
    config::NodeConfig,
    error::NodeError,
    bootstrap,
};

/// Execute the `init` command.
///
/// # Arguments
///
/// * `config_path` – path to the node's TOML configuration file.
/// * `base_path`   – directory where node data (state, keys, etc.) will be created.
pub fn execute(config_path: PathBuf, base_path: PathBuf) -> Result<(), NodeError> {
    // 1. Load and validate the node configuration
    let cfg = NodeConfig::load(&config_path)?;

    // 2. Bootstrap the node: create directories, write config.toml, generate keys
    bootstrap::init(&cfg, &base_path)?;

    // 3. Inform the user
    println!("Initialization complete. Node data and keys created at '{}'.", base_path.display());

    Ok(())
}
