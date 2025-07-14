//! QLink Configuration
//!
//! Defines the `QLinkConfig` struct for the QLink crate, including
//! quantum identity length, consent probability, and metrics toggles.
//! Supports loading from a TOML file.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Default length of generated QIDs (number of decimal digits).
fn default_qid_length() -> usize {
    6
}

/// Default probability of granting consent (0.0…1.0).
fn default_consent_probability() -> f64 {
    0.5
}

/// Default toggle for exporting Prometheus metrics.
fn default_enable_metrics() -> bool {
    false
}

/// Configuration parameters for the QLink crate.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QLinkConfig {
    /// Number of decimal digits in generated QIDs.
    #[serde(default = "default_qid_length")]
    pub qid_length: usize,

    /// Probability of granting consent when requested (0.0 to 1.0).
    #[serde(default = "default_consent_probability")]
    pub consent_probability: f64,

    /// Enable collection and export of Prometheus metrics.
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,
}

impl Default for QLinkConfig {
    fn default() -> Self {
        QLinkConfig {
            qid_length: default_qid_length(),
            consent_probability: default_consent_probability(),
            enable_metrics: default_enable_metrics(),
        }
    }
}

/// Errors that can occur loading a `QLinkConfig`.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the configuration file.
    #[error("I/O error reading QLinkConfig: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl QLinkConfig {
    /// Load configuration from the given TOML file path.
    ///
    /// Returns an error if the file cannot be read or parsed.
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
        let cfg = QLinkConfig::default();
        assert_eq!(cfg.qid_length, 6);
        assert!((cfg.consent_probability - 0.5).abs() < 1e-12);
        assert!(!cfg.enable_metrics);
    }

    #[test]
    fn load_valid_toml() {
        let toml = r#"
            qid_length = 8
            consent_probability = 0.75
            enable_metrics = true
        "#;
        let mut file = NamedTempFile::new().expect("temp file");
        fs::write(file.path(), toml).expect("write TOML");
        let cfg = QLinkConfig::load(file.path()).expect("load config");
        assert_eq!(cfg.qid_length, 8);
        assert!((cfg.consent_probability - 0.75).abs() < 1e-12);
        assert!(cfg.enable_metrics);
    }

    #[test]
    fn missing_file_returns_io_error() {
        let err = QLinkConfig::load("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }

    #[test]
    fn invalid_toml_returns_parse_error() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "not = valid = toml").unwrap();
        let err = QLinkConfig::load(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }
}
