//! Simulation Error Types for Qublis-sim — Qublis v2.0
//!
//! Defines the `SimError` enum for errors encountered in the simulation suite:
//! configuration loading, I/O, serialization, and domain‐specific simulation modules.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use thiserror::Error;
use crate::config::ConfigError;
use std::io;
use serde_json;

/// Errors returned by simulators and utilities in the Qublis-sim crate.
#[derive(Debug, Error)]
pub enum SimError {
    /// Configuration loading or parsing failure.
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    /// I/O error, such as reading or writing files.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Serialization / deserialization error (e.g., JSON).
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Transactions‐per‐second simulator error.
    #[error("TPS simulation error: {0}")]
    TpsError(String),

    /// Multi‐dimensional viewer error.
    #[error("Dimension viewer error: {0}")]
    DimensionError(String),

    /// Latency waveform modeling error.
    #[error("Latency model error: {0}")]
    LatencyError(String),

    /// NeuroFlux optimization simulation error.
    #[error("NeuroFlux simulation error: {0}")]
    NeuroFluxError(String),

    /// Full network simulation error.
    #[error("Network simulation error: {0}")]
    NetworkError(String),

    /// Report generation error.
    #[error("Report generation error: {0}")]
    ReportError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigError;
    use serde_json;
    use std::io;

    #[test]
    fn display_tps_error() {
        let err = SimError::TpsError("overflow".into());
        assert_eq!(err.to_string(), "TPS simulation error: overflow");
    }

    #[test]
    fn display_dimension_error() {
        let err = SimError::DimensionError("invalid axis".into());
        assert_eq!(err.to_string(), "Dimension viewer error: invalid axis");
    }

    #[test]
    fn from_config_error() {
        let cfg_err = ConfigError::Parse(toml::de::Error::custom("bad toml"));
        let err: SimError = cfg_err.into();
        assert!(err.to_string().starts_with("configuration error:"));
    }

    #[test]
    fn from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "disk full");
        let err: SimError = io_err.into();
        assert!(err.to_string().starts_with("I/O error: disk full"));
    }

    #[test]
    fn from_serialization_error() {
        let json_err = serde_json::from_str::<u32>("not a number").unwrap_err();
        let err: SimError = json_err.into();
        assert!(err.to_string().starts_with("serialization error:"));
    }
}
