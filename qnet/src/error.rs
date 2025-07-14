//! QNet Error Types
//!
//! Defines the `QNetError` enum for errors encountered during routing,
//! packet relay, and teleportation.

use crate::types::NodeId;
use thiserror::Error;

/// Errors returned by QNet routing, relay, and teleport operations.
#[derive(Debug, Error)]
pub enum QNetError {
    /// No route exists between the given source and destination.
    #[error("No path found from `{0}` to `{1}`")]
    NoPath(NodeId, NodeId),

    /// A failure occurred while sending a packet.
    #[error("Transport send error: {0}")]
    SendError(String),

    /// Teleportation overlay failed.
    #[error("Teleport error: {0}")]
    TeleportError(String),

    /// Configuration or internal error.
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_no_path() {
        let err = QNetError::NoPath("A".into(), "B".into());
        let msg = format!("{}", err);
        assert!(msg.contains("No path found from `A` to `B`"));
    }

    #[test]
    fn display_send_error() {
        let err = QNetError::SendError("timeout".into());
        let msg = format!("{}", err);
        assert_eq!(msg, "Transport send error: timeout");
    }

    #[test]
    fn display_teleport_error() {
        let err = QNetError::TeleportError("broken entanglement".into());
        assert_eq!(err.to_string(), "Teleport error: broken entanglement");
    }

    #[test]
    fn display_config_error() {
        let err = QNetError::ConfigError("invalid k_paths".into());
        assert_eq!(err.to_string(), "Configuration error: invalid k_paths");
    }
}
