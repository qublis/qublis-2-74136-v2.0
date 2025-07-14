//! CI‐Core Error Types
//!
//! Defines the `CiCoreError` enum for errors encountered in the
//! Conscious‐AI core modules: configuration, MorphicAI, MoralRegulator,
//! and CollectiveSync.

use thiserror::Error;
use crate::config::ConfigError;

/// Errors returned by the CI‐Core subsystems.
#[derive(Debug, Error)]
pub enum CiCoreError {
    /// Configuration loading or parsing failure.
    #[error("configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    /// Dimension mismatch when perceiving input.
    #[error("dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch {
        expected: usize,
        got: usize,
    },

    /// Error returned by the ethics lattice or regulator.
    #[error("ethics error: {0}")]
    EthicsError(String),

    /// Enforcement veto due to a principle violation.
    #[error("ethics violation: {0}")]
    EthicsViolation(String),

    /// Synchronization failure in CollectiveSync.
    #[error("sync error: {0}")]
    SyncError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigError;

    #[test]
    fn display_dimension_mismatch() {
        let err = CiCoreError::DimensionMismatch { expected: 4, got: 2 };
        assert_eq!(err.to_string(), "dimension mismatch: expected 4, got 2");
    }

    #[test]
    fn from_config_error() {
        let cfg_err = ConfigError::Parse(toml::de::Error::custom("oops"));
        let err: CiCoreError = cfg_err.into();
        assert!(err.to_string().starts_with("configuration error:"));
    }

    #[test]
    fn ethics_error_display() {
        let err = CiCoreError::EthicsError("bad principle".into());
        assert_eq!(err.to_string(), "ethics error: bad principle");
    }

    #[test]
    fn ethics_violation_display() {
        let err = CiCoreError::EthicsViolation("forbidden action".into());
        assert_eq!(err.to_string(), "ethics violation: forbidden action");
    }

    #[test]
    fn sync_error_display() {
        let err = CiCoreError::SyncError("no agents".into());
        assert_eq!(err.to_string(), "sync error: no agents");
    }
}
