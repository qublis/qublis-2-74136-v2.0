//! Simulation Prelude
//!
//! Convenient imports and re‐exports for the Qublis‐sim crate (Qublis v2.0).
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::config::SimConfig;
pub use crate::metrics::SimMetrics;
pub use crate::types::{
    TpsResult,
    DimensionView,
    LatencyProfile,
    NeuroFluxResult,
    NetworkStats,
    ReportData,
};

pub use crate::tps_simulator::TpsSimulator;
pub use crate::dimension_viewer::DimensionViewer;
pub use crate::latency_wave::LatencyWave;
pub use crate::neuroflux_simulator::NeuroFluxSimulator;
pub use crate::network_sim::NetworkSimulator;
pub use crate::report_generator::ReportGenerator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prelude_reexports_compile() {
        // Config
        let cfg: SimConfig = SimConfig::default();

        // Metrics
        let mut metrics = SimMetrics::new();
        metrics.inc_counter("example_counter", 1);
        let prom = metrics.export_prometheus();
        assert!(prom.contains("sim_example_counter 1"));

        // Types
        let tps = TpsResult {
            target_tps: cfg.target_tps,
            average_tps: 0.0,
            samples: vec![],
        };
        assert_eq!(tps.target_tps, cfg.target_tps);

        let dim_view = DimensionView {
            dimensions: cfg.dimensions,
            utilization: std::collections::HashMap::new(),
        };
        assert_eq!(dim_view.dimensions, cfg.dimensions);

        let latency = LatencyProfile {
            mean_ms: cfg.latency_mean_ms,
            stddev_ms: cfg.latency_stddev_ms,
            samples: vec![],
        };
        assert!((latency.mean_ms - cfg.latency_mean_ms).abs() < 1e-12);

        let neuro = NeuroFluxResult {
            iterations: 0,
            best_metric: 0.0,
            progress: vec![],
        };
        assert_eq!(neuro.iterations, 0);

        let net_stats = NetworkStats {
            node_count: cfg.network_size,
            messages_routed: 0,
            average_latency_ms: 0.0,
            drop_rate: 0.0,
        };
        assert_eq!(net_stats.node_count, cfg.network_size);

        let report = ReportData {
            tps,
            dimensions: dim_view,
            latency,
            neuroflux: Some(neuro),
            network: net_stats,
        };
        assert!(report.neuroflux.is_some());

        // Simulators – just ensure constructors exist and take &SimConfig
        let _ = TpsSimulator::new(&cfg);
        let _ = DimensionViewer::new(&cfg);
        let _ = LatencyWave::new(&cfg);
        let _ = NeuroFluxSimulator::new(&cfg);
        let _ = NetworkSimulator::new(&cfg);
        let _ = ReportGenerator::new(&cfg);
    }
}
