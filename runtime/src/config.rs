//! Runtime configuration loader for Qublis v2.0 (Universe 2-74136).
//!
//! Defines `RuntimeConfig` and sub-configs for consensus, entanglement loop,
//! causal reflector, WASM executor, and metrics. Supports TOML parsing with
//! reasonable defaults and error reporting.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

/// Errors that can occur when loading the runtime configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O error reading the file.
    #[error("I/O error reading config: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error.
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
}

/// Top-level runtime configuration.
#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    /// Consensus engine parameters.
    pub consensus: ConsensusConfig,
    /// Entanglement loop parameters.
    pub entanglement: EntanglementConfig,
    /// Causal reflector parameters.
    #[serde(rename = "causal_reflector")]
    pub causal: CausalReflectorConfig,
    /// WASM smart-contract executor parameters.
    pub wasm: WasmConfig,
    /// Prometheus metrics exporter parameters.
    pub metrics: MetricsConfig,
}

/// Consensus engine configuration.
#[derive(Debug, Deserialize)]
pub struct ConsensusConfig {
    /// Path to the QMesh consensus TOML (for on-disk parameters).
    pub qmesh_config_path: String,
    /// Enable NeuroFlux RL integration.
    #[serde(default = "default_true")]
    pub neuroflux_enabled: bool,
    /// Optional path to a NeuroFlux configuration TOML.
    #[serde(default)]
    pub neuroflux_config_path: Option<String>,
}

/// Entanglement loop configuration.
#[derive(Debug, Deserialize)]
pub struct EntanglementConfig {
    /// How often (ms) to run the entanglement propagation loop.
    #[serde(default = "default_ent_loop_interval")]
    pub interval_ms: u64,
    /// Maximum number of branches to explore per cycle.
    #[serde(default = "default_max_branches")]
    pub max_branches: usize,
}

/// Causal reflector configuration.
#[derive(Debug, Deserialize)]
pub struct CausalReflectorConfig {
    /// Enable multi-dimensional causal reflection.
    #[serde(default)]
    pub enabled: bool,
    /// Maximum depth of causal reflection (number of ancestor hops).
    #[serde(default = "default_causal_depth")]
    pub max_depth: usize,
}

/// WASM executor configuration.
#[derive(Debug, Deserialize)]
pub struct WasmConfig {
    /// Max memory (bytes) allowed for a single contract instance.
    #[serde(default = "default_wasm_memory_limit")]
    pub memory_limit: u64,
    /// Max gas units per contract invocation.
    #[serde(default = "default_wasm_gas_limit")]
    pub gas_limit: u64,
}

/// Metrics exporter configuration.
#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
    /// TCP port for the Prometheus metrics endpoint.
    #[serde(default = "default_metrics_port")]
    pub port: u16,
    /// Globally enable or disable metrics exporting.
    #[serde(default = "default_metrics_enabled")]
    pub enabled: bool,
}

fn default_true() -> bool { true }
fn default_ent_loop_interval() -> u64 { 100 }
fn default_max_branches() -> usize { 1024 }
fn default_causal_depth() -> usize { 64 }
fn default_wasm_memory_limit() -> u64 { 64 * 1024 * 1024 }
fn default_wasm_gas_limit() -> u64 { 1_000_000 }
fn default_metrics_port() -> u16 { 9300 }
fn default_metrics_enabled() -> bool { true }

impl RuntimeConfig {
    /// Load a `RuntimeConfig` from a TOML file at `path`.
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
    use std::io::Write;

    #[test]
    fn load_valid_config() {
        let toml = r#"
            [consensus]
            qmesh_config_path = "qmesh.toml"
            neuroflux_enabled = false

            [entanglement]
            interval_ms = 250
            max_branches = 512

            [causal_reflector]
            enabled = true
            max_depth = 16

            [wasm]
            memory_limit = 33554432
            gas_limit = 500000

            [metrics]
            port = 9400
            enabled = false
        "#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml).unwrap();

        let cfg = RuntimeConfig::load(file.path()).unwrap();
        assert_eq!(cfg.consensus.qmesh_config_path, "qmesh.toml");
        assert!(!cfg.consensus.neuroflux_enabled);
        assert_eq!(cfg.entanglement.interval_ms, 250);
        assert_eq!(cfg.entanglement.max_branches, 512);
        assert!(cfg.causal.enabled);
        assert_eq!(cfg.causal.max_depth, 16);
        assert_eq!(cfg.wasm.memory_limit, 33_554_432);
        assert_eq!(cfg.wasm.gas_limit, 500_000);
        assert_eq!(cfg.metrics.port, 9400);
        assert!(!cfg.metrics.enabled);
    }

    #[test]
    fn missing_file_errors_io() {
        let err = RuntimeConfig::load("nonexistent.toml").unwrap_err();
        matches!(err, ConfigError::Io(_));
    }

    #[test]
    fn invalid_toml_errors_parse() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "not = valid = toml").unwrap();
        let err = RuntimeConfig::load(file.path()).unwrap_err();
        matches!(err, ConfigError::Parse(_));
    }
}
