//! MoralRegulator — Ethical‐Constraint Enforcement for Qublis v2.0
//!
//! `MoralRegulator` uses the QLink `EthicsLattice` to enforce high‐level
//! ethical constraints on AI motor outputs.  It entangles proposed outputs
//! with a lattice of principles and collapses to decide whether an action
//! is permitted.  Violations are recorded and can be vetoed or modified.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use qublis_qlink::{EthicsLattice, QLinkConfig};
use crate::{
    config::CiCoreConfig,
    error::CiCoreError,
    metrics::CiCoreMetrics,
    types::MotorOutput,
};

/// `MoralRegulator` holds an ethics lattice and records enforcement metrics.
#[derive(Clone, Debug)]
pub struct MoralRegulator {
    /// Underlying ethics lattice from QLink
    lattice: EthicsLattice,
    /// Internal metrics collector
    metrics: CiCoreMetrics,
}

impl MoralRegulator {
    /// Create a new `MoralRegulator`.  
    /// Uses the default QLinkConfig; you can swap in a custom one if needed.
    pub fn new(_cfg: &CiCoreConfig) -> Self {
        // Initialize the ethics lattice with default QLink settings
        let qlink_cfg = QLinkConfig::default();
        MoralRegulator {
            lattice: EthicsLattice::new(&qlink_cfg),
            metrics: CiCoreMetrics::new(),
        }
    }

    /// Add a new ethical principle to the lattice.
    ///
    /// Returns an error if the principle already exists.
    pub fn add_principle(
        &mut self,
        name: String,
        initial_state: qublis_qnum::QNum,
    ) -> Result<(), CiCoreError> {
        self.lattice
            .add_principle(name.clone(), initial_state)
            .map_err(|e| CiCoreError::EthicsError(e.to_string()))?;
        self.metrics.inc_counter("principles_added", 1);
        Ok(())
    }

    /// Entangle two existing principles by name.
    ///
    /// Returns an error if either principle is missing.
    pub fn entangle_principles(
        &mut self,
        a: &str,
        b: &str,
    ) -> Result<(), CiCoreError> {
        self.lattice
            .entangle_principles(&a.to_string(), &b.to_string())
            .map_err(|e| CiCoreError::EthicsError(e.to_string()))?;
        self.metrics.inc_counter("principles_entangled", 1);
        Ok(())
    }

    /// Enforce the current ethics lattice against a proposed `MotorOutput`.
    ///
    /// If any principle collapses to a “forbid” weight (zero), the action
    /// is considered a violation and is vetoed (returned as Err).
    /// Otherwise, the output is permitted.
    pub fn enforce(&mut self, output: MotorOutput) -> Result<MotorOutput, CiCoreError> {
        // Evaluate all principles: principle -> weight (0…9)
        let weights = self.lattice.evaluate();
        // If any principle collapses to zero, veto the action
        let violation = weights.iter().any(|(_, &w)| w == 0);
        if violation {
            self.metrics.inc_counter("actions_violated", 1);
            Err(CiCoreError::EthicsViolation(
                "one or more principles violated".into(),
            ))
        } else {
            self.metrics.inc_counter("actions_allowed", 1);
            Ok(output)
        }
    }

    /// Export Prometheus‐style metrics for the moral regulator.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    /// Helper: build a simple ethical QNum (classical).
    fn classical_qnum(digit: u8) -> QNum {
        QNum::from_digits(&[digit])
    }

    #[test]
    fn add_and_entangle_principles() {
        let cfg = CiCoreConfig::default();
        let mut mr = MoralRegulator::new(&cfg);
        // Add two principles
        assert!(mr.add_principle("p1".into(), classical_qnum(3)).is_ok());
        assert!(mr.add_principle("p2".into(), classical_qnum(7)).is_ok());
        // Entangle them
        assert!(mr.entangle_principles("p1", "p2").is_ok());
    }

    #[test]
    fn enforce_allows_when_no_violation() {
        let cfg = CiCoreConfig::default();
        let mut mr = MoralRegulator::new(&cfg);
        // Single principle with non-zero classical state
        let _ = mr.add_principle("fairness".into(), classical_qnum(5));
        let out = MotorOutput { signals: vec![1,2,3] };
        let result = mr.enforce(out.clone());
        assert_eq!(result.unwrap().signals, out.signals);
    }

    #[test]
    fn enforce_violates_on_zero_weight() {
        let cfg = CiCoreConfig::default();
        let mut mr = MoralRegulator::new(&cfg);
        // Add a principle with zero‐state → always violate
        let _ = mr.add_principle("strict".into(), QNum::zero(1));
        let out = MotorOutput { signals: vec![9] };
        let err = mr.enforce(out).unwrap_err();
        matches!(err, CiCoreError::EthicsViolation(_));
    }

    #[test]
    fn metrics_recorded() {
        let cfg = CiCoreConfig::default();
        let mut mr = MoralRegulator::new(&cfg);
        let _ = mr.add_principle("a".into(), classical_qnum(1));
        let ok = mr.enforce(MotorOutput { signals: vec![0] }).unwrap_err();
        let prom = mr.export_metrics();
        // One violation and one addition
        assert!(prom.contains("principles_added 1"));
        assert!(prom.contains("actions_violated 1"));
    }
}
