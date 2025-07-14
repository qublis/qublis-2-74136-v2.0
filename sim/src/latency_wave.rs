//! Latency Waveform Simulator for Qublis‐sim — Qublis v2.0
//!
//! Models network latency over time by sampling once per second from a
//! normal distribution with configured mean and standard deviation (ms).
//! Records samples and exposes Prometheus‐style metrics.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use rand::Rng;
use rand_distr::{Normal, Distribution};
use crate::{
    config::SimConfig,
    error::SimError,
    metrics::SimMetrics,
    types::LatencyProfile,
};

/// `LatencyWave` runs a latency profile simulation.
#[derive(Debug)]
pub struct LatencyWave {
    config: SimConfig,
    metrics: SimMetrics,
}

impl LatencyWave {
    /// Construct a new `LatencyWave` with the given simulation configuration.
    pub fn new(config: &SimConfig) -> Self {
        LatencyWave {
            config: config.clone(),
            metrics: SimMetrics::new(),
        }
    }

    /// Simulate latency over each second of the configured duration.
    ///
    /// Samples latency (ms) from Normal(mean_ms, stddev_ms) per second.
    pub fn simulate(&mut self) -> Result<LatencyProfile, SimError> {
        let dur = self.config.duration_secs;
        let mean = self.config.latency_mean_ms;
        let stddev = self.config.latency_stddev_ms;
        let mut rng = rand::thread_rng();
        let normal = Normal::new(mean, stddev)
            .map_err(|e| SimError::LatencyError(format!("invalid distribution: {}", e)))?;

        let mut samples = Vec::with_capacity(dur as usize);
        for t in 0..dur {
            let sample = normal.sample(&mut rng).max(0.0);
            samples.push((t, sample));
            self.metrics.record_latency_sample();
        }

        Ok(LatencyProfile {
            mean_ms: mean,
            stddev_ms: stddev,
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
    fn simulate_generates_correct_length() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 5;
        cfg.latency_mean_ms = 100.0;
        cfg.latency_stddev_ms = 10.0;
        let mut lw = LatencyWave::new(&cfg);
        let profile = lw.simulate().expect("should simulate");
        assert_eq!(profile.samples.len(), 5);
        // Check metadata
        assert_eq!(profile.mean_ms, 100.0);
        assert_eq!(profile.stddev_ms, 10.0);
    }

    #[test]
    fn simulate_samples_non_negative() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 3;
        cfg.latency_mean_ms = 0.0;
        cfg.latency_stddev_ms = 50.0;
        let mut lw = LatencyWave::new(&cfg);
        let profile = lw.simulate().unwrap();
        for &(_, latency) in &profile.samples {
            assert!(latency >= 0.0);
        }
    }

    #[test]
    fn metrics_recorded_per_sample() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 4;
        let mut lw = LatencyWave::new(&cfg);
        let _ = lw.simulate().unwrap();
        let prom = lw.export_metrics();
        // Expect "sim_latency_samples 4" entry
        assert!(prom.contains("sim_latency_samples 4"));
    }

    #[test]
    fn zero_duration_yields_empty_samples() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 0;
        let mut lw = LatencyWave::new(&cfg);
        let profile = lw.simulate().unwrap();
        assert!(profile.samples.is_empty());
        // Metrics should record zero samples
        let prom = lw.export_metrics();
        assert!(!prom.contains("sim_latency_samples"));
    }
}
