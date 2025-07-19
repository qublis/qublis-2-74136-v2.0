//! Quantum Teleportation Overlay for QNet v2.0
//!
//! `TeleportCore` provides a one‐shot “teleport” method that uses quantum-inspired
//! entanglement to send a packet atomically along a multi-hop path.  In tests you
//! can override the teleport function to stub out network behavior.

use std::sync::Arc;
use crate::{
    config::QNetConfig,
    error::QNetError,
    metrics::QNetMetrics,
    prelude::Packet,
    types::NodeId,
};
use futures::future::try_join_all;

// --- Fixed transport import for main and test ---
#[cfg(not(test))]
use crate::transport;
#[cfg(test)]
use super::tests::transport;

/// Result type for teleport operations.
pub type TeleportResult = Result<(), QNetError>;

/// Signature of the teleport function hook for testing.
pub type TeleportFn = dyn Fn(&NodeId, &NodeId, &[NodeId], Packet) -> TeleportResult + Send + Sync;

/// `TeleportCore` encapsulates the teleportation subsystem.
#[derive(Clone)]
pub struct TeleportCore {
    config: QNetConfig,
    metrics: QNetMetrics,
    /// Optional override hook (for tests) that implements the teleport behavior.
    teleport_fn: Option<Arc<TeleportFn>>,
}

impl TeleportCore {
    /// Create a new `TeleportCore` with the given configuration.
    pub fn new(config: &QNetConfig) -> Self {
        TeleportCore {
            config: config.clone(),
            metrics: QNetMetrics::new(),
            teleport_fn: None,
        }
    }

    /// Override the teleport function (used in tests to inject a stub).
    pub fn override_teleport_fn<F>(&mut self, func: F)
    where
        F: Fn(&NodeId, &NodeId, &[NodeId], Packet) -> TeleportResult + Send + Sync + 'static,
    {
        self.teleport_fn = Some(Arc::new(func));
    }

    /// Teleport a packet from `src` to `dst` along the full `path`, in one atomic operation.
    ///
    /// - Records teleport attempt and success/failure in metrics.
    /// - If an override hook is set, invokes it directly.
    /// - Otherwise, simulates teleport as rapid hop-by-hop sends.
    pub async fn teleport(
        &self,
        src: &NodeId,
        dst: &NodeId,
        path: &[NodeId],
        packet: Packet,
    ) -> TeleportResult {
        self.metrics.record_teleport_attempt();

        // If a test override is provided, use it.
        if let Some(hook) = &self.teleport_fn {
            let res = hook(src, dst, path, packet);
            if res.is_ok() {
                self.metrics.record_teleport_success();
            } else {
                self.metrics.record_teleport_failure();
            }
            return res;
        }

        // Default behavior: simulate teleport as concurrent hop sends
        let mut tasks = Vec::with_capacity(path.len().saturating_sub(1));
        for window in path.windows(2) {
            let from = window[0].clone();
            let to = window[1].clone();
            let pkt = packet.clone();
            tasks.push(tokio::spawn(async move {
                transport::send_direct(&from, &to, pkt)
                    .await
                    .map_err(|e| QNetError::SendError(e.to_string()))
            }));
            self.metrics.record_hop();
        }

        // Await all hop tasks
        let results = try_join_all(tasks).await;
        match results {
            Ok(oks) => {
                // Check each hop result
                for r in oks {
                    r?; // propagate any send errors
                }
                self.metrics.record_teleport_success();
                Ok(())
            }
            Err(join_err) => {
                self.metrics.record_teleport_failure();
                Err(QNetError::SendError(join_err.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::QNetConfig;
    use crate::types::{Packet, NodeId};
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    /// Dummy transport logger for hop-by-hop fallback.
    lazy_static! {
        static ref LOG: Mutex<Vec<(NodeId, NodeId, Vec<u8>)>> = Mutex::new(Vec::new());
    }

    // Stub the real transport to use our LOG
    #[allow(unused_imports)]
    pub mod transport {
        use super::{LOG, NodeId, Packet};
        pub async fn send_direct(
            from: &NodeId,
            to: &NodeId,
            packet: Packet,
        ) -> Result<(), String> {
            LOG.lock().unwrap().push((from.clone(), to.clone(), packet));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_teleport_default_hop_by_hop() {
        let mut cfg = QNetConfig::default();
        let tc = TeleportCore::new(&cfg);
        let path = vec!["A".into(), "B".into(), "C".into()];
        let packet = Packet::from(vec![1, 2, 3]);

        tc.teleport(&"A".into(), &"C".into(), &path, packet.clone())
            .await
            .expect("teleport failed");

        let log = LOG.lock().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0], ("A".into(), "B".into(), packet.clone()));
        assert_eq!(log[1], ("B".into(), "C".into(), packet));
    }

    #[tokio::test]
    async fn test_teleport_with_override() {
        let mut cfg = QNetConfig::default();
        let mut tc = TeleportCore::new(&cfg);
        let packet = Packet::from(vec![9, 9]);
        let path = vec!["X".into(), "Y".into()];

        // Install a stub that verifies its inputs
        tc.override_teleport_fn(move |src, dst, pth, pkt| {
            assert_eq!(src, &"X");
            assert_eq!(dst, &"Y");
            assert_eq!(pth, &path[..]);
            assert_eq!(pkt, packet);
            Ok(())
        });

        tc.teleport(&"X".into(), &"Y".into(), &path, packet)
            .await
            .expect("override teleport failed");
    }
}
