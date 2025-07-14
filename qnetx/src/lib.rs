//! Entangled overlay mesh (QNetX) for Qublis v2.0
//!
//! `qublis-qnetx` provides a multi-dimensional, entangled overlay network
//! on top of QNet.  Core components include:
//! - `QuantumMesh`: build and manage entangled channels between dimensions.
//! - `ZeroPropagator`: propagate quantum‐zero modes through the mesh.
//! - `StateCondenser`: condense global mesh state into lower‐dimensional summaries.
//! - `AnomalyFilter`: detect and filter entanglement anomalies.
//! - `QNetXConfig`: configurable parameters for mesh behavior.
//! - `QNetXMetrics`: telemetry for mesh operations.
//! - `QNetXError`: unified error handling.
//!
//! All routing identifiers and channel IDs are represented as `QNum` superpositions
//! from the `qublis-qnum` crate, enabling true quantum‐inspired randomness and context‐sensitive routing.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod quantum_mesh;
pub mod zero_prop;
pub mod state_condenser;
pub mod anomaly_filter;
pub mod config;
pub mod types;
pub mod error;
pub mod metrics;
pub mod prelude;

pub use config::QNetXConfig;
pub use quantum_mesh::QuantumMesh;
pub use zero_prop::ZeroPropagator;
pub use state_condenser::StateCondenser;
pub use anomaly_filter::AnomalyFilter;
pub use error::QNetXError;
pub use metrics::QNetXMetrics;
pub use types::{Dimension, ChannelId};

/// Convenient import of all primary QNetX types.
pub use prelude::*;
