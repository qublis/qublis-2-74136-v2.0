//! Bootstrap module for the `qublis-qnetx-node`.
//!
//! Creates the on-disk directory structure, writes out the node config,
//! and generates a placeholder identity key.  

use crate::config::NodeConfig;
use crate::error::NodeError;
use std::{fs, io::Write, path::Path};

/// Initialize a new QNetX node at `base_path` using the provided `cfg`.
///
/// This will:
/// 1. Create `base_path/`, `base_path/data/`, and `base_path/keys/`.
/// 2. Serialize and write the TOML config to `base_path/config.toml`.
/// 3. Generate a placeholder 32-byte identity key at `base_path/keys/identity.key`.
pub fn init(cfg: &NodeConfig, base_path: &Path) -> Result<(), NodeError> {
    // 1. Create directories
    fs::create_dir_all(base_path)?;
    let data_dir = base_path.join("data");
    let keys_dir = base_path.join("keys");
    fs::create_dir_all(&data_dir)?;
    fs::create_dir_all(&keys_dir)?;

    // 2. Serialize & write config.toml
    let toml_str = toml::to_string_pretty(&cfg)
        .map_err(|e| NodeError::Other(format!("Config serialization error: {}", e)))?;
    fs::write(base_path.join("config.toml"), toml_str)?;

    // 3. Generate placeholder identity key (32 zero bytes for now)
    //    TODO: replace with real crypto keypair generation
    let identity_key = [0u8; 32];
    let mut key_file = fs::File::create(keys_dir.join("identity.key"))?;
    key_file.write_all(&identity_key)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn make_dummy_cfg() -> NodeConfig {
        NodeConfig {
            node_name: "test-node".into(),
            listen_addr: "/ip4/0.0.0.0/tcp/30333".into(),
            jsonrpc_addr: None,
            bootstrap_peers: vec![],
            consensus: Default::default(),
            metrics: Default::default(),
            telemetry: Default::default(),
            logging: Default::default(),
            base_path: Path::new("/tmp").into(),
            dev_mode: true,
        }
    }

    #[test]
    fn init_creates_dirs_and_files() {
        let tmp = tempdir().unwrap();
        let cfg = make_dummy_cfg();
        init(&cfg, tmp.path()).unwrap();

        assert!(tmp.path().join("data").is_dir());
        assert!(tmp.path().join("keys").is_dir());
        assert!(tmp.path().join("config.toml").is_file());
        assert!(tmp.path().join("keys/identity.key").is_file());

        // config.toml should parse back
        let toml_data = fs::read_to_string(tmp.path().join("config.toml")).unwrap();
        assert!(toml_data.contains("node_name = \"test-node\""));
    }
}
