//! Quantum Identity Layer (QidLayer) for Qublis v2.0
//!
//! This module provides generation, registration, lookup, and revocation of
//! quantum identities (QIDs) on‐chain.  A QID is represented as a `QNum`,
//! derived deterministically from a seed; identities are stored in an
//! in‐memory registry (simulating a blockchain state), with lifecycle events
//! recorded in metrics.

use std::collections::HashMap;
use qublis_qnum::QNum;
use crate::config::QLinkConfig;
use crate::error::QLinkError;
use crate::types::IdentityState;
use crate::metrics::QLinkMetrics;

/// QidLayer manages the lifecycle of quantum identities.
#[derive(Debug)]
pub struct QidLayer {
    config: QLinkConfig,
    metrics: QLinkMetrics,
    /// Registry mapping QNum → IdentityState
    registry: HashMap<QNum, IdentityState>,
}

impl QidLayer {
    /// Create a new `QidLayer` using the given configuration.
    pub fn new(config: &QLinkConfig) -> Self {
        QidLayer {
            config: config.clone(),
            metrics: QLinkMetrics::new(),
            registry: HashMap::new(),
        }
    }

    /// Deterministically generate a QID (`QNum`) from a seed byte string.
    ///
    /// Uses the configured `qid_length` to produce a fixed‐digit decimal QNum:
    /// each output digit = `seed[i % seed.len()] % 10`.
    pub fn generate_qid(&self, seed: &[u8]) -> QNum {
        let len = self.config.qid_length;
        let digits: Vec<u8> = (0..len)
            .map(|i| seed[i % seed.len()] % 10)
            .collect();
        QNum::from_digits(&digits)
    }

    /// Register a new identity with given `qid` and `created` timestamp.
    ///
    /// Returns the `IdentityState` on success, or an error if already registered.
    pub fn register_identity(
        &mut self,
        qid: QNum,
        created: u64
    ) -> Result<IdentityState, QLinkError> {
        if self.registry.contains_key(&qid) {
            return Err(QLinkError::IdentityAlreadyExists(qid));
        }
        let state = IdentityState::new(qid.clone(), created);
        self.registry.insert(qid.clone(), state.clone());
        self.metrics.inc_counter("identities_registered", 1);
        Ok(state)
    }

    /// Look up an identity’s state by its `qid`.
    pub fn get_identity(&self, qid: &QNum) -> Option<&IdentityState> {
        self.registry.get(qid)
    }

    /// Revoke an existing identity, marking it as inactive.
    ///
    /// Returns the updated `IdentityState` or an error if not found.
    pub fn revoke_identity(&mut self, qid: &QNum) -> Result<IdentityState, QLinkError> {
        let state = self.registry.get_mut(qid)
            .ok_or_else(|| QLinkError::IdentityNotFound(qid.clone()))?;
        state.revoked = true;
        self.metrics.inc_counter("identities_revoked", 1);
        Ok(state.clone())
    }

    /// Export current metrics (e.g., for Prometheus).
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    fn default_cfg() -> QLinkConfig {
        let mut cfg = QLinkConfig::default();
        cfg.qid_length = 6;
        cfg
    }

    #[test]
    fn generate_qid_is_deterministic() {
        let cfg = default_cfg();
        let layer = QidLayer::new(&cfg);
        let seed = b"user123";
        let q1 = layer.generate_qid(seed);
        let q2 = layer.generate_qid(seed);
        assert_eq!(q1, q2);
        assert_eq!(q1.len(), cfg.qid_length);
    }

    #[test]
    fn register_and_lookup_identity() {
        let mut layer = QidLayer::new(&default_cfg());
        let seed = b"alice";
        let qid = layer.generate_qid(seed);
        let state = layer.register_identity(qid.clone(), 1_600_000_000).unwrap();
        assert_eq!(state.qid, qid);
        assert_eq!(state.created, 1_600_000_000);
        assert!(!state.revoked);

        let looked = layer.get_identity(&qid).unwrap();
        assert_eq!(looked.qid, qid);
    }

    #[test]
    fn duplicate_registration_fails() {
        let mut layer = QidLayer::new(&default_cfg());
        let qid = layer.generate_qid(b"bob");
        let _ = layer.register_identity(qid.clone(), 0).unwrap();
        let err = layer.register_identity(qid.clone(), 10).unwrap_err();
        matches!(err, QLinkError::IdentityAlreadyExists(_));
    }

    #[test]
    fn revoke_identity_marks_revoked() {
        let mut layer = QidLayer::new(&default_cfg());
        let qid = layer.generate_qid(b"carol");
        let _ = layer.register_identity(qid.clone(), 42).unwrap();
        let state = layer.revoke_identity(&qid).unwrap();
        assert!(state.revoked);

        // Revoking again still returns the same state
        let state2 = layer.revoke_identity(&qid).unwrap();
        assert!(state2.revoked);
    }

    #[test]
    fn revoke_missing_identity_fails() {
        let mut layer = QidLayer::new(&default_cfg());
        let missing = QNum::zero(6);
        let err = layer.revoke_identity(&missing).unwrap_err();
        matches!(err, QLinkError::IdentityNotFound(_));
    }

    #[test]
    fn metrics_increment_on_operations() {
        let mut layer = QidLayer::new(&default_cfg());
        let qid = layer.generate_qid(b"dan");
        let _ = layer.register_identity(qid.clone(), 0).unwrap();
        let _ = layer.revoke_identity(&qid).unwrap();
        let prom = layer.export_metrics();
        assert!(prom.contains("qlink_identities_registered 1"));
        assert!(prom.contains("qlink_identities_revoked 1"));
    }
}
