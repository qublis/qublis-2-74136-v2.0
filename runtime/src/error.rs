//! Runtime error types for Qublis v2.0 (2-74136).
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use thiserror::Error;

use crate::config::ConfigError;
use crate::types::EngineError;
use crate::wasm::WasmError;

/// Errors that can occur in the `qublis-runtime` crate.
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// Failed to load or parse configuration.
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Error returned by the consensus engine (QMesh).
    #[error("Consensus engine error: {0}")]
    Engine(#[from] EngineError),

    /// Error during WASM smart‐contract execution.
    #[error("WASM execution error: {0}")]
    Wasm(#[from] WasmError),

    /// I/O failure (file system, network, etc.).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A generic, catch‐all error.
    #[error("{0}")]
    Other(String),
}
