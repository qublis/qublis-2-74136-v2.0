//! CI‐Core Metrics Collector
//!
//! Collects and exports Prometheus‐style metrics for the Conscious‐AI core,
//! covering MorphicAI, MoralRegulator, and CollectiveSync events.

use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::Serialize;

/// A timestamped snapshot of counters and gauges.
#[derive(Debug, Clone, Serialize)]
struct MetricSnapshot {
    timestamp: u128,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
}

/// Collector for CI‐Core metrics.
#[derive(Clone, Debug)]
pub struct CiCoreMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl CiCoreMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        CiCoreMetrics {
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

    /// Set a gauge to `value`.
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

    /// Export recorded snapshots as pretty‐printed JSON.
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| format!("Metrics JSON export failed: {}", e))
    }

    /// Export current counters and gauges in Prometheus text format.
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.counters {
            out.push_str(&format!("ci_core_{} {}\n", k, v));
        }
        for (k, v) in &self.gauges {
            out.push_str(&format!("ci_core_{} {}\n", k, v));
        }
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("ci_core_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain‐specific metrics ===

    /// Record initialization of MorphicAI.
    pub fn record_morphic_initialized(&mut self) {
        self.inc_counter("morphic_ai_initialized", 1);
    }

    /// Record a perception event in MorphicAI.
    pub fn record_morphic_perceptions(&mut self) {
        self.inc_counter("morphic_ai_perceptions", 1);
    }

    /// Record a generation event in MorphicAI.
    pub fn record_morphic_generations(&mut self) {
        self.inc_counter("morphic_ai_generations", 1);
    }

    /// Record a training event in MorphicAI.
    pub fn record_morphic_trains(&mut self) {
        self.inc_counter("morphic_ai_trains", 1);
    }

    /// Record addition of a principle in MoralRegulator.
    pub fn record_principles_added(&mut self) {
        self.inc_counter("principles_added", 1);
    }

    /// Record entanglement of principles in MoralRegulator.
    pub fn record_principles_entangled(&mut self) {
        self.inc_counter("principles_entangled", 1);
    }

    /// Record an action allowed by MoralRegulator.
    pub fn record_actions_allowed(&mut self) {
        self.inc_counter("actions_allowed", 1);
    }

    /// Record an action violation by MoralRegulator.
    pub fn record_actions_violated(&mut self) {
        self.inc_counter("actions_violated", 1);
    }

    /// Record initialization of CollectiveSync.
    pub fn record_collective_initialized(&mut self) {
        self.inc_counter("collective_sync_initialized", 1);
    }

    /// Record agent registration in CollectiveSync.
    pub fn record_agents_registered(&mut self) {
        self.inc_counter("agents_registered", 1);
    }

    /// Record a message sent in CollectiveSync.
    pub fn record_messages_sent(&mut self) {
        self.inc_counter("messages_sent", 1);
    }

    /// Record a global entanglement event in CollectiveSync.
    pub fn record_global_entanglements(&mut self) {
        self.inc_counter("global_entanglements", 1);
    }

    /// Record a global averaging event in CollectiveSync.
    pub fn record_global_averages(&mut self) {
        self.inc_counter("global_averages", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counter_gauge_and_export() {
        let mut m = CiCoreMetrics::new();
        m.inc_counter("foo", 2);
        m.set_gauge("bar", 3.14);
        m.record_snapshot();

        let prom = m.export_prometheus();
        assert!(prom.contains("ci_core_foo 2"));
        assert!(prom.contains("ci_core_bar 3.14"));
        assert!(prom.contains("ci_core_uptime_seconds"));

        let json = m.export_json().unwrap();
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["counters"]["foo"].as_u64(), Some(2));
        assert_eq!(arr[0]["gauges"]["bar"].as_f64(), Some(3.14));
    }

    #[test]
    fn domain_specific_records() {
        let mut m = CiCoreMetrics::new();
        m.record_morphic_initialized();
        m.record_morphic_perceptions();
        m.record_morphic_generations();
        m.record_morphic_trains();
        m.record_principles_added();
        m.record_principles_entangled();
        m.record_actions_allowed();
        m.record_actions_violated();
        m.record_collective_initialized();
        m.record_agents_registered();
        m.record_messages_sent();
        m.record_global_entanglements();
        m.record_global_averages();

        assert_eq!(m.counters["morphic_ai_initialized"], 1);
        assert_eq!(m.counters["morphic_ai_perceptions"], 1);
        assert_eq!(m.counters["morphic_ai_generations"], 1);
        assert_eq!(m.counters["morphic_ai_trains"], 1);
        assert_eq!(m.counters["principles_added"], 1);
        assert_eq!(m.counters["principles_entangled"], 1);
        assert_eq!(m.counters["actions_allowed"], 1);
        assert_eq!(m.counters["actions_violated"], 1);
        assert_eq!(m.counters["collective_sync_initialized"], 1);
        assert_eq!(m.counters["agents_registered"], 1);
        assert_eq!(m.counters["messages_sent"], 1);
        assert_eq!(m.counters["global_entanglements"], 1);
        assert_eq!(m.counters["global_averages"], 1);
    }
}
