//! Prelude imports for the `qublis-qnetx-node` crate.
//!
//! Re-exports the most commonly used types, functions, and macros
//! for convenient use throughout the `qnetx-node` codebase.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::{
    // CLI
    cli::{Cli, Command},
    // Configuration types
    config::{NodeConfig, ConsensusConfig, MetricsConfig, TelemetryConfig, LoggingConfig},
    // Bootstrap helpers
    bootstrap::init as bootstrap_node,
    // Core node operations
    node::{run as run_node, status as node_status},
    // Telemetry server
    telemetry::start as start_telemetry,
    // Metrics helpers
    metrics::{inc_counter, set_gauge, export_prometheus},
    // Unified error type
    error::NodeError,
};

// Re-export commonly used external crates/macros
pub use clap::Parser;
pub use log::{debug, error, info, trace, warn};
