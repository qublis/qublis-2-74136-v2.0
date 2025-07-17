//! QNet Prelude
//! ------------------------
//! Common imports and re‐exports for the QNet routing & relay crate.

pub use crate::config::QNetConfig;
pub use crate::router::Router;
pub use crate::relay::Relay;
pub use crate::teleport_core::TeleportCore;
pub use crate::types::{NodeId, Path, Packet};
pub use crate::metrics::QNetMetrics;
pub use crate::error::QNetError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_reexports() {
        // Config
        let cfg: QNetConfig = QNetConfig::default();
        // Router
        let mut router = Router::new(&cfg);
        router.add_edge("A".into(), "B".into());
        // Relay
        let mut relay = Relay::new(&cfg);
        // TeleportCore
        let tc = TeleportCore::new(&cfg);
        // Types
        let src: NodeId = "A".into();
        let dst: NodeId = "B".into();
        let path: Path = vec![src.clone(), dst.clone()];
        let pkt: Packet = Packet::from(vec![0x01, 0x02]);
        assert_eq!(path, vec!["A".into(), "B".into()]);
        assert_eq!(pkt.as_slice(), &[0x01, 0x02]);
        // Metrics
        let mut metrics = QNetMetrics::new();
        metrics.record_relay_attempt();
        metrics.record_path_length(2);
        // Error
        let err: QNetError = QNetError::NoPath(src.clone(), dst.clone());
        let msg = err.to_string();
        assert!(msg.contains("No path found from"));
    }
}
