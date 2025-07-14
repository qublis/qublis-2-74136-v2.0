//! QLink Error Types
//!
//! Defines the `QLinkError` enum for errors encountered in the QLink crate,
//! including identity, principle, consent, and mutation errors.

use qublis_qnum::QNum;
use thiserror::Error;

/// Errors returned by QLink modules.
#[derive(Debug, Error)]
pub enum QLinkError {
    /// Attempted to register an identity that already exists.
    #[error("identity already exists: {0:?}")]
    IdentityAlreadyExists(QNum),

    /// Requested identity not found in registry.
    #[error("identity not found: {0:?}")]
    IdentityNotFound(QNum),

    /// Attempted to add a principle that already exists.
    #[error("principle already exists: {0}")]
    PrincipleExists(String),

    /// Requested principle not found in lattice.
    #[error("principle not found: {0}")]
    PrincipleNotFound(String),

    /// Error during consent operations.
    #[error("consent error: {0}")]
    ConsentError(String),

    /// Error during mutation engine operations.
    #[error("mutation error: {0}")]
    MutationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn test_identity_already_exists_error() {
        let qnum = QNum::from_digits(&[1, 2, 3]);
        let err = QLinkError::IdentityAlreadyExists(qnum.clone());
        let s = err.to_string();
        assert!(s.contains("identity already exists"));
        assert!(s.contains("123"));
    }

    #[test]
    fn test_identity_not_found_error() {
        let qnum = QNum::from_digits(&[4, 5, 6]);
        let err = QLinkError::IdentityNotFound(qnum.clone());
        let s = err.to_string();
        assert!(s.contains("identity not found"));
    }

    #[test]
    fn test_principle_exists_error() {
        let err = QLinkError::PrincipleExists("ethics".into());
        assert_eq!(err.to_string(), "principle already exists: ethics");
    }

    #[test]
    fn test_principle_not_found_error() {
        let err = QLinkError::PrincipleNotFound("fairness".into());
        assert_eq!(err.to_string(), "principle not found: fairness");
    }

    #[test]
    fn test_consent_error() {
        let err = QLinkError::ConsentError("already requested".into());
        assert_eq!(err.to_string(), "consent error: already requested");
    }

    #[test]
    fn test_mutation_error() {
        let err = QLinkError::MutationError("duplicate update".into());
        assert_eq!(err.to_string(), "mutation error: duplicate update");
    }
}
