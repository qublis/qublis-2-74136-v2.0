//! Conscious Consent module for QLink — Qublis v2.0
//!
//! Manages user consent as quantum‐number states (`QNum`), with probabilistic
//! superposition based on configured consent probability.  Records consent
//! grants and revocations as `ConsentRecord`s, and exposes metrics.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;
use qublis_qnum::{QNum, Qid};
use crate::{
    config::QLinkConfig,
    error::QLinkError,
    metrics::QLinkMetrics,
    types::ConsentRecord,
};

/// `ConsciousConsent` manages consent records per QID.
#[derive(Clone, Debug)]
pub struct ConsciousConsent {
    config: QLinkConfig,
    metrics: QLinkMetrics,
    /// Registry: QID → consent record
    consents: HashMap<QNum, ConsentRecord>,
}

impl ConsciousConsent {
    /// Create a new `ConsciousConsent` manager.
    pub fn new(config: &QLinkConfig) -> Self {
        ConsciousConsent {
            config: config.clone(),
            metrics: QLinkMetrics::new(),
            consents: HashMap::new(),
        }
    }

    /// Request consent for the given `qid` under `terms` at `timestamp`.
    ///
    /// Generates a single‐digit superposed QNum with amplitudes
    /// √p for consent (digit 1) and √(1−p) for denial (digit 0),
    /// where `p = config.consent_probability`.
    /// Measures (collapses) to a classical boolean, stores the record,
    /// and returns it.
    pub fn request_consent(
        &mut self,
        qid: &QNum,
        terms: &str,
        timestamp: u64,
    ) -> Result<ConsentRecord, QLinkError> {
        // Prevent duplicate requests
        if self.consents.contains_key(qid) {
            return Err(QLinkError::ConsentError("already requested".into()));
        }

        // Build superposed Qid: |0⟩ vs |1⟩
        let p = self.config.consent_probability;
        let amp_yes = (p).sqrt();
        let amp_no = (1.0 - p).sqrt();
        let mut super_qid = Qid::new([
            num_complex::Complex::new(amp_no, 0.0), // |0⟩: denial
            num_complex::Complex::new(amp_yes, 0.0), // |1⟩: consent
            // rest of basis unused
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
            num_complex::Complex::new(0.0, 0.0),
        ]);

        // Collapse to classical digit
        let result = super_qid.measure();
        let granted = result == 1;

        // Build and store the record
        let record = ConsentRecord {
            qid: qid.clone(),
            terms: terms.to_string(),
            granted,
            timestamp,
        };
        self.consents.insert(qid.clone(), record.clone());
        self.metrics.inc_counter("consents_requested", 1);
        if granted {
            self.metrics.inc_counter("consents_granted", 1);
        } else {
            self.metrics.inc_counter("consents_denied", 1);
        }
        Ok(record)
    }

    /// Retrieve the existing `ConsentRecord` for `qid`, if any.
    pub fn get_consent(&self, qid: &QNum) -> Option<&ConsentRecord> {
        self.consents.get(qid)
    }

    /// Revoke consent for `qid`, updating the record’s `granted` flag to `false`.
    /// Returns the updated record or an error if not found.
    pub fn revoke_consent(&mut self, qid: &QNum) -> Result<ConsentRecord, QLinkError> {
        let rec = self
            .consents
            .get_mut(qid)
            .ok_or_else(|| QLinkError::ConsentError("no existing consent".into()))?;
        rec.granted = false;
        self.metrics.inc_counter("consents_revoked", 1);
        Ok(rec.clone())
    }

    /// Export consent metrics in Prometheus format.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    /// Helper to create a dummy QNum identity.
    fn dummy_qnum() -> QNum {
        QNum::from_digits(&[4, 2, 0]) // identity "420"
    }

    #[test]
    fn request_and_get_consent() {
        let mut cfg = QLinkConfig::default();
        cfg.consent_probability = 1.0; // always grant
        let mut cc = ConsciousConsent::new(&cfg);

        let qid = dummy_qnum();
        let rec = cc.request_consent(&qid, "terms", 12345).unwrap();
        assert!(rec.granted);
        assert_eq!(rec.terms, "terms");
        assert_eq!(rec.timestamp, 12345);

        let got = cc.get_consent(&qid).unwrap();
        assert_eq!(got.granted, true);
    }

    #[test]
    fn duplicate_request_fails() {
        let mut cfg = QLinkConfig::default();
        cfg.consent_probability = 0.5;
        let mut cc = ConsciousConsent::new(&cfg);

        let qid = dummy_qnum();
        let _ = cc.request_consent(&qid, "t", 1).unwrap();
        let err = cc.request_consent(&qid, "t", 2).unwrap_err();
        matches!(err, QLinkError::ConsentError(_));
    }

    #[test]
    fn revoke_consent() {
        let mut cfg = QLinkConfig::default();
        cfg.consent_probability = 1.0;
        let mut cc = ConsciousConsent::new(&cfg);

        let qid = dummy_qnum();
        let _ = cc.request_consent(&qid, "t", 100).unwrap();
        let rec = cc.revoke_consent(&qid).unwrap();
        assert!(!rec.granted);

        // Nonexistent revoke fails
        let missing = QNum::from_digits(&[0]);
        let err = cc.revoke_consent(&missing).unwrap_err();
        matches!(err, QLinkError::ConsentError(_));
    }

    #[test]
    fn metrics_counted() {
        let mut cfg = QLinkConfig::default();
        cfg.consent_probability = 0.5;
        let mut cc = ConsciousConsent::new(&cfg);

        let qid = dummy_qnum();
        let _ = cc.request_consent(&qid, "x", 0).unwrap();
        let _ = cc.revoke_consent(&qid).unwrap();
        let prom = cc.export_metrics();
        assert!(prom.contains("qlink_consents_requested 1"));
        assert!(prom.contains("qlink_consents_revoked 1"));
    }
}
