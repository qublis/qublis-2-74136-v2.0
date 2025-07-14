//! QNet Configuration
//!
//! Defines the `QNetConfig` struct for routing, teleportation, and metrics settings,
//! with support for loading from a TOML file.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Default number of candidate paths to superpose.
fn default_k_paths() -> usize {
    4
}

/// QNet configuration parameters.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QNetConfig {
    /// Number of shortest paths to include in the routing superposition.
    #[serde(default = "default_k_paths")]
    pub k_paths: usize,

    /// Enable the teleportation overlay.
    #[serde(default)]
    pub enable_teleport: bool,

    /// Enable collection of routing & relay metrics.
    #[serde(default)]
    pub enable_metrics: bool,
}

impl Default for QNetConfig {
    fn default() -> Self {
        QNetConfig {
            k_paths: default_k_paths(),
            enable_teleport: false,
            enable_metrics: false,
        }
    }
}

/// Errors that can occur while loading a `QNetConfig`.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// I/O error reading the config file.
    #[error("I/O error reading config file: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl QNetConfig {
    /// Load `QNetConfig` from the given TOML file path.
    ///
    /// If the file does not exist or is invalid, returns an error.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let cfg = toml::from_str(&content)?;
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn default_config_values() {
        let cfg = QNetConfig::default();
        assert_eq!(cfg.k_paths, 4);
        assert!(!cfg.enable_teleport);
        assert!(!cfg.enable_metrics);
    }

    #[test]
    fn load_config_from_toml() {
        let mut file = NamedTempFile::new().expect("create temp file");
        let toml = r#"
            k_paths = 7
            enable_teleport = true
            enable_metrics = true
        "#;
        fs::write(file.path(), toml).expect("write TOML");
        let cfg = QNetConfig::load(file.path()).expect("load config");
        assert_eq!(cfg.k_paths, 7);
        assert!(cfg.enable_teleport);
        assert!(cfg.enable_metrics);
    }

    #[test]
    fn error_on_invalid_toml() {
        let mut file = NamedTempFile::new().expect("create temp file");
        fs::write(file.path(), "not = valid = toml").unwrap();
        let err = QNetConfig::load(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }

    #[test]
    fn error_on_missing_file() {
        let err = QNetConfig::load("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }
}
