//! Anomaly Filter for QNetX — Qublis v2.0
//!
//! The `AnomalyFilter` identifies entangled channels whose quantum‐number
//! state entropy exceeds a configured threshold, marking them as anomalous.
//! Useful for pruning unstable or corrupted channels in the overlay mesh.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use crate::config::QNetXConfig;
use crate::metrics::QNetXMetrics;
use crate::quantum_mesh::QuantumMesh;
use crate::types::ChannelId;
use qublis_qnum::QNum;

/// Default entropy threshold: channels with entropy above this are anomalous.
const DEFAULT_THRESHOLD: f64 = 1.0;

/// `AnomalyFilter` holds the entropy threshold and metrics collector.
#[derive(Clone, Debug)]
pub struct AnomalyFilter {
    threshold: f64,
    metrics: QNetXMetrics,
}

impl AnomalyFilter {
    /// Create a new filter, using `config.anomaly_threshold` if set,
    /// otherwise `DEFAULT_THRESHOLD`.
    pub fn new(config: &QNetXConfig) -> Self {
        let threshold = config
            .anomaly_threshold
            .unwrap_or(DEFAULT_THRESHOLD);
        AnomalyFilter {
            threshold,
            metrics: QNetXMetrics::new(),
        }
    }

    /// Detect all channel IDs whose QNum entropy exceeds the threshold.
    ///
    /// Increments the `"anomalies_detected"` counter for each anomaly.
    pub fn detect(&mut self, mesh: &QuantumMesh) -> Vec<ChannelId> {
        let mut anomalies = Vec::new();
        for (id, state) in &mesh.channels {
            let ent = state.entropy();
            if ent > self.threshold {
                anomalies.push(id.clone());
                self.metrics.inc_counter("anomalies_detected", 1);
            }
        }
        anomalies
    }

    /// Access the internal metrics collector.
    pub fn metrics(&self) -> &QNetXMetrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::{QNum, Qid};
    use crate::config::QNetXConfig;

    /// Helper: build a mesh with given channel QNums.
    fn build_mesh(states: Vec<(ChannelId, QNum)>) -> QuantumMesh {
        let cfg = QNetXConfig {
            anomaly_threshold: None,
            ..Default::default()
        };
        let mut mesh = QuantumMesh::new(&cfg);
        for (id, q) in states {
            mesh.channels.insert(id, q);
        }
        mesh
    }

    #[test]
    fn no_anomalies_when_entropy_below_threshold() {
        // Create a classical channel: entropy = 0
        let q0 = QNum::from_digits(&[3, 3]);
        let mesh = build_mesh(vec![(vec![1,2], q0)]);
        let mut cfg = QNetXConfig::default();
        // Set threshold high
        cfg.anomaly_threshold = Some(0.5);
        let mut filter = AnomalyFilter::new(&cfg);

        let anomalies = filter.detect(&mesh);
        assert!(anomalies.is_empty());
        assert_eq!(filter.metrics().export_prometheus().contains("anomalies_detected"), false);
    }

    #[test]
    fn detects_anomalies_when_entropy_high() {
        // Build a QNum in equal superposition of all digits => high entropy
        let mut amps = [num_complex::Complex::new(1.0, 0.0); 10];
        let total = 10.0;
        for a in amps.iter_mut() {
            *a /= total.sqrt();
        }
        let q_high = Qid::new(amps);
        let qnum = QNum(vec![q_high.clone(), q_high]);
        let mesh = build_mesh(vec![(vec![9,9], qnum)]);
        let mut cfg = QNetXConfig::default();
        // Low threshold to catch it
        cfg.anomaly_threshold = Some(0.1);
        let mut filter = AnomalyFilter::new(&cfg);

        let anomalies = filter.detect(&mesh);
        assert_eq!(anomalies, vec![vec![9,9]]);
        let prom = filter.metrics().export_prometheus();
        assert!(prom.contains("anomalies_detected 1"));
    }

    #[test]
    fn uses_default_threshold_if_not_set() {
        // A small superposition: moderate entropy
        let states = vec![
            (vec![1], num_complex::Complex::new(1.0/2f64.sqrt(),0.0)),
            (vec![2], num_complex::Complex::new(1.0/2f64.sqrt(),0.0))
        ];
        let qnum = QNum::from_superposed(states);
        let mesh = build_mesh(vec![(vec![0], qnum)]);
        // cfg.anomaly_threshold = None => DEFAULT_THRESHOLD = 1.0
        let cfg = QNetXConfig::default();
        let mut filter = AnomalyFilter::new(&cfg);

        // Entropy of this qnum = ln2 ≈ 0.693 < 1.0 => no anomaly
        let anomalies = filter.detect(&mesh);
        assert!(anomalies.is_empty());
    }
}
