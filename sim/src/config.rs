//! Simulation Configuration
//!
//! Defines the `SimConfig` struct for the Qublis-sim crate, including
//! parameters for TPS simulation, latency modeling, NeuroFlux runs,
//! network topology size, and report generation.  Supports loading from TOML.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Default simulation duration in seconds.
fn default_duration_secs() -> u64 {
    60
}

/// Default target transactions‐per‐second.
fn default_target_tps() -> u64 {
    1_000_000
}

/// Default number of dimensions to model.
fn default_dimensions() -> usize {
    4
}

/// Default mean latency in milliseconds.
fn default_latency_mean_ms() -> f64 {
    100.0
}

/// Default latency standard deviation in milliseconds.
fn default_latency_stddev_ms() -> f64 {
    20.0
}

/// Default toggle for NeuroFlux‐driven optimization.
fn default_neuroflux_enabled() -> bool {
    false
}

/// Default number of NeuroFlux iterations.
fn default_neuroflux_iterations() -> usize {
    10_000
}

/// Default size of the simulated network (number of nodes).
fn default_network_size() -> usize {
    1_000
}

/// Default report format (`"json"` or `"csv"`).
fn default_report_format() -> String {
    "json".into()
}

/// Default toggle for enabling plotting support.
fn default_enable_plotting() -> bool {
    false
}

/// Simulation configuration parameters.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimConfig {
    /// Total duration of the simulation (seconds).
    #[serde(default = "default_duration_secs")]
    pub duration_secs: u64,

    /// Target transactions per second to simulate.
    #[serde(default = "default_target_tps")]
    pub target_tps: u64,

    /// Number of dimensions (for multidimensional views).
    #[serde(default = "default_dimensions")]
    pub dimensions: usize,

    /// Latency mean (ms) for latency‐wave modeling.
    #[serde(default = "default_latency_mean_ms")]
    pub latency_mean_ms: f64,

    /// Latency standard deviation (ms).
    #[serde(default = "default_latency_stddev_ms")]
    pub latency_stddev_ms: f64,

    /// Whether to enable NeuroFlux optimization simulation.
    #[serde(default = "default_neuroflux_enabled")]
    pub neuroflux_enabled: bool,

    /// Number of iterations for NeuroFlux RL simulation.
    #[serde(default = "default_neuroflux_iterations")]
    pub neuroflux_iterations: usize,

    /// Size of the network (number of simulated nodes).
    #[serde(default = "default_network_size")]
    pub network_size: usize,

    /// Output report format: `"json"` or `"csv"`.
    #[serde(default = "default_report_format")]
    pub report_format: String,

    /// Whether to generate plots (requires `plotting` feature).
    #[serde(default = "default_enable_plotting")]
    pub enable_plotting: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        SimConfig {
            duration_secs: default_duration_secs(),
            target_tps: default_target_tps(),
            dimensions: default_dimensions(),
            latency_mean_ms: default_latency_mean_ms(),
            latency_stddev_ms: default_latency_stddev_ms(),
            neuroflux_enabled: default_neuroflux_enabled(),
            neuroflux_iterations: default_neuroflux_iterations(),
            network_size: default_network_size(),
            report_format: default_report_format(),
            enable_plotting: default_enable_plotting(),
        }
    }
}

/// Errors that can occur when loading a `SimConfig`.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the file.
    #[error("I/O error reading SimConfig: {0}")]
    Io(#[from] std::io::Error),
    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl SimConfig {
    /// Load a `SimConfig` from the given TOML file path.
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
        let cfg = SimConfig::default();
        assert_eq!(cfg.duration_secs, 60);
        assert_eq!(cfg.target_tps, 1_000_000);
        assert_eq!(cfg.dimensions, 4);
        assert!((cfg.latency_mean_ms - 100.0).abs() < 1e-12);
        assert!((cfg.latency_stddev_ms - 20.0).abs() < 1e-12);
        assert!(!cfg.neuroflux_enabled);
        assert_eq!(cfg.neuroflux_iterations, 10_000);
        assert_eq!(cfg.network_size, 1_000);
        assert_eq!(cfg.report_format, "json");
        assert!(!cfg.enable_plotting);
    }

    #[test]
    fn load_valid_toml() {
        let toml = r#"
            duration_secs = 120
            target_tps = 5000000
            dimensions = 8
            latency_mean_ms = 50.5
            latency_stddev_ms = 5.2
            neuroflux_enabled = true
            neuroflux_iterations = 20000
            network_size = 5000
            report_format = "csv"
            enable_plotting = true
        "#;
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), toml).unwrap();

        let cfg = SimConfig::load(file.path()).unwrap();
        assert_eq!(cfg.duration_secs, 120);
        assert_eq!(cfg.target_tps, 5_000_000);
        assert_eq!(cfg.dimensions, 8);
        assert!((cfg.latency_mean_ms - 50.5).abs() < 1e-12);
        assert!((cfg.latency_stddev_ms - 5.2).abs() < 1e-12);
        assert!(cfg.neuroflux_enabled);
        assert_eq!(cfg.neuroflux_iterations, 20_000);
        assert_eq!(cfg.network_size, 5_000);
        assert_eq!(cfg.report_format, "csv");
        assert!(cfg.enable_plotting);
    }

    #[test]
    fn missing_file_errs_io() {
        let err = SimConfig::load("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }

    #[test]
    fn invalid_toml_errs_parse() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "not = valid = toml").unwrap();
        let err = SimConfig::load(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }
}
