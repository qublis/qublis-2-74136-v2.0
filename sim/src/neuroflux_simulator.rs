//! NeuroFlux Simulator for Qublis‐sim — Qublis v2.0
//!
//! Runs a reinforcement‐learning inspired, quantum‐enhanced optimization simulation
//! over a configured number of iterations (`neuroflux_iterations`).  
//! Records per‐iteration performance metrics and tracks the best metric found.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use crate::{config::SimConfig, error::SimError, metrics::SimMetrics, types::NeuroFluxResult};
use rand::Rng;

/// `NeuroFluxSimulator` executes NeuroFlux optimization loops.
#[derive(Debug)]
pub struct NeuroFluxSimulator {
    config: SimConfig,
    metrics: SimMetrics,
}

impl NeuroFluxSimulator {
    /// Create a new simulator with the given configuration.
    pub fn new(config: &SimConfig) -> Self {
        NeuroFluxSimulator {
            config: config.clone(),
            metrics: SimMetrics::new(),
        }
    }

    /// Run the NeuroFlux simulation.
    ///
    /// If `config.neuroflux_enabled` is false, returns an error.
    /// Otherwise performs `neuroflux_iterations` random‐search iterations,
    /// recording a random performance metric in [0.0, 1.0) each time,
    /// tracking the best value, and returning a `NeuroFluxResult`.
    pub fn simulate(&mut self) -> Result<NeuroFluxResult, SimError> {
        if !self.config.neuroflux_enabled {
            return Err(SimError::NeuroFluxError(
                "NeuroFlux simulation disabled".into(),
            ));
        }
        let iterations = self.config.neuroflux_iterations;
        let mut rng = rand::thread_rng();
        let mut progress = Vec::with_capacity(iterations);
        let mut best_metric = f64::NEG_INFINITY;

        for i in 0..iterations {
            let metric: f64 = rng.gen_range(0.0..1.0);
            if metric > best_metric {
                best_metric = metric;
            }
            progress.push((i, metric));
            self.metrics.record_neuroflux_iteration();
        }

        Ok(NeuroFluxResult {
            iterations,
            best_metric: if best_metric.is_finite() { best_metric } else { 0.0 },
            progress,
        })
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

    #[test]
    fn simulate_returns_results_when_enabled() {
        let mut cfg = SimConfig::default();
        cfg.neuroflux_enabled = true;
        cfg.neuroflux_iterations = 50;
        let mut sim = NeuroFluxSimulator::new(&cfg);
        let res = sim.simulate().expect("should run simulation");
        assert_eq!(res.iterations, 50);
        assert_eq!(res.progress.len(), 50);
        // All metrics are in [0,1)
        for &(_, m) in &res.progress {
            assert!(m >= 0.0 && m < 1.0);
        }
        // best_metric is the max of progress
        let max_sample = res.progress.iter().map(|&(_, m)| m).fold(0./0., f64::max);
        assert!((res.best_metric - max_sample).abs() < 1e-12);
    }

    #[test]
    fn simulate_errors_when_disabled() {
        let cfg = SimConfig::default(); // neuroflux_enabled = false by default
        let mut sim = NeuroFluxSimulator::new(&cfg);
        let err = sim.simulate().unwrap_err();
        match err {
            SimError::NeuroFluxError(msg) => {
                assert!(msg.contains("disabled"));
            }
            _ => panic!("Expected NeuroFluxError"),
        }
    }

    #[test]
    fn metrics_recorded_per_iteration() {
        let mut cfg = SimConfig::default();
        cfg.neuroflux_enabled = true;
        cfg.neuroflux_iterations = 7;
        let mut sim = NeuroFluxSimulator::new(&cfg);
        let _ = sim.simulate().unwrap();
        let prom = sim.export_metrics();
        assert!(prom.contains("sim_neuroflux_iterations 7"));
    }
}
