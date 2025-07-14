//! Qublis-sim — Simulation suite for Qublis v2.0
//!
//! Provides a framework and tools to simulate TPS, latency, multi-dimensional views,
//! NeuroFlux optimization, full network behavior, and report generation for Qublis v2.0.
//
//! # Modules
//!
//! - `config`: simulation configuration loader  
//! - `types`: data types for simulation inputs and outputs  
//! - `metrics`: internal metrics collector  
//! - `tps_simulator`: TPS and throughput simulation  
//! - `dimension_viewer`: multi-dimensional resource visualization  
//! - `latency_wave`: latency waveform modeling  
//! - `neuroflux_simulator`: NeuroFlux RL-driven optimization simulation  
//! - `network_sim`: full network traffic and topology simulation  
//! - `report_generator`: aggregation and export of simulation results  
//! - `prelude`: convenient re-exports  

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod config;
pub mod types;
pub mod metrics;

pub mod tps_simulator;
pub mod dimension_viewer;
pub mod latency_wave;
pub mod neuroflux_simulator;
pub mod network_sim;
pub mod report_generator;
pub mod prelude;

/// Re-export core configuration and metrics types.
pub use config::SimConfig;
pub use metrics::SimMetrics;

/// Re-export all typed simulation results.
pub use types::{
    TpsResult,
    DimensionView,
    LatencyProfile,
    NeuroFluxResult,
    NetworkStats,
    ReportData,
};

/// Re-export main simulators and generators.
pub use tps_simulator::TpsSimulator;
pub use dimension_viewer::DimensionViewer;
pub use latency_wave::LatencyWave;
pub use neuroflux_simulator::NeuroFluxSimulator;
pub use network_sim::NetworkSimulator;
pub use report_generator::ReportGenerator;
