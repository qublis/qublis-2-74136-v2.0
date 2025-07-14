//! Core data types for QLink — Qublis v2.0
//!
//! Defines the on‐chain identity, consent, and mutation types used by the QidLayer,
//! ConsciousConsent, and MutationEngine modules.

use serde::{Deserialize, Serialize};
use qublis_qnum::QNum;

/// Represents the on‐chain state of a quantum identity (QID).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityState {
    /// The underlying quantum identifier.
    pub qid: QNum,
    /// UNIX timestamp when the identity was created.
    pub created: u64,
    /// Whether the identity has been revoked.
    pub revoked: bool,
}

impl IdentityState {
    /// Construct a new `IdentityState` with `revoked = false`.
    pub fn new(qid: QNum, created: u64) -> Self {
        IdentityState { qid, created, revoked: false }
    }
}

/// A record of a user’s consent decision.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Quantum identity to which this consent applies.
    pub qid: QNum,
    /// Text of the terms under which consent was requested.
    pub terms: String,
    /// Whether consent was granted (`true`) or denied (`false`).
    pub granted: bool,
    /// UNIX timestamp when the consent was recorded.
    pub timestamp: u64,
}

/// A policy update to be applied to an identity’s QID state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyUpdate {
    /// Unique identifier for this policy change.
    pub policy_id: String,
    /// Parameters of the update, represented as a `QNum`.
    pub parameters: QNum,
    /// UNIX timestamp when the update was issued.
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn identity_state_new_defaults_revoked_false() {
        let q = QNum::from_digits(&[1,2,3]);
        let state = IdentityState::new(q.clone(), 1_600_000_000);
        assert_eq!(state.qid.measure(), q.measure());
        assert_eq!(state.created, 1_600_000_000);
        assert!(!state.revoked);
    }

    #[test]
    fn consent_record_fields_roundtrip() {
        let q = QNum::from_digits(&[4,2,0]);
        let rec = ConsentRecord {
            qid: q.clone(),
            terms: "T&C".into(),
            granted: true,
            timestamp: 12345,
        };
        assert_eq!(rec.qid.measure(), q.measure());
        assert_eq!(rec.terms, "T&C");
        assert!(rec.granted);
        assert_eq!(rec.timestamp, 12345);
    }

    #[test]
    fn policy_update_fields() {
        let params = QNum::from_digits(&[9,9]);
        let upd = PolicyUpdate {
            policy_id: "P1".into(),
            parameters: params.clone(),
            timestamp: 999,
        };
        assert_eq!(upd.policy_id, "P1");
        assert_eq!(upd.parameters.measure(), params.measure());
        assert_eq!(upd.timestamp, 999);
    }
}
