//! QMesh Configuration
//!
//! Defines the `QMeshConfig` struct for entropic‐DAG and cognitive‐entropy settings,
//! including the sliding‐window history length and whether to expose metrics.
//! Supports loading from a TOML file.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Default is no history window configured (use crate defaults).
fn default_history_window() -> Option<usize> {
    None
}

/// Default is to disable metrics.
fn default_enable_metrics() -> bool {
    false
}

/// QMesh configuration parameters.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QMeshConfig {
    /// Length of the sliding‐window for cognitive‐entropy history.
    /// If `None`, defaults to 10.
    #[serde(default = "default_history_window")]
    pub history_window: Option<usize>,

    /// Whether to collect and expose Prometheus metrics.
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,
}

impl Default for QMeshConfig {
    fn default() -> Self {
        QMeshConfig {
            history_window: default_history_window(),
            enable_metrics: default_enable_metrics(),
        }
    }
}

/// Errors that can occur while loading a `QMeshConfig`.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the config file.
    #[error("I/O error reading QMeshConfig: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl QMeshConfig {
    /// Load `QMeshConfig` from the given TOML file path.
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
    use tempfile::NamedTempFile;
    use std::fs;

    #[test]
    fn default_values() {
        let cfg = QMeshConfig::default();
        assert!(cfg.history_window.is_none());
        assert!(!cfg.enable_metrics);
    }

    #[test]
    fn load_from_toml() {
        let mut file = NamedTempFile::new().expect("create temp file");
        let toml = r#"
            history_window = 7
            enable_metrics = true
        "#;
        fs::write(file.path(), toml).expect("write TOML");
        let cfg = QMeshConfig::load(file.path()).expect("load config");
        assert_eq!(cfg.history_window, Some(7));
        assert!(cfg.enable_metrics);
    }

    #[test]
    fn missing_file_errs_io() {
        let err = QMeshConfig::load("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }

    #[test]
    fn invalid_toml_errs_parse() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "not = valid = toml").unwrap();
        let err = QMeshConfig::load(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }
}
