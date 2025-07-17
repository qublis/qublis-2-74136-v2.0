//! Runtime Prelude for Qublis v2.0 (2-74136).
//!
//! Re-exports the most common types, traits, and macros used throughout the
//! `qublis-runtime` crate for convenient import.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::{
    // Configuration types
    config::{
        RuntimeConfig,
        ConsensusConfig,
        EntanglementConfig,
        CausalReflectorConfig,
        WasmConfig,
        MetricsConfig,
    },
    // Core types
    types::{Block, ConsensusEngine, EngineError},
    // Error handling
    error::RuntimeError,
    // Metrics collector
    metrics::RuntimeMetrics,
    // Runtime modules
    consensus_neuroflux::ConsensusNeuroFlux,
    entanglement_loop::EntanglementLoop,
    causal_reflector::CausalReflector,
    wasm::WasmExecutor,
};

// Serde derives and traits
pub use serde::{Deserialize, Serialize};
// Logging macros
pub use log::{debug, error, info, trace, warn};
