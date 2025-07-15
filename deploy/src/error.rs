//! Deployment Error Types for Qublis‐deploy — Qublis v2.0
//!
//! Defines the `DeployError` enum that aggregates errors from configuration loading,
//! CI fork launching, and QNetX node bootstrapping.

use thiserror::Error;

use crate::config::ConfigError;
use crate::ci_fork_launcher::CiForkError;
use crate::qnetx_node_bootstrap::BootstrapError;

/// Errors returned by the `qublis-deploy` CLI.
#[derive(Debug, Error)]
pub enum DeployError {
    /// Error loading or parsing a TOML configuration.
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Error during CI fork launching operations.
    #[error("CI fork launcher error: {0}")]
    CiFork(#[from] CiForkError),

    /// Error during QNetX node bootstrap operations.
    #[error("QNetX node bootstrap error: {0}")]
    Bootstrap(#[from] BootstrapError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ci_fork_launcher::CiForkError;
    use crate::qnetx_node_bootstrap::BootstrapError;
    use crate::config::ConfigError;
    use std::io;

    #[test]
    fn from_config_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "failed read");
        let cfg_err: ConfigError = io_err.into();
        let err: DeployError = cfg_err.into();
        let msg = err.to_string();
        assert!(msg.starts_with("configuration error:"));
        assert!(msg.contains("failed read"));
    }

    #[test]
    fn from_cifork_error() {
        let git_err = CiForkError::Git("clone failed".into());
        let err: DeployError = git_err.into();
        let msg = err.to_string();
        assert!(msg.starts_with("CI fork launcher error:"));
        assert!(msg.contains("clone failed"));
    }

    #[test]
    fn from_bootstrap_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "no config");
        let bs_err: BootstrapError = io_err.into();
        let err: DeployError = bs_err.into();
        let msg = err.to_string();
        assert!(msg.starts_with("QNetX node bootstrap error:"));
        assert!(msg.contains("no config"));
    }
}
