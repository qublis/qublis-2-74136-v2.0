//! Probabilistic routing, relay, and teleportation core for Qublis v2.0
//!
//! This crate provides the `Router`, `Relay`, and `TeleportCore` abstractions
//! for Qublis network routing.  Instead of classical path selection, routing
//! decisions are expressed as `QNum` superpositions (via `qublis-qnum`), and
//! measured upon relay for true quantum‐inspired randomness and load balancing.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Path‐finding and quantum‐superposed routing decisions.
pub mod router;
/// Packet relay over chosen paths.
pub mod relay;
/// Quantum teleportation overlay for instantaneous packet transfer.
pub mod teleport_core;
/// Configuration types for QNet.
pub mod config;
/// Core data types (NodeId, Path, etc.).
pub mod types;
/// Error definitions for QNet operations.
pub mod error;
/// Metrics collection for routing & relay.
pub mod metrics;
/// Prelude: common re-exports.
pub mod prelude;

pub use config::QNetConfig;
pub use error::QNetError;
pub use metrics::QNetMetrics;
pub use prelude::*;
