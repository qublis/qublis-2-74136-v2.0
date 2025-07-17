//! Error definitions for the `qublis-qnetx-node` CLI.
//!
//! Centralizes all error types into a single `NodeError`, using `thiserror` for
//! convenient `From` conversions and `Display` implementations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    /// I/O error (file system, network, etc.).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to parse the node configuration TOML.
    #[error("Configuration parse error: {0}")]
    Config(#[from] toml::de::Error),

    /// CLI argument parsing or validation error.
    #[error("CLI error: {0}")]
    Clap(#[from] clap::Error),

    /// QNet networking error.
    #[error("Network error: {0}")]
    Network(#[from] qublis_qnet::NetworkError),

    /// QNetX overlay error (QuantumMeshOverlay, ZeroPropagation, etc.).
    #[error("Overlay error: {0}")]
    Overlay(#[from] qublis_qnetx::OverlayError),

    /// Runtime integration error (consensus, entanglement, WASM, etc.).
    #[error("Runtime error: {0}")]
    Runtime(#[from] qublis_runtime::RuntimeError),

    /// Any other uncategorized error.
    #[error("{0}")]
    Other(String),
}
