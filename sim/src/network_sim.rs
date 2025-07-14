//! Full Network Simulator for Qublis‐sim — Qublis v2.0
//!
//! Simulates end‐to‐end network behavior by combining TPS and latency models,
//! aggregating over a configured number of nodes, and computing total messages,
//! average latency, and packet drop rate.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use rand::Rng;
use crate::{
    config::SimConfig,
    error::SimError,
    metrics::SimMetrics,
    types::NetworkStats,
};
use crate::tps_simulator::TpsSimulator;
use crate::latency_wave::LatencyWave;

/// `NetworkSimulator` runs a combined network simulation.
#[derive(Debug)]
pub struct NetworkSimulator {
    config: SimConfig,
    metrics: SimMetrics,
}

impl NetworkSimulator {
    /// Create a new `NetworkSimulator` with the given configuration.
    pub fn new(config: &SimConfig) -> Self {
        NetworkSimulator {
            config: config.clone(),
            metrics: SimMetrics::new(),
        }
    }

    /// Run the full network simulation.
    ///
    /// - Uses `TpsSimulator` to simulate message rates.
    /// - Uses `LatencyWave` to simulate per‐second latencies.
    /// - Aggregates total messages, computes average latency, and simulates
    ///   a random packet drop rate between 0% and 5%.
    pub fn simulate(&mut self) -> Result<NetworkStats, SimError> {
        // Simulate transactions‐per‐second
        let mut tps_sim = TpsSimulator::new(&self.config);
        let tps_res = tps_sim.simulate()?;

        // Simulate latency profile
        let mut lat_sim = LatencyWave::new(&self.config);
        let latency_profile = lat_sim.simulate()?;

        // Aggregate total messages routed (sum of per‐second TPS samples)
        let messages_routed: u64 = tps_res
            .samples
            .iter()
            .map(|&(_, inst)| inst as u64)
            .sum();

        // Compute average end‐to‐end latency
        let average_latency_ms = if latency_profile.samples.is_empty() {
            0.0
        } else {
            let sum: f64 = latency_profile
                .samples
                .iter()
                .map(|&(_, latency)| latency)
                .sum();
            sum / (latency_profile.samples.len() as f64)
        };

        // Simulate a random drop rate in [0.0, 0.05)
        let mut rng = rand::thread_rng();
        let drop_rate = rng.gen_range(0.0..0.05);

        // Record a network‐event metric per simulated second
        for _ in 0..self.config.duration_secs {
            self.metrics.record_network_events();
        }

        Ok(NetworkStats {
            node_count: self.config.network_size,
            messages_routed,
            average_latency_ms,
            drop_rate,
        })
    }

    /// Export internal metrics (Prometheus text format).
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SimConfig;

    #[test]
    fn simulate_network_stats_basic() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 3;
        cfg.network_size = 5;
        cfg.target_tps = 100;
        cfg.latency_mean_ms = 50.0;
        cfg.latency_stddev_ms = 10.0;

        let mut sim = NetworkSimulator::new(&cfg);
        let stats = sim.simulate().expect("network simulate succeeds");

        // Basic field checks
        assert_eq!(stats.node_count, 5);
        assert!(stats.messages_routed > 0);
        assert!(stats.average_latency_ms >= 0.0);
        assert!(stats.drop_rate >= 0.0 && stats.drop_rate < 1.0);

        // Metrics should record one event per second
        let prom = sim.export_metrics();
        assert!(prom.contains(&format!("sim_network_events {}", cfg.duration_secs)));
    }

    #[test]
    fn zero_duration_yields_zero_messages_and_latency() {
        let mut cfg = SimConfig::default();
        cfg.duration_secs = 0;
        cfg.network_size = 10;

        let mut sim = NetworkSimulator::new(&cfg);
        let stats = sim.simulate().unwrap();

        assert_eq!(stats.messages_routed, 0);
        assert_eq!(stats.average_latency_ms, 0.0);
        // No metrics recorded
        let prom = sim.export_metrics();
        assert!(!prom.contains("sim_network_events"));
    }
}
