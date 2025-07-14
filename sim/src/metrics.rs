//! Simulation Metrics Collector for Qublis‐sim — Qublis v2.0
//!
//! Collects and exports Prometheus‐style metrics for the simulation suite,
//! covering TPS sampling, latency sampling, dimension views, NeuroFlux iterations,
//! network events, and report generation.

use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::Serialize;

/// A snapshot of counters and gauges at a point in time.
#[derive(Debug, Clone, Serialize)]
struct MetricSnapshot {
    /// Milliseconds since UNIX epoch.
    timestamp: u128,
    /// Monotonic counters.
    counters: HashMap<String, u64>,
    /// Instantaneous gauges.
    gauges: HashMap<String, f64>,
}

/// Collector for simulation metrics.
#[derive(Clone, Debug)]
pub struct SimMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl SimMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        SimMetrics {
            start: Instant::now(),
            counters: HashMap::new(),
            gauges: HashMap::new(),
            snapshots: Vec::new(),
        }
    }

    /// Increment a named counter by `value`.
    pub fn inc_counter(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    /// Set a gauge (instantaneous metric) to `value`.
    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    /// Record a snapshot of current counters and gauges.
    pub fn record_snapshot(&mut self) {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.snapshots.push(MetricSnapshot {
            timestamp: ts,
            counters: self.counters.clone(),
            gauges: self.gauges.clone(),
        });
    }

    /// Export all recorded snapshots as pretty‐printed JSON.
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| format!("Metrics JSON export failed: {}", e))
    }

    /// Export current counters and gauges in Prometheus text format.
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.counters {
            out.push_str(&format!("sim_{} {}\n", k, v));
        }
        for (k, v) in &self.gauges {
            out.push_str(&format!("sim_{} {}\n", k, v));
        }
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("sim_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain‐specific metrics ===

    /// Record a TPS sample.
    pub fn record_tps_sample(&mut self) {
        self.inc_counter("tps_samples", 1);
    }

    /// Record a latency sample.
    pub fn record_latency_sample(&mut self) {
        self.inc_counter("latency_samples", 1);
    }

    /// Record a dimension view generation.
    pub fn record_dimension_views(&mut self) {
        self.inc_counter("dimension_views", 1);
    }

    /// Record a NeuroFlux iteration.
    pub fn record_neuroflux_iteration(&mut self) {
        self.inc_counter("neuroflux_iterations", 1);
    }

    /// Record a network simulation step or event.
    pub fn record_network_events(&mut self) {
        self.inc_counter("network_events", 1);
    }

    /// Record a report generation.
    pub fn record_reports_generated(&mut self) {
        self.inc_counter("reports_generated", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counters_and_gauges() {
        let mut m = SimMetrics::new();
        m.inc_counter("foo", 2);
        m.set_gauge("bar", 3.14);
        m.record_snapshot();

        let prom = m.export_prometheus();
        assert!(prom.contains("sim_foo 2"));
        assert!(prom.contains("sim_bar 3.14"));
        assert!(prom.contains("sim_uptime_seconds"));

        let json = m.export_json().unwrap();
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["counters"]["foo"].as_u64(), Some(2));
        assert_eq!(arr[0]["gauges"]["bar"].as_f64(), Some(3.14));
    }

    #[test]
    fn domain_specific_metrics_increment() {
        let mut m = SimMetrics::new();
        m.record_tps_sample();
        m.record_latency_sample();
        m.record_dimension_views();
        m.record_neuroflux_iteration();
        m.record_network_events();
        m.record_reports_generated();

        assert_eq!(m.counters["tps_samples"], 1);
        assert_eq!(m.counters["latency_samples"], 1);
        assert_eq!(m.counters["dimension_views"], 1);
        assert_eq!(m.counters["neuroflux_iterations"], 1);
        assert_eq!(m.counters["network_events"], 1);
        assert_eq!(m.counters["reports_generated"], 1);
    }
}
