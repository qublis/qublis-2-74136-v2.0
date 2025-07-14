//! TPS Simulator for Qublis‐sim — Qublis v2.0
//!
//! Simulates transactions‐per‐second over a configured duration,
//! sampling once per second with random jitter around the target TPS.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use rand::Rng;
use crate::{
    config::SimConfig,
    error::SimError,
    metrics::SimMetrics,
    types::TpsResult,
};

/// `TpsSimulator` runs a simple per‐second TPS simulation.
#[derive(Debug)]
pub struct TpsSimulator {
    config: SimConfig,
    metrics: SimMetrics,
}

impl TpsSimulator {
    /// Create a new `TpsSimulator` with the given configuration.
    pub fn new(config: &SimConfig) -> Self {
        TpsSimulator {
            config: config.clone(),
            metrics: SimMetrics::new(),
        }
    }

    /// Run the TPS simulation.
    ///
    /// Samples instantaneous TPS once per second, with ±10% random jitter.
    /// Returns a `TpsResult` with target, average, and time‐series samples.
    pub fn simulate(&mut self) -> Result<TpsResult, SimError> {
        let target = self.config.target_tps;
        let duration = self.config.duration_secs;
        let mut rng = rand::thread_rng();
        let mut samples = Vec::with_capacity(duration as usize);
        let mut sum = 0.0;

        for t in 0..duration {
            // instantaneous TPS = target * random factor in [0.9,1.1)
            let factor: f64 = rng.gen_range(0.9..1.1);
            let inst = (target as f64) * factor;
            samples.push((t, inst));
            sum += inst;
            self.metrics.record_tps_sample();
        }

        let average = if duration > 0 {
            sum / (duration as f64)
        } else {
            0.0
        };

        Ok(TpsResult {
            target_tps: target,
            average_tps: average,
            samples,
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
    fn simulate_short_run() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 3;
        cfg.target_tps = 100;
        let mut sim = TpsSimulator::new(&cfg);
        let res = sim.simulate().expect("simulation should succeed");
        assert_eq!(res.target_tps, 100);
        assert_eq!(res.samples.len(), 3);
        // average should be around target ±10%
        assert!(res.average_tps > 90.0 && res.average_tps < 110.0);
    }

    #[test]
    fn metrics_count_matches_duration() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 5;
        cfg.target_tps = 50;
        let mut sim = TpsSimulator::new(&cfg);
        let _ = sim.simulate().unwrap();
        let prom = sim.export_metrics();
        // should have recorded 5 TPS samples
        assert!(prom.contains("sim_tps_samples 5"));
    }

    #[test]
    fn zero_duration_returns_zero_average() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 0;
        cfg.target_tps = 123;
        let mut sim = TpsSimulator::new(&cfg);
        let res = sim.simulate().unwrap();
        assert_eq!(res.samples.len(), 0);
        assert_eq!(res.average_tps, 0.0);
    }
}
