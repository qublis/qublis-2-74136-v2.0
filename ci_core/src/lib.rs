//! Conscious AI Core for Qublis v2.0
//!
//! This crate implements the core Conscious-AI subsystems:
//! - `MorphicAI`: an adaptive, generative neural substrate  
//! - `MoralRegulator`: enforces ethical constraints on AI decisions  
//! - `CollectiveSync`: synchronizes distributed AI agents into coherent collectives  
//!
//! Additional modules provide configuration, shared types, error handling, and metrics.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Adaptive, self-organizing neural AI engine.
pub mod morphic_ai;
/// Ethical‐constraint enforcement layer.
pub mod moral_regulator;
/// Distributed multi-agent synchronization engine.
pub mod collective_sync;
/// Configuration loader and defaults.
pub mod config;
/// Core shared types (agent state, policies, etc.).
pub mod types;
/// Error definitions for CI-Core operations.
pub mod error;
/// Prometheus-style metrics collector for CI-Core.
pub mod metrics;
/// Prelude re-exports the most common types and traits.
pub mod prelude;

pub use config::CiCoreConfig;
pub use error::CiCoreError;
pub use metrics::CiCoreMetrics;

pub use morphic_ai::MorphicAI;
pub use moral_regulator::MoralRegulator;
pub use collective_sync::CollectiveSync;

/// Conveniently import everything needed to get started.
pub use prelude::*;
