//! End-to-end integration tests for the `qublis-qnetx-node` binary.
//!
//! These tests invoke the compiled CLI binary via `assert_cmd` to verify
//! that the `init`, `--help`, and other commands behave as expected.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::{fs, path::PathBuf};

/// Helper: write a minimal valid TOML config into `path`.
fn write_minimal_config(path: &PathBuf, base_path: &PathBuf) {
    let toml = format!(r#"
node_name       = "test-node"
listen_addr     = "/ip4/0.0.0.0/tcp/30333"
bootstrap_peers = []

[consensus]
qmesh_config_path     = "qmesh.toml"
neuroflux_enabled     = false

[metrics]
port    = 9200
enabled = true

[telemetry]
prometheus_bind = "0.0.0.0:9301"

[logging]
level  = "debug"
format = "plain"

base_path = "{}"
dev_mode  = true
"#, base_path.display());
    fs::write(path, toml).expect("failed to write config.toml");
}

#[test]
fn help_shows_usage_and_version() {
    let mut cmd = Command::cargo_bin("qublis-qnetx-node").unwrap();
    cmd.arg("--help")
       .assert()
       .success()
       .stdout(predicate::str::contains("Extended QNetX validator node"))
       .stdout(predicate::str::contains("USAGE"));
}

#[test]
fn init_command_creates_directories_and_files() {
    // Create a temp workspace
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("config.toml");
    let data_dir = tmp.path().join("node-data");

    // Write minimal config (base_path is placeholder)
    write_minimal_config(&config_path, &data_dir);

    // Run `init`
    let mut cmd = Command::cargo_bin("qublis-qnetx-node").unwrap();
    cmd.arg("init")
       .arg("--config")
       .arg(&config_path)
       .arg("--base-path")
       .arg(&data_dir)
       .assert()
       .success()
       .stdout(predicate::str::contains("Initialization successful"));

    // Verify on-disk structure
    assert!(data_dir.exists(), "base_path was created");
    assert!(data_dir.join("data").is_dir(), "`data/` directory exists");
    assert!(data_dir.join("keys/identity.key").is_file(), "identity.key generated");
    assert!(data_dir.join("config.toml").is_file(), "config.toml was written");
}

#[test]
fn status_command_errors_gracefully_when_not_running() {
    // Even if the node isn't running, status should load config and then error on missing state
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("config.toml");
    let data_dir = tmp.path().join("node-data");

    write_minimal_config(&config_path, &data_dir);

    let mut cmd = Command::cargo_bin("qublis-qnetx-node").unwrap();
    cmd.arg("status")
       .arg("--config")
       .arg(&config_path)
       .arg("--base-path")
       .arg(&data_dir)
       .assert()
       .failure()
       .stderr(predicate::str::contains("Error"));
}
