//! Unit tests for the node configuration loader (`config.rs`) of `qublis-qnetx-node`.

use crate::config::{NodeConfig, ConsensusConfig, MetricsConfig, TelemetryConfig, LoggingConfig};
use crate::error::NodeError;
use tempfile::NamedTempFile;
use std::{fs, io::Write, path::PathBuf};

const VALID_CONFIG: &str = r#"
node_name       = "validator-1"
listen_addr     = "/ip4/0.0.0.0/tcp/30333"
jsonrpc_addr    = "/ip4/0.0.0.0/tcp/9944"
bootstrap_peers = [ "/ip4/1.2.3.4/tcp/30333", "/ip4/5.6.7.8/tcp/30333" ]

[consensus]
qmesh_config_path     = "qmesh.toml"
neuroflux_enabled     = true
neuroflux_config_path = "neuroflux.toml"

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
    let mut file = NamedTempFile::new().expect("create temp file");
    write!(file, "{}", VALID_CONFIG).expect("write config");

    let cfg = NodeConfig::load(file.path()).expect("load should succeed");

    assert_eq!(cfg.node_name, "validator-1");
    assert_eq!(cfg.listen_addr, "/ip4/0.0.0.0/tcp/30333");
    assert_eq!(cfg.jsonrpc_addr.as_deref(), Some("/ip4/0.0.0.0/tcp/9944"));
    assert_eq!(
        cfg.bootstrap_peers,
        vec![
            "/ip4/1.2.3.4/tcp/30333".to_string(),
            "/ip4/5.6.7.8/tcp/30333".to_string()
        ]
    );

    // Consensus
    let cons = &cfg.consensus;
    assert_eq!(cons.qmesh_config_path, "qmesh.toml");
    assert!(cons.neuroflux_enabled);
    assert_eq!(cons.neuroflux_config_path.as_deref(), Some("neuroflux.toml"));

    // Metrics
    let met = &cfg.metrics;
    assert_eq!(met.port, 9400);
    assert!(met.enabled);

    // Telemetry
    let tel = &cfg.telemetry;
    assert_eq!(tel.prometheus_bind, "0.0.0.0:9300");

    // Logging
    let log = &cfg.logging;
    assert_eq!(log.level, "info");
    assert_eq!(log.format, "json");

    assert_eq!(cfg.base_path.to_str().unwrap(), "/var/lib/qublis/qnetx-node");
    assert!(!cfg.dev_mode);
}

#[test]
fn missing_optional_fields_default() {
    // Remove jsonrpc_addr and neuroflux_config_path
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
dev_mode  = true
"#;
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", toml).unwrap();

    let cfg = NodeConfig::load(file.path()).expect("load should succeed");
    // Optional fields not present should be None
    assert!(cfg.jsonrpc_addr.is_none());
    assert!(cfg.consensus.neuroflux_config_path.is_none());
}

#[test]
fn invalid_toml_errors_config() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "not = valid = toml").unwrap();

    match NodeConfig::load(file.path()).unwrap_err() {
        NodeError::Config(_) => {}
        other => panic!("expected Config error, got {:?}", other),
    }
}

#[test]
fn missing_file_errors_io() {
    let missing_path = PathBuf::from("/nonexistent/config.toml");
    match NodeConfig::load(&missing_path).unwrap_err() {
        NodeError::Io(_) => {}
        other => panic!("expected Io error, got {:?}", other),
    }
}
