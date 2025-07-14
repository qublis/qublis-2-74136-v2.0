//! QNetX Node Bootstrap for Qublis v2.0
//!
//! Reads a TOML configuration for a QNetX node, prepares the node directory,
//! writes the ChainSpec, and launches the `qublis-qnetx-node` process with the
//! specified parameters. Designed for asynchronous execution via Tokio.

use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;
use tokio::process::Command;

/// Configuration for bootstrapping a QNetX node.
#[derive(Debug, Deserialize)]
pub struct BootstrapConfig {
    /// Unique name for this node (used for directories and logging).
    pub node_name: String,

    /// Multiaddr on which the node should listen (e.g. "/ip4/0.0.0.0/tcp/30333").
    pub listen_addr: String,

    /// List of peer multiaddrs to bootstrap from.
    pub bootstrap_peers: Vec<String>,

    /// Path to the ChainSpec file to use.
    pub chain_spec: String,

    /// Base directory under which to create the node data (e.g. "./nodes").
    #[serde(default = "default_base_path")]
    pub base_path: String,

    /// Whether to launch in dev mode (single validator, local).
    #[serde(default)]
    pub dev_mode: bool,
}

fn default_base_path() -> String {
    "./qnetx_nodes".into()
}

/// Errors returned by the QNetX node bootstrapper.
#[derive(Debug, Error)]
pub enum BootstrapError {
    /// I/O error reading config or writing files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),

    /// Failed to launch the node process.
    #[error("node launch failed: {0}")]
    Launch(String),
}

/// Entry point for the `qnetx_node_bootstrap` command.
///
/// # Arguments
///
/// * `config_path` - Path to the TOML file defining `BootstrapConfig`.
///
pub async fn run(config_path: &str) -> Result<(), BootstrapError> {
    // Load and parse the bootstrap config
    let toml_str = fs::read_to_string(config_path)?;
    let cfg: BootstrapConfig = toml::from_str(&toml_str)?;
    println!("Bootstrapping QNetX node: {:#?}", cfg);

    // Prepare node directory
    let node_dir = Path::new(&cfg.base_path).join(&cfg.node_name);
    tokio::fs::create_dir_all(&node_dir).await?;

    // Copy or link the ChainSpec into node directory
    let spec_dest = node_dir.join("chainspec.json");
    fs::copy(&cfg.chain_spec, &spec_dest)?;

    // Build command arguments
    let mut args = vec![
        "--base-path".into(), node_dir.to_string_lossy().into_owned(),
        "--chain".into(), spec_dest.to_string_lossy().into_owned(),
        "--listen-addr".into(), cfg.listen_addr.clone(),
    ];

    // Bootstrap peers
    for peer in &cfg.bootstrap_peers {
        args.push("--bootnode".into());
        args.push(peer.clone());
    }

    // Dev mode flag
    if cfg.dev_mode {
        args.push("--dev".into());
    }

    println!("Launching qublis-qnetx-node with args: {:?}", args);

    // Spawn the node process
    let status = Command::new("qublis-qnetx-node")
        .args(&args)
        .status()
        .await
        .map_err(|e| BootstrapError::Launch(format!("failed to spawn process: {}", e)))?;

    if !status.success() {
        return Err(BootstrapError::Launch(format!(
            "process exited with code {:?}",
            status.code()
        )));
    }

    println!("Node '{}' launched successfully.", cfg.node_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::runtime::Runtime;
    use std::io::Write;

    #[test]
    fn parse_minimal_config() {
        let toml = r#"
            node_name = "alpha"
            listen_addr = "/ip4/127.0.0.1/tcp/30333"
            bootstrap_peers = []
            chain_spec = "spec.json"
        "#;
        let cfg: BootstrapConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.node_name, "alpha");
        assert_eq!(cfg.listen_addr, "/ip4/127.0.0.1/tcp/30333");
        assert!(cfg.bootstrap_peers.is_empty());
        assert_eq!(cfg.chain_spec, "spec.json");
        assert_eq!(cfg.base_path, "./qnetx_nodes");
        assert!(!cfg.dev_mode);
    }

    #[test]
    fn run_errors_on_missing_config() {
        let rt = Runtime::new().unwrap();
        let res = rt.block_on(run("nonexistent.toml"));
        assert!(matches!(res, Err(BootstrapError::Io(_))));
    }

    #[test]
    fn run_errors_on_invalid_toml() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "not = valid = toml").unwrap();
        let rt = Runtime::new().unwrap();
        let res = rt.block_on(run(file.path().to_str().unwrap()));
        assert!(matches!(res, Err(BootstrapError::Parse(_))));
    }
}
