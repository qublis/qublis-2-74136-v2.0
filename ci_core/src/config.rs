//! CI‐Core Configuration
//!
//! Defines `CiCoreConfig` for the Conscious AI core, including MorphicAI,
//! MoralRegulator, and CollectiveSync parameters. Supports loading from TOML.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Default number of neurons in the MorphicAI substrate.
fn default_num_neurons() -> usize {
    128
}

/// Default learning rate for MorphicAI reinforcement training.
fn default_learning_rate() -> f64 {
    0.01
}

/// Default toggle for global entangle in CollectiveSync.
fn default_enable_global_entangle() -> bool {
    false
}

/// Default toggle for global average in CollectiveSync.
fn default_enable_global_average() -> bool {
    false
}

/// Default toggle for exporting Prometheus metrics.
fn default_enable_metrics() -> bool {
    false
}

/// Configuration for the CI‐Core crate.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CiCoreConfig {
    /// Number of neurons in the MorphicAI engine.
    #[serde(default = "default_num_neurons")]
    pub num_neurons: usize,

    /// Learning rate used during MorphicAI training.
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f64,

    /// If true, CollectiveSync entangles each receiver with sender on messages.
    #[serde(default = "default_enable_global_entangle")]
    pub enable_global_entangle: bool,

    /// If true, CollectiveSync averages all agent states globally.
    #[serde(default = "default_enable_global_average")]
    pub enable_global_average: bool,

    /// Enable collection/export of Prometheus metrics for CI‐Core.
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,
}

impl Default for CiCoreConfig {
    fn default() -> Self {
        CiCoreConfig {
            num_neurons: default_num_neurons(),
            learning_rate: default_learning_rate(),
            enable_global_entangle: default_enable_global_entangle(),
            enable_global_average: default_enable_global_average(),
            enable_metrics: default_enable_metrics(),
        }
    }
}

/// Errors that can occur when loading `CiCoreConfig`.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the configuration file.
    #[error("I/O error reading CiCoreConfig: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl CiCoreConfig {
    /// Load `CiCoreConfig` from a TOML file at `path`.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let s = fs::read_to_string(path)?;
        let cfg = toml::from_str(&s)?;
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
        let cfg = CiCoreConfig::default();
        assert_eq!(cfg.num_neurons, 128);
        assert!((cfg.learning_rate - 0.01).abs() < 1e-12);
        assert!(!cfg.enable_global_entangle);
        assert!(!cfg.enable_global_average);
        assert!(!cfg.enable_metrics);
    }

    #[test]
    fn load_valid_toml() {
        let toml = r#"
            num_neurons = 256
            learning_rate = 0.05
            enable_global_entangle = true
            enable_global_average = true
            enable_metrics = true
        "#;
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), toml).unwrap();
        let cfg = CiCoreConfig::load(file.path()).unwrap();
        assert_eq!(cfg.num_neurons, 256);
        assert!((cfg.learning_rate - 0.05).abs() < 1e-12);
        assert!(cfg.enable_global_entangle);
        assert!(cfg.enable_global_average);
        assert!(cfg.enable_metrics);
    }

    #[test]
    fn missing_file_errs_io() {
        let err = CiCoreConfig::load("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }

    #[test]
    fn invalid_toml_errs_parse() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "not = valid = toml").unwrap();
        let err = CiCoreConfig::load(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }
}
