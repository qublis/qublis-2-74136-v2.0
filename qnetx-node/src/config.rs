//! Node configuration loader for `qublis-qnetx-node`.
//!
//! Defines `NodeConfig` and nested sections matching `config/default.toml`,
//! and provides a `load` method to parse a TOML file into `NodeConfig`.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};
use crate::error::NodeError;

/// Top‐level node configuration.
#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    /// Human‐readable name for this node instance.
    pub node_name: String,

    /// Multiaddr to listen on for QNetX P2P connections.
    pub listen_addr: String,

    /// Optional multiaddr to listen on for JSON-RPC requests.
    #[serde(default)]
    pub jsonrpc_addr: Option<String>,

    /// List of bootstrap peers (multiaddrs) to connect to at startup.
    pub bootstrap_peers: Vec<String>,

    /// Consensus (QMesh + NeuroFlux) configuration.
    pub consensus: ConsensusConfig,

    /// Prometheus metrics export configuration.
    pub metrics: MetricsConfig,

    /// Telemetry / Prometheus scrape endpoint configuration.
    pub telemetry: TelemetryConfig,

    /// Logging configuration.
    pub logging: LoggingConfig,

    /// Base directory for node data (chain state, keys, keystore).
    pub base_path: PathBuf,

    /// Development mode flag: disables peer auth, uses dev keys, etc.
    pub dev_mode: bool,
}

/// Consensus section of the node config.
#[derive(Debug, Deserialize)]
pub struct ConsensusConfig {
    /// Path to the QMesh consensus TOML config.
    pub qmesh_config_path: String,

    /// Enable or disable NeuroFlux RL optimization.
    pub neuroflux_enabled: bool,

    /// Path to the NeuroFlux config (if `neuroflux_enabled`).
    #[serde(default)]
    pub neuroflux_config_path: Option<String>,
}

/// Metrics section of the node config.
#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
    /// TCP port for Prometheus metrics export.
    pub port: u16,

    /// Globally enable or disable the metrics endpoint.
    pub enabled: bool,
}

/// Telemetry section of the node config.
#[derive(Debug, Deserialize)]
pub struct TelemetryConfig {
    /// Bind address for Prometheus scrape endpoint (e.g. "0.0.0.0:9300").
    pub prometheus_bind: String,
}

/// Logging section of the node config.
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    /// Minimum log level: trace, debug, info, warn, error.
    pub level: String,

    /// Output format: "plain" or "json".
    pub format: String,
}

impl NodeConfig {
    /// Load and parse a `NodeConfig` from the given TOML file path.
    ///
    /// # Errors
    ///
    /// Returns `NodeError::Io` if the file cannot be read, or
    /// `NodeError::Config` if the TOML is invalid.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, NodeError> {
        let s = fs::read_to_string(&path)?;
        let cfg: NodeConfig = toml::from_str(&s)?;
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    const VALID_TOML: &str = r#"
        node_name       = "validator-1"
        listen_addr     = "/ip4/0.0.0.0/tcp/30333"
        jsonrpc_addr    = "/ip4/0.0.0.0/tcp/9944"
        bootstrap_peers = [ "/ip4/1.2.3.4/tcp/30333" ]

        [consensus]
        qmesh_config_path     = "../qmesh/config/default.toml"
        neuroflux_enabled     = true
        neuroflux_config_path = "../ci_core/config/neuroflux.toml"

        [metrics]
        port    = 9400
        enabled = true

        [telemetry]
        prometheus_bind = "0.0.0.0:9300"

        [logging]
        level  = "info"
        format = "json"

        base_path = "/var/lib/qublis/qnetx-node"
        dev_mode  = false
    "#;

    #[test]
    fn load_valid_config() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", VALID_TOML).unwrap();

        let cfg = NodeConfig::load(file.path()).unwrap();
        assert_eq!(cfg.node_name, "validator-1");
        assert_eq!(cfg.listen_addr, "/ip4/0.0.0.0/tcp/30333");
        assert_eq!(cfg.jsonrpc_addr.as_deref(), Some("/ip4/0.0.0.0/tcp/9944"));
        assert_eq!(cfg.bootstrap_peers, vec!["/ip4/1.2.3.4/tcp/30333"]);
        assert!(cfg.consensus.neuroflux_enabled);
        assert_eq!(
            cfg.consensus.neuroflux_config_path.as_deref(),
            Some("../ci_core/config/neuroflux.toml")
        );
        assert_eq!(cfg.metrics.port, 9400);
        assert!(cfg.metrics.enabled);
        assert_eq!(cfg.telemetry.prometheus_bind, "0.0.0.0:9300");
        assert_eq!(cfg.logging.level, "info");
        assert_eq!(cfg.logging.format, "json");
        assert_eq!(
            cfg.base_path.to_str().unwrap(),
            "/var/lib/qublis/qnetx-node"
        );
        assert!(!cfg.dev_mode);
    }

    #[test]
    fn missing_optional_fields_default() {
        let toml = r#"
            node_name       = "n"
            listen_addr     = "l"
            bootstrap_peers = []
            [consensus]
            qmesh_config_path = "qmesh.toml"
            neuroflux_enabled = false
            [metrics]
            port = 1234
            enabled = false
            [telemetry]
            prometheus_bind = "0.0.0.0:9300"
            [logging]
            level = "debug"
            format = "plain"
            base_path = "/tmp"
            dev_mode = true
        "#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml).unwrap();

        let cfg = NodeConfig::load(file.path()).unwrap();
        // jsonrpc_addr should default to None
        assert!(cfg.jsonrpc_addr.is_none());
        // consensus.neuroflux_config_path should default to None
        assert!(cfg.consensus.neuroflux_config_path.is_none());
    }

    #[test]
    fn invalid_toml_errors_config() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "not = valid = toml").unwrap();
        let err = NodeConfig::load(file.path()).unwrap_err();
        match err {
            NodeError::Config(_) => {}
            _ => panic!("expected NodeError::Config"),
        }
    }

    #[test]
    fn missing_file_errors_io() {
        let err = NodeConfig::load("does_not_exist.toml").unwrap_err();
        match err {
            NodeError::Io(_) => {}
            _ => panic!("expected NodeError::Io"),
        }
    }
}
