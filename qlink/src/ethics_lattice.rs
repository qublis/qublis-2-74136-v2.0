//! Ethics Lattice for QLink — Qublis v2.0
//!
//! Implements a lattice of ethical principles, each with a quantum‐number state (`QNum`).
//! Principles can be entangled to represent interdependencies, and evaluated by collapsing
//! their QNum states to classical weights.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;

use qublis_qnum::{QNum, entangle};
use crate::{
    config::QLinkConfig,
    error::QLinkError,
    metrics::QLinkMetrics,
};

/// A named ethical principle in the lattice.
pub type Principle = String;

/// `EthicsLattice` holds a set of principles, each with an associated `QNum` state,
/// and allows entangling and evaluating them.
#[derive(Clone, Debug)]
pub struct EthicsLattice {
    config: QLinkConfig,
    metrics: QLinkMetrics,
    nodes: HashMap<Principle, QNum>,
}

impl EthicsLattice {
    /// Create a new, empty `EthicsLattice` with the given configuration.
    pub fn new(config: &QLinkConfig) -> Self {
        EthicsLattice {
            config: config.clone(),
            metrics: QLinkMetrics::new(),
            nodes: HashMap::new(),
        }
    }

    /// Add a new principle with an initial `QNum` state.
    /// Returns an error if the principle already exists.
    pub fn add_principle(&mut self, name: Principle, initial: QNum) -> Result<(), QLinkError> {
        if self.nodes.contains_key(&name) {
            return Err(QLinkError::PrincipleExists(name));
        }
        self.nodes.insert(name, initial);
        self.metrics.inc_counter("principles_added", 1);
        Ok(())
    }

    /// Entangle two existing principles by name.
    /// Returns an error if either principle is missing.
    ///
    /// This avoids double mutable borrow of self.nodes by temporarily removing one QNum,
    /// performing entanglement, then reinserting.
    pub fn entangle_principles(&mut self, a: &Principle, b: &Principle) -> Result<(), QLinkError> {
        if a == b {
            return Err(QLinkError::PrincipleNotFound(a.clone()));
        }
        // Remove one, get &mut to the other, entangle, then reinsert.
        let mut qa = self
            .nodes
            .remove(a)
            .ok_or_else(|| QLinkError::PrincipleNotFound(a.clone()))?;
        let qb = self
            .nodes
            .get_mut(b)
            .ok_or_else(|| QLinkError::PrincipleNotFound(b.clone()))?;
        entangle(&mut qa, qb);
        self.nodes.insert(a.clone(), qa);
        self.metrics.inc_counter("principles_entangled", 1);
        Ok(())
    }

    /// Evaluate all principles by collapsing each `QNum` to a classical digit weight.
    ///
    /// Returns a map principle→weight (0…9). Records one evaluation event.
    pub fn evaluate(&mut self) -> HashMap<Principle, u8> {
        let mut results = HashMap::new();
        for (name, qnum) in &mut self.nodes {
            // collapse first digit (MSB) of QNum to produce a weight
            let mut q = qnum.clone();
            let digits = q.measure();
            let weight = digits.first().cloned().unwrap_or(0);
            results.insert(name.clone(), weight);
        }
        self.metrics.inc_counter("lattice_evaluations", 1);
        results
    }

    /// Retrieve the current `QNum` state for a principle, if present.
    pub fn get_state(&self, name: &Principle) -> Option<&QNum> {
        self.nodes.get(name)
    }

    /// Export current metrics for Prometheus.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::{QNum, Qid};

    /// Helper: build a simple QNum with a single basis state.
    fn classical_qnum(digit: u8) -> QNum {
        QNum::from_digits(&[digit])
    }

    #[test]
    fn add_and_get_principle() {
        let cfg = QLinkConfig::default();
        let mut lat = EthicsLattice::new(&cfg);
        assert!(lat.add_principle("fairness".into(), classical_qnum(3)).is_ok());
        let state = lat.get_state(&"fairness".into()).unwrap();
        assert_eq!(state.measure(), vec![3]);
    }

    #[test]
    fn duplicate_principle_error() {
        let cfg = QLinkConfig::default();
        let mut lat = EthicsLattice::new(&cfg);
        let _ = lat.add_principle("transparency".into(), classical_qnum(5));
        let err = lat.add_principle("transparency".into(), classical_qnum(5)).unwrap_err();
        matches!(err, QLinkError::PrincipleExists(_));
    }

    #[test]
    fn entangle_and_evaluate() {
        let cfg = QLinkConfig::default();
        let mut lat = EthicsLattice::new(&cfg);
        let q1 = classical_qnum(2);
        let q2 = classical_qnum(7);
        lat.add_principle("p1".into(), q1.clone()).unwrap();
        lat.add_principle("p2".into(), q2.clone()).unwrap();
        lat.entangle_principles(&"p1".into(), &"p2".into()).unwrap();

        // After entanglement, weights may swap; ensure both present
        let eval = lat.evaluate();
        assert!(eval.contains_key("p1") && eval.contains_key("p2"));
        let w1 = eval["p1"];
        let w2 = eval["p2"];
        assert!((w1 == 2 && w2 == 7) || (w1 == 7 && w2 == 2));
    }

    #[test]
    fn entangle_missing_principle_error() {
        let cfg = QLinkConfig::default();
        let mut lat = EthicsLattice::new(&cfg);
        let err = lat.entangle_principles(&"x".into(), &"y".into()).unwrap_err();
        matches!(err, QLinkError::PrincipleNotFound(_));
    }

    #[test]
    fn metrics_recorded() {
        let cfg = QLinkConfig::default();
        let mut lat = EthicsLattice::new(&cfg);
        lat.add_principle("a".into(), classical_qnum(1)).unwrap();
        lat.add_principle("b".into(), classical_qnum(2)).unwrap();
        lat.entangle_principles(&"a".into(), &"b".into()).unwrap();
        let _ = lat.evaluate();
        let prom = lat.export_metrics();
        assert!(prom.contains("qlink_principles_added 2"));
        assert!(prom.contains("qlink_principles_entangled 1"));
        assert!(prom.contains("qlink_lattice_evaluations 1"));
    }
}
