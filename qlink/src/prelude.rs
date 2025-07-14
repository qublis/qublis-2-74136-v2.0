//! QLink Prelude
//!
//! Common imports and re‐exports for the QLink quantum identity, ethics, consent,
//! and mutation‐engine crate (Qublis v2.0).

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::config::QLinkConfig;
pub use crate::qid_layer::QidLayer;
pub use crate::ethics_lattice::EthicsLattice;
pub use crate::conscious_consent::ConsciousConsent;
pub use crate::mutation_engine::MutationEngine;
pub use crate::types::{IdentityState, ConsentRecord, PolicyUpdate};
pub use crate::metrics::QLinkMetrics;
pub use crate::error::QLinkError;

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;
    use crate::types::{IdentityState, ConsentRecord, PolicyUpdate};

    #[test]
    fn prelude_reexports_compile() {
        // Config
        let cfg: QLinkConfig = QLinkConfig::default();

        // QidLayer
        let mut qid_layer = QidLayer::new(&cfg);
        let seed = b"test";
        let qnum = qid_layer.generate_qid(seed);
        let state: IdentityState = qid_layer.register_identity(qnum.clone(), 0).unwrap();
        assert_eq!(state.created, 0);
        assert!(!state.revoked);

        // EthicsLattice
        let mut lattice = EthicsLattice::new(&cfg);
        lattice.add_principle("fairness".into(), QNum::from_digits(&[5])).unwrap();
        let _ = lattice.evaluate();

        // ConsciousConsent
        let mut consent = ConsciousConsent::new(&cfg);
        let rec: ConsentRecord = consent.request_consent(&qnum, "terms", 42).unwrap();
        assert_eq!(rec.timestamp, 42);

        // MutationEngine
        let mut engine = MutationEngine::new(&cfg);
        let update = PolicyUpdate {
            policy_id: "upd".to_string(),
            parameters: QNum::from_digits(&[1]),
            timestamp: 1,
        };
        engine.record_update(&qnum, update.clone()).unwrap();
        let new_state = engine.apply_updates(&state).unwrap();
        assert_eq!(new_state.created, 0);

        // Metrics export
        let metrics_txt = qid_layer.export_metrics();
        assert!(metrics_txt.contains("qlink_identities_registered"));

        // Error formatting
        let err: QLinkError = QLinkError::ConsentError("denied".into());
        assert!(err.to_string().contains("consent error: denied"));
    }
}
