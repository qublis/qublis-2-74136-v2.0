//! Report Generator for Qublis‐sim — Qublis v2.0
//!
//! Aggregates results from all simulators into a final `ReportData`,
//! then serializes to JSON or CSV according to configuration.  
//! Records a metric for report generation.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::fmt::Write as FmtWrite;

use crate::{
    config::SimConfig,
    error::SimError,
    metrics::SimMetrics,
    types::{ReportData, TpsResult, DimensionView, LatencyProfile, NeuroFluxResult, NetworkStats},
};
use crate::tps_simulator::TpsSimulator;
use crate::dimension_viewer::DimensionViewer;
use crate::latency_wave::LatencyWave;
use crate::neuroflux_simulator::NeuroFluxSimulator;
use crate::network_sim::NetworkSimulator;

/// `ReportGenerator` orchestrates simulation components and exports the final report.
#[derive(Debug)]
pub struct ReportGenerator {
    config: SimConfig,
    metrics: SimMetrics,
}

impl ReportGenerator {
    /// Create a new report generator with the given configuration.
    pub fn new(config: &SimConfig) -> Self {
        ReportGenerator {
            config: config.clone(),
            metrics: SimMetrics::new(),
        }
    }

    /// Run all simulations and collect the `ReportData`.
    pub fn generate(&mut self) -> Result<ReportData, SimError> {
        // TPS
        let mut tps_sim = TpsSimulator::new(&self.config);
        let tps = tps_sim.simulate()?;
        // Dimension view
        let mut dim_sim = DimensionViewer::new(&self.config);
        let dimensions = dim_sim.view();
        // Latency
        let mut lat_sim = LatencyWave::new(&self.config);
        let latency = lat_sim.simulate()?;
        // NeuroFlux (optional)
        let neuroflux = if self.config.neuroflux_enabled {
            let mut nf_sim = NeuroFluxSimulator::new(&self.config);
            Some(nf_sim.simulate()?)
        } else {
            None
        };
        // Network
        let mut net_sim = NetworkSimulator::new(&self.config);
        let network = net_sim.simulate()?;

        // Record report generation metric
        self.metrics.record_reports_generated();

        Ok(ReportData {
            tps,
            dimensions,
            latency,
            neuroflux,
            network,
        })
    }

    /// Serialize `ReportData` to JSON or CSV according to `config.report_format`.
    pub fn export(&self, report: &ReportData) -> Result<String, SimError> {
        match self.config.report_format.as_str() {
            "csv" => self.export_csv(report).map_err(|e| SimError::ReportError(e)),
            _ => {
                serde_json::to_string_pretty(report)
                    .map_err(SimError::Serialization)
            }
        }
    }

    fn export_csv(&self, report: &ReportData) -> Result<String, String> {
        let mut w = String::new();
        // TPS
        writeln!(&mut w, "section,field,value").unwrap();
        writeln!(&mut w, "tps,target_tps,{}", report.tps.target_tps).unwrap();
        writeln!(&mut w, "tps,average_tps,{}", report.tps.average_tps).unwrap();
        // Dimension utilizations
        for (dim, util) in &report.dimensions.utilization {
            writeln!(&mut w, "dimension,utilization_{}", dim,).unwrap();
            writeln!(&mut w, "dimension_{}_utilization,{}", dim, util).unwrap();
        }
        // Latency
        writeln!(&mut w, "latency,mean_ms,{}", report.latency.mean_ms).unwrap();
        writeln!(&mut w, "latency,stddev_ms,{}", report.latency.stddev_ms).unwrap();
        // NeuroFlux
        if let Some(nf) = &report.neuroflux {
            writeln!(&mut w, "neuroflux,iterations,{}", nf.iterations).unwrap();
            writeln!(&mut w, "neuroflux,best_metric,{}", nf.best_metric).unwrap();
        }
        // Network
        writeln!(&mut w, "network,node_count,{}", report.network.node_count).unwrap();
        writeln!(&mut w, "network,messages_routed,{}", report.network.messages_routed).unwrap();
        writeln!(&mut w, "network,average_latency_ms,{}", report.network.average_latency_ms).unwrap();
        writeln!(&mut w, "network,drop_rate,{}", report.network.drop_rate).unwrap();

        Ok(w)
    }

    /// Export internal metrics in Prometheus text format.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SimConfig;
    use crate::types::ReportData;

    #[test]
    fn generate_and_export_json() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 2;
        cfg.target_tps = 100;
        cfg.neuroflux_enabled = false;
        cfg.report_format = "json".into();

        let mut rg = ReportGenerator::new(&cfg);
        let report = rg.generate().expect("generate should succeed");
        // export JSON
        let s = rg.export(&report).unwrap();
        let de: ReportData = serde_json::from_str(&s).unwrap();
        assert_eq!(de.tps.target_tps, 100);
        assert_eq!(de.network.node_count, cfg.network_size);
    }

    #[test]
    fn generate_and_export_csv() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 1;
        cfg.target_tps = 50;
        cfg.neuroflux_enabled = false;
        cfg.report_format = "csv".into();

        let mut rg = ReportGenerator::new(&cfg);
        let report = rg.generate().unwrap();
        let csv = rg.export(&report).unwrap();
        // Should contain header and at least one line
        assert!(csv.starts_with("section,field,value"));
        assert!(csv.contains("tps,target_tps,50"));
    }

    #[test]
    fn metrics_recorded() {
        let cfg = SimConfig::default();
        let mut rg = ReportGenerator::new(&cfg);
        let report = rg.generate().unwrap();
        let _ = rg.export(&report).unwrap();
        let prom = rg.export_metrics();
        assert!(prom.contains("sim_reports_generated 1"));
    }
}
