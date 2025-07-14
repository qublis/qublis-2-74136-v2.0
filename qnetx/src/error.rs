//! QNetX Error Types
//!
//! Defines the `QNetXError` enum for errors encountered in the entangled overlay mesh:
//! handshake failures, channel lookups, condensation errors, serialization, I/O, etc.

use crate::types::ChannelId;
use thiserror::Error;

/// Errors returned by QNetX operations.
#[derive(Debug, Error)]
pub enum QNetXError {
    /// Failure during peer‐to‐peer handshake.
    #[error("Handshake error: {0}")]
    HandshakeError(String),

    /// Requested channel ID was not found.
    #[error("Channel not found: {0:?}")]
    ChannelNotFound(ChannelId),

    /// Error during state condensation (e.g., no channels to condense).
    #[error("Condensation error: {0}")]
    CondensationError(String),

    /// Teleportation overlay failure.
    #[error("Teleport error: {0}")]
    TeleportError(String),

    /// Error serializing or deserializing data (e.g., JSON).
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// I/O error (e.g., reading/writing sockets).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChannelId;

    #[test]
    fn display_handshake_error() {
        let err = QNetXError::HandshakeError("bad payload".into());
        assert_eq!(err.to_string(), "Handshake error: bad payload");
    }

    #[test]
    fn display_channel_not_found() {
        let id: ChannelId = vec![1, 2, 3];
        let err = QNetXError::ChannelNotFound(id.clone());
        assert!(err.to_string().contains("Channel not found"));
        assert!(err.to_string().contains("[1, 2, 3]"));
    }

    #[test]
    fn display_condensation_error() {
        let err = QNetXError::CondensationError("no channels".into());
        assert_eq!(err.to_string(), "Condensation error: no channels");
    }

    #[test]
    fn from_serde_error() {
        let json = "not valid json";
        let res: Result<ChannelId, QNetXError> = Err(serde_json::from_str::<ChannelId>(json).unwrap_err().into());
        if let Err(QNetXError::SerializationError(_)) = res {
            // OK
        } else {
            panic!("Expected SerializationError");
        }
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
        let err: QNetXError = io_err.into();
        assert_eq!(err.to_string(), "I/O error: disk full");
    }
}
