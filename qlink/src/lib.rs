//! QLink — Quantum Identity, Ethics & Consent for Qublis v2.0
//!
//! The `qublis-qlink` crate implements on‐chain quantum identities (QIDs),
//! entangled ethics lattices, conscious consent mechanics, and a mutation
//! engine for dynamic policy updates.  All state is represented as `QNum`
//! superpositions and evolves via unitary gates and measurements.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Quantum Identity layer: generate and register QIDs
pub mod qid_layer;
/// Ethics Lattice: entangled ethical constraints and evaluation
pub mod ethics_lattice;
/// Conscious Consent: manage user consent as collapsible QNums
pub mod conscious_consent;
/// Mutation Engine: apply dynamic policy updates to QID states
pub mod mutation_engine;
/// Configuration loader for QLink parameters
pub mod config;
/// Core data types (IdentityState, ConsentRecord, etc.)
pub mod types;
/// Error definitions for QLink operations
pub mod error;
/// Metrics collector for QLink events
pub mod metrics;
/// Prelude: convenient re-exports of primary QLink types
pub mod prelude;

pub use config::QLinkConfig;
pub use qid_layer::QidLayer;
pub use ethics_lattice::EthicsLattice;
pub use conscious_consent::ConsciousConsent;
pub use mutation_engine::MutationEngine;
pub use types::{IdentityState, ConsentRecord};
pub use metrics::QLinkMetrics;
pub use error::QLinkError;
pub use prelude::*;
