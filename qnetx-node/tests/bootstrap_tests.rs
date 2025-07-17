//! Integration tests for the `bootstrap` module of the `qublis-qnetx-node` crate.
//!
//! Verifies that `bootstrap::init` correctly creates directories, writes the
//! configuration file, and generates an identity key.

use std::{fs, io::Read, path::PathBuf};
use tempfile::tempdir;

use qublis_qnetx_node::{bootstrap, config::NodeConfig, error::NodeError};

/// Helper to construct a minimal `NodeConfig` pointing at the given base path.
fn make_dummy_config(base_path: PathBuf) -> NodeConfig {
    NodeConfig {
        node_name: "test-node".into(),
        listen_addr: "/ip4/0.0.0.0/tcp/30333".into(),
        jsonrpc_addr: None,
        bootstrap_peers: vec!["/ip4/1.2.3.4/tcp/30333".into()],
        consensus: Default::default(),
        metrics: Default::default(),
        telemetry: Default::default(),
        logging: Default::default(),
        base_path,
        dev_mode: true,
    }
}

#[test]
fn init_creates_directory_structure() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("node-data");
    let cfg = make_dummy_config(base.clone());

    // Should succeed and create dirs
    bootstrap::init(&cfg, &base).expect("bootstrap init failed");

    assert!(base.is_dir(), "base path directory was not created");
    assert!(base.join("data").is_dir(), "`data/` directory missing");
    assert!(base.join("keys").is_dir(), "`keys/` directory missing");
}

#[test]
fn init_writes_config_toml() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("node-data");
    let cfg = make_dummy_config(base.clone());

    bootstrap::init(&cfg, &base).unwrap();

    let cfg_path = base.join("config.toml");
    assert!(cfg_path.is_file(), "config.toml was not written");

    // Read and ensure it contains the correct node_name field
    let mut contents = String::new();
    fs::File::open(&cfg_path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    assert!(
        contents.contains("node_name = \"test-node\""),
        "config.toml does not contain the expected content"
    );
}

#[test]
fn init_generates_identity_key() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("node-data");
    let cfg = make_dummy_config(base.clone());

    bootstrap::init(&cfg, &base).unwrap();

    let key_path = base.join("keys/identity.key");
    assert!(key_path.is_file(), "identity.key was not created");

    // The placeholder key is 32 zero bytes
    let data = fs::read(&key_path).unwrap();
    assert_eq!(data.len(), 32, "identity.key length is not 32 bytes");
    assert!(data.iter().all(|&b| b == 0), "identity.key is not all zeros");
}

#[test]
fn init_fails_if_base_is_file() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("not-a-dir");
    // Create a file where the directory should be
    fs::write(&base, b"hello").unwrap();
    let cfg = make_dummy_config(base.clone());

    let err = bootstrap::init(&cfg, &base).unwrap_err();
    match err {
        NodeError::Io(_) => {} // expected
        _ => panic!("Expected Io error when base path is a file, got {:?}", err),
    }
}
