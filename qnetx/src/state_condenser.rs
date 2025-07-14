//! StateCondenser — Condenses high‐dimensional mesh state into summary QNums
//! for QNetX (Universe 2-74136, QBL v2.0).
//!
//! The `StateCondenser` provides utilities to reduce a collection of entangled
//! channel QNums into a single summary QNum (`condense_all`) or into grouped
//! summaries by prefix (`condense_by_prefix`).  Useful for generating low‐dimensional
//! views of global mesh uncertainty.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;

use qublis_qnum::{QNum, qadd};

use crate::{
    config::QNetXConfig,
    error::QNetXError,
    metrics::QNetXMetrics,
    types::ChannelId,
    quantum_mesh::QuantumMesh,
};

/// `StateCondenser` holds configuration and metrics for state condensation.
#[derive(Clone, Debug)]
pub struct StateCondenser {
    config: QNetXConfig,
    metrics: QNetXMetrics,
}

impl StateCondenser {
    /// Create a new `StateCondenser` with the given config.
    pub fn new(config: &QNetXConfig) -> Self {
        StateCondenser {
            config: config.clone(),
            metrics: QNetXMetrics::new(),
        }
    }

    /// Condense **all** channel QNums in the mesh into a single summary QNum
    /// by iteratively applying quantum addition (`qadd`).
    ///
    /// Returns an error if the mesh has no channels.
    pub fn condense_all(&mut self, mesh: &QuantumMesh) -> Result<QNum, QNetXError> {
        let mut iter = mesh.channels.values();
        let first = iter
            .next()
            .ok_or_else(|| QNetXError::CondensationError("no channels to condense".into()))?;
        let mut summary = first.clone();

        for q in iter {
            summary = qadd(&summary, q);
        }

        self.metrics.inc_counter("condense_all", 1);
        Ok(summary)
    }

    /// Condense channel QNums grouped by the first digit of their `ChannelId`.
    ///
    /// Returns a map from group key (0–9) to the group summary QNum.
    /// Channels whose ID is empty are skipped.
    pub fn condense_by_prefix(&mut self, mesh: &QuantumMesh) -> HashMap<u8, QNum> {
        let mut groups: HashMap<u8, QNum> = HashMap::new();

        for (id, qnum) in &mesh.channels {
            if let Some(&prefix) = id.first() {
                groups
                    .entry(prefix)
                    .and_modify(|acc| *acc = qadd(acc, qnum))
                    .or_insert_with(|| qnum.clone());
            }
        }

        self.metrics.inc_counter("condense_by_prefix", 1);
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::{QNum, Qid};

    /// Build a dummy mesh with given channel QNums.
    fn build_dummy_mesh(channels: Vec<(ChannelId, QNum)>) -> QuantumMesh {
        let cfg = QNetXConfig::default();
        let mut mesh = QuantumMesh::new(&cfg);
        for (id, q) in channels {
            mesh.channels.insert(id, q);
        }
        mesh
    }

    #[test]
    fn test_condense_all_single() {
        let q1 = QNum::from_digits(&[1, 0]);
        let mesh = build_dummy_mesh(vec![(vec![0,1], q1.clone())]);
        let mut sc = StateCondenser::new(&QNetXConfig::default());
        let summary = sc.condense_all(&mesh).unwrap();
        assert_eq!(summary.measure(), q1.measure());
    }

    #[test]
    fn test_condense_all_multiple() {
        // 10 + 20 = 30
        let q1 = QNum::from_digits(&[1, 0]);
        let q2 = QNum::from_digits(&[2, 0]);
        let mesh = build_dummy_mesh(vec![(vec![0,1], q1), (vec![0,2], q2)]);
        let mut sc = StateCondenser::new(&QNetXConfig::default());
        let summary = sc.condense_all(&mesh).unwrap();
        // measurement always yields 30
        assert_eq!(summary.measure(), vec![3, 0]);
    }

    #[test]
    fn test_condense_all_empty() {
        let mesh = build_dummy_mesh(vec![]);
        let mut sc = StateCondenser::new(&QNetXConfig::default());
        let err = sc.condense_all(&mesh).unwrap_err();
        matches!(err, QNetXError::CondensationError(_));
    }

    #[test]
    fn test_condense_by_prefix_groups() {
        // Channels: [1x, 1y, 2z]
        let q1 = QNum::from_digits(&[1]);
        let q2 = QNum::from_digits(&[2]);
        let mesh = build_dummy_mesh(vec![
            (vec![1], q1.clone()),
            (vec![1], q1.clone()),
            (vec![2], q2.clone()),
        ]);
        let mut sc = StateCondenser::new(&QNetXConfig::default());
        let groups = sc.condense_by_prefix(&mesh);
        // Group 1: 1+1 = 2, Group 2: 2
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get(&1).unwrap().measure(), vec![2]);
        assert_eq!(groups.get(&2).unwrap().measure(), vec![2]);
    }
}
