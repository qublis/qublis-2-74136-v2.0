//! Mutation Engine for QLink — Qublis v2.0
//!
//! Applies dynamic policy updates to on‐chain quantum identities (QIDs).
//! Each `PolicyUpdate` carries a `QNum` of parameters; applying an update
//! entangles the identity’s QNum with the update QNum, evolving its state.
//
//! For details on `PolicyUpdate`, see [`crate::types::PolicyUpdate`].

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::{
    config::QLinkConfig,
    error::QLinkError,
    metrics::QLinkMetrics,
    types::{IdentityState, PolicyUpdate},
};
use qublis_qnum::{QNum, entangle};

/// Engine that records and applies policy‐level mutations to identities.
#[derive(Debug)]
pub struct MutationEngine {
    config: QLinkConfig,
    metrics: QLinkMetrics,
    /// Mapping from identity QNum → sequence of policy updates.
    updates: HashMap<QNum, Vec<PolicyUpdate>>,
}

impl MutationEngine {
    /// Create a new `MutationEngine` with the given configuration.
    pub fn new(config: &QLinkConfig) -> Self {
        MutationEngine {
            config: config.clone(),
            metrics: QLinkMetrics::new(),
            updates: HashMap::new(),
        }
    }

    /// Record a `PolicyUpdate` for the identity `state.qid`.
    ///
    /// Returns an error if the update is malformed or duplicates a prior update ID.
    pub fn record_update(
        &mut self,
        qid: &QNum,
        update: PolicyUpdate,
    ) -> Result<(), QLinkError> {
        // Ensure no duplicate timestamp for same policy
        let entry = self.updates.entry(qid.clone()).or_default();
        if entry.iter().any(|u| u.timestamp == update.timestamp && u.policy_id == update.policy_id) {
            return Err(QLinkError::MutationError(format!(
                "duplicate update for QID at timestamp {}",
                update.timestamp
            )));
        }
        entry.push(update);
        self.metrics.inc_counter("policy_updates_recorded", 1);
        Ok(())
    }

    /// Apply all recorded updates for `state` in chronological order,
    /// returning a new `IdentityState` with the evolved QNum.
    ///
    /// If no updates exist, returns the original `state` clone.
    pub fn apply_updates(&mut self, state: &IdentityState) -> Result<IdentityState, QLinkError> {
        // Clone original state
        let mut new_state = state.clone();
        // Fetch updates, if any
        if let Some(upds) = self.updates.get(&state.qid) {
            // Sort updates by timestamp
            let mut sorted = upds.clone();
            sorted.sort_by_key(|u| u.timestamp);
            // Entangle identity QNum with each update's parameters
            for upd in sorted {
                entangle(&mut new_state.qid, &mut upd.parameters.clone());
                self.metrics.inc_counter("policy_updates_applied", 1);
            }
        }
        Ok(new_state)
    }

    /// Export internal metrics (e.g., for Prometheus).
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    /// Helper: create a simple `PolicyUpdate` with given ID, parameters, and timestamp.
    fn make_update(id: &str, params: QNum, ts: u64) -> PolicyUpdate {
        PolicyUpdate {
            policy_id: id.to_string(),
            parameters: params,
            timestamp: ts,
        }
    }

    fn default_cfg() -> QLinkConfig {
        QLinkConfig::default()
    }

    #[test]
    fn record_and_apply_single_update() {
        let mut engine = MutationEngine::new(&default_cfg());
        // Identity = classical QNum "42"
        let mut identity = IdentityState::new(QNum::from_digits(&[4, 2]), 0);
        let original_qid = identity.qid.clone();

        // Create an update that flips digits (e.g., entangles with [2,4])
        let update = make_update("u1", QNum::from_digits(&[2, 4]), 100);
        engine.record_update(&identity.qid, update.clone()).unwrap();

        let updated = engine.apply_updates(&identity).unwrap();
        // After entanglement, measuring yields either original or swapped digits
        let meas = updated.qid.measure();
        assert!(meas == vec![4,2] || meas == vec![2,4]);

        // Metrics recorded
        let prom = engine.export_metrics();
        assert!(prom.contains("qlink_policy_updates_recorded 1"));
        assert!(prom.contains("qlink_policy_updates_applied 1"));
    }

    #[test]
    fn apply_no_updates_returns_same_state() {
        let mut engine = MutationEngine::new(&default_cfg());
        let identity = IdentityState::new(QNum::from_digits(&[5,5]), 0);
        let result = engine.apply_updates(&identity).unwrap();
        assert_eq!(result.qid.measure(), identity.qid.measure());
    }

    #[test]
    fn duplicate_update_fails() {
        let mut engine = MutationEngine::new(&default_cfg());
        let qid = QNum::from_digits(&[1,0]);
        let upd1 = make_update("p", QNum::from_digits(&[1,1]), 10);
        let upd2 = make_update("p", QNum::from_digits(&[1,1]), 10);
        engine.record_update(&qid, upd1).unwrap();
        let err = engine.record_update(&qid, upd2).unwrap_err();
        matches!(err, QLinkError::MutationError(_));
    }
}
