//! QMesh Error Types
//!
//! Defines the `QMeshError` enum for errors encountered in QMesh operations:
//! cycle detection in the entropic DAG, configuration loading, I/O, and serialization.

use crate::config::ConfigError;
use serde_json;
use thiserror::Error;

/// Errors returned by QMesh subsystems.
#[derive(Debug, Error)]
pub enum QMeshError {
    /// Adding an edge would introduce a cycle in the DAG.
    #[error("cycle detected in entropic DAG")]
    CycleDetected,

    /// Error loading or parsing QMesh configuration.
    #[error("configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    /// I/O error, e.g., reading or writing files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization / deserialization error (e.g., JSON).
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigError;
    use std::io;

    #[test]
    fn display_cycle_detected() {
        let err = QMeshError::CycleDetected;
        assert_eq!(err.to_string(), "cycle detected in entropic DAG");
    }

    #[test]
    fn from_config_error() {
        let cfg_err = ConfigError::Parse(toml::de::Error::custom("oops"));
        let err: QMeshError = cfg_err.into();
        assert!(err.to_string().starts_with("configuration error:"));
    }

    #[test]
    fn from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "disk full");
        let err: QMeshError = io_err.into();
        assert_eq!(err.to_string(), "I/O error: disk full");
    }

    #[test]
    fn from_serialization_error() {
        let ser_err: serde_json::Error = serde_json::from_str::<i32>("not_int").unwrap_err();
        let err: QMeshError = ser_err.into();
        assert!(err.to_string().starts_with("serialization error:"));
    }
}
