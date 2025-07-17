//! Core runtime for Qublis v2.0 (Universe 2-74136).
//!
//! This crate implements the blockchain runtime modules:
//! - Consensus integration with NeuroFlux optimization  
//! - Entanglement loop for quantum-inspired state propagation  
//! - Causal reflector enforcing multi-dimensional causality  
//! - WASM execution support for QBLang smart contracts  
//! - Configuration loading, error definitions, metrics, and core types

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, missing_debug_implementations)]

pub mod config;
pub mod types;
pub mod error;
pub mod metrics;

pub mod consensus_neuroflux;
pub mod entanglement_loop;
pub mod causal_reflector;
pub mod wasm;

pub mod prelude;

/// Runtime configuration loaded from TOML.
pub use config::RuntimeConfig;
/// Core blockchain types, e.g. `Block` and `ConsensusEngine`.
pub use types::{Block, ConsensusEngine};
/// Error type for all runtime operations.
pub use error::RuntimeError;
/// Prometheus-style metrics collector for the runtime.
pub use metrics::RuntimeMetrics;
/// NeuroFlux-powered consensus adapter.
pub use consensus_neuroflux::ConsensusNeuroFlux;
/// Entanglement loop for propagating quantum state across dimensions.
pub use entanglement_loop::EntanglementLoop;
/// Causal reflector ensuring consistency across infinite branches.
pub use causal_reflector::CausalReflector;
/// WASM smart-contract executor for QBLang modules.
pub use wasm::WasmExecutor;
