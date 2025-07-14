//! QMesh — Entropic DAG, Cognitive Entropy, and Retrochain Tracking for Qublis v2.0
//!
//! This crate provides:
//! - `EntropicDag`: a directed acyclic graph of `QNum` states with entropic propagation.
//! - `CognitiveEntropy`: utilities to compute and monitor mesh‐wide entropy patterns.
//! - `RetrochainTracker`: track entropic “retrochain” dependencies across blocks.
//! - `QMeshConfig`: configuration loader for QMesh settings.
//! - `QMeshError`: error definitions.
//! - `QMeshMetrics`: domain‐specific metrics.
//! - `prelude`: convenient imports of core types and traits.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Entropic DAG management
pub mod entropic_dag;
/// Cognitive entropy analysis routines
pub mod cognitive_entropy;
/// Retrochain dependency tracking
pub mod retrochain_tracker;
/// Configuration loader and defaults
pub mod config;
/// Core data types (NodeId, NodeData, etc.)
pub mod types;
/// Error definitions for QMesh operations
pub mod error;
/// Metrics collector for entropic operations
pub mod metrics;
/// Prelude for easy importing of common types
pub mod prelude;

/// Re-export main QMesh types at crate root.
pub use config::QMeshConfig;
pub use entropic_dag::EntropicDag;
pub use cognitive_entropy::CognitiveEntropy;
pub use retrochain_tracker::RetrochainTracker;
pub use error::QMeshError;
pub use metrics::QMeshMetrics;
