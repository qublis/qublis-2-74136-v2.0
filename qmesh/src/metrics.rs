//! QMesh Metrics Collector
//!
//! Collects and exports metrics for the QMesh subsystem, including entropic DAG
//! operations, cognitive‐entropy analyses, and retrochain tracking.

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

/// Collector for QMesh metrics.
#[derive(Clone, Debug)]
pub struct QMeshMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl QMeshMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        QMeshMetrics {
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
        // counters
        for (k, v) in &self.counters {
            out.push_str(&format!("qmesh_{} {}\n", k, v));
        }
        // gauges
        for (k, v) in &self.gauges {
            out.push_str(&format!("qmesh_{} {}\n", k, v));
        }
        // uptime gauge
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("qmesh_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain‐specific metrics ===

    /// Record addition of a node in the entropic DAG.
    pub fn record_node_added(&mut self) {
        self.inc_counter("nodes_added", 1);
    }

    /// Record addition of an edge in the entropic DAG.
    pub fn record_edge_added(&mut self) {
        self.inc_counter("edges_added", 1);
    }

    /// Record an entanglement operation.
    pub fn record_entanglement(&mut self) {
        self.inc_counter("entanglements", 1);
    }

    /// Record a cognitive‐entropy analysis run.
    pub fn record_analysis_run(&mut self) {
        self.inc_counter("analysis_runs", 1);
    }

    /// Record a retrochain block recording.
    pub fn record_block_recorded(&mut self) {
        self.inc_counter("blocks_recorded", 1);
    }

    /// Record retrieval of a retrochain.
    pub fn record_retrochain_retrieved(&mut self) {
        self.inc_counter("retrochain_retrieved", 1);
    }

    /// Record computation of retrochain diffs.
    pub fn record_diffs_computed(&mut self) {
        self.inc_counter("diffs_computed", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counter_gauge_and_export() {
        let mut m = QMeshMetrics::new();
        m.inc_counter("nodes_added", 2);
        m.set_gauge("last_entropy", 3.5);
        m.record_snapshot();

        let prom = m.export_prometheus();
        assert!(prom.contains("qmesh_nodes_added 2"));
        assert!(prom.contains("qmesh_last_entropy 3.5"));
        assert!(prom.contains("qmesh_uptime_seconds"));

        let json = m.export_json().expect("export json");
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        let snap = &arr[0];
        assert_eq!(snap["counters"]["nodes_added"].as_u64(), Some(2));
        assert_eq!(snap["gauges"]["last_entropy"].as_f64(), Some(3.5));
    }

    #[test]
    fn domain_specific_records() {
        let mut m = QMeshMetrics::new();
        m.record_node_added();
        m.record_edge_added();
        m.record_entanglement();
        m.record_analysis_run();
        m.record_block_recorded();
        m.record_retrochain_retrieved();
        m.record_diffs_computed();

        assert_eq!(m.counters["nodes_added"], 1);
        assert_eq!(m.counters["edges_added"], 1);
        assert_eq!(m.counters["entanglements"], 1);
        assert_eq!(m.counters["analysis_runs"], 1);
        assert_eq!(m.counters["blocks_recorded"], 1);
        assert_eq!(m.counters["retrochain_retrieved"], 1);
        assert_eq!(m.counters["diffs_computed"], 1);
    }
}
