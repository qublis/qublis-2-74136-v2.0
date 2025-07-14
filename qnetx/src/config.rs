//! QNetX Configuration
//!
//! Defines the `QNetXConfig` struct for the entangled‐overlay mesh, including
//! bootstrap dimensions, metrics, and anomaly‐filter thresholds.  
//! Supports loading from a TOML file.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Default is no bootstrap dimensions.
fn default_bootstrap_nodes() -> Vec<String> {
    Vec::new()
}

/// Default is to disable metrics.
fn default_enable_metrics() -> bool {
    false
}

/// Default is no anomaly threshold configured.
fn default_anomaly_threshold() -> Option<f64> {
    None
}

/// Default is to disable anomaly filtering.
fn default_enable_anomaly() -> bool {
    false
}

/// QNetX mesh configuration parameters.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QNetXConfig {
    /// Names or addresses of dimensions to bootstrap from.
    #[serde(default = "default_bootstrap_nodes")]
    pub bootstrap_nodes: Vec<String>,

    /// Expose Prometheus metrics if true.
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,

    /// If set, any channel with entropy > this value is anomalous.
    #[serde(default = "default_anomaly_threshold")]
    pub anomaly_threshold: Option<f64>,

    /// Whether to run the anomaly filter.
    #[serde(default = "default_enable_anomaly")]
    pub enable_anomaly: bool,
}

impl Default for QNetXConfig {
    fn default() -> Self {
        QNetXConfig {
            bootstrap_nodes: default_bootstrap_nodes(),
            enable_metrics: default_enable_metrics(),
            anomaly_threshold: default_anomaly_threshold(),
            enable_anomaly: default_enable_anomaly(),
        }
    }
}

/// Errors that can occur when loading a `QNetXConfig`.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the file.
    #[error("I/O error reading QNetXConfig: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl QNetXConfig {
    /// Load a `QNetXConfig` from the given TOML file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let cfg = toml::from_str(&contents)?;
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn default_config() {
        let cfg = QNetXConfig::default();
        assert!(cfg.bootstrap_nodes.is_empty());
        assert!(!cfg.enable_metrics);
        assert!(cfg.anomaly_threshold.is_none());
        assert!(!cfg.enable_anomaly);
    }

    #[test]
    fn load_valid_toml() {
        let toml = r#"
            bootstrap_nodes = ["dimA","dimB"]
            enable_metrics = true
            anomaly_threshold = 0.42
            enable_anomaly = true
        "#;
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), toml).unwrap();

        let cfg = QNetXConfig::load(file.path()).unwrap();
        assert_eq!(cfg.bootstrap_nodes, vec!["dimA".to_string(), "dimB".to_string()]);
        assert!(cfg.enable_metrics);
        assert_eq!(cfg.anomaly_threshold, Some(0.42));
        assert!(cfg.enable_anomaly);
    }

    #[test]
    fn load_missing_file_errs_io() {
        let err = QNetXConfig::load("nonexistent.toml").unwrap_err();
        match err {
            ConfigError::Io(_) => {},
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn load_invalid_toml_errs_parse() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "not = valid = toml").unwrap();
        let err = QNetXConfig::load(file.path()).unwrap_err();
        match err {
            ConfigError::Parse(_) => {},
            _ => panic!("Expected Parse error"),
        }
    }
}
