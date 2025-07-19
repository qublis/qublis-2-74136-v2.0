//! Packet Relay for Qublis QNet v2.0  
//!  
//! The `Relay` struct routes and forwards packets through the network using  
//! quantum‐inspired probabilistic path selection.  If teleportation is enabled,  
//! it will use `TeleportCore` to quantum‐teleport the packet along the chosen path.  

use crate::{
    config::QNetConfig,
    error::QNetError,
    metrics::QNetMetrics,
    prelude::{Router, TeleportCore},
    types::{NodeId, Packet},
};
use futures::future::try_join_all;

// Remove this import since crate::transport does not exist
// #[cfg(not(test))]
// use crate::transport;

#[cfg(test)]
use super::tests::dummy_transport as transport;

// For production, define a stub or trait for transport.
// For now, define a module for the relay to use. In real code, move to its own file.
#[cfg(not(test))]
mod transport {
    use super::{NodeId, Packet, QNetError};
    pub async fn send_direct(
        _from: &NodeId,
        _to: &NodeId,
        _packet: Packet,
    ) -> Result<(), String> {
        // In production, implement real transport logic here.
        // For now, just Ok(()).
        Ok(())
    }
}
#[cfg(not(test))]
use transport as transport;

/// Packet relay engine.
#[derive(Clone)]
pub struct Relay {
    config: QNetConfig,
    router: Router,
    teleport: Option<TeleportCore>,
    metrics: QNetMetrics,
}

impl Relay {
    /// Create a new `Relay` with the given configuration.
    pub fn new(config: &QNetConfig) -> Self {
        let teleport = if config.enable_teleport {
            Some(TeleportCore::new(config))
        } else {
            None
        };
        Relay {
            config: config.clone(),
            router: Router::new(config),
            teleport,
            metrics: QNetMetrics::new(),
        }
    }

    /// Relay a packet from `src` to `dst`.
    ///  
    /// 1. Uses the `Router` to select a path (collapse a `QNum` superposition).  
    /// 2. Records path metrics.  
    /// 3. If teleportation is enabled, invokes `TeleportCore::teleport` to send  
    ///    the packet atomically along the entire path.  
    /// 4. Otherwise, forwards hop-by-hop.  
    pub async fn relay(
        &mut self,
        src: &NodeId,
        dst: &NodeId,
        packet: Packet,
    ) -> Result<(), QNetError> {
        // Record attempt
        self.metrics.record_relay_attempt();

        // Step 1: probabilistic path selection
        let path = self.router.route(src, dst)?;
        self.metrics.record_path_length(path.len());

        // Step 2: teleport or hop-by-hop
        if let Some(tc) = &self.teleport {
            // Teleport the packet in one shot
            tc.teleport(src, dst, &path, packet)
                .await
                .map_err(QNetError::from)?;
            self.metrics.record_teleport_success();
        } else {
            // Forward along each hop
            // we clone packet for each hop; in real usage you'd stream or consume
            let mut tasks = Vec::new();
            for window in path.windows(2) {
                let from = window[0].clone();
                let to = window[1].clone();
                let pkt = packet.clone();
                tasks.push(tokio::spawn(async move {
                    // send_direct is a placeholder for your transport layer
                    transport::send_direct(&from, &to, pkt)
                        .await
                        .map_err(|e| QNetError::SendError(e.to_string()))
                }));
                self.metrics.record_hop();
            }
            // wait for all hops to complete
            try_join_all(tasks)
                .await
                .map_err(|e| QNetError::SendError(e.to_string()))?
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
        }

        // Record success
        self.metrics.record_relay_success();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::QNetConfig;
    use crate::types::Packet;

    /// A dummy transport implementation for tests.
    pub mod dummy_transport {
        use super::{NodeId, Packet};
        use std::sync::Mutex;
        lazy_static::lazy_static! {
            pub static ref LOG: Mutex<Vec<(NodeId, NodeId, Vec<u8>)>> = Mutex::new(vec![]);
        }
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
    async fn test_relay_hop_by_hop() {
        let mut cfg = QNetConfig::default();
        cfg.enable_teleport = false;
        cfg.k_paths = 1;
        let mut relay = Relay::new(&cfg);
        // Build graph: A-B-C
        relay.router.add_edge("A".into(), "B".into());
        relay.router.add_edge("B".into(), "C".into());

        let packet = Packet::from(vec![1, 2, 3]);
        relay.relay(&"A".into(), &"C".into(), packet.clone()).await.unwrap();

        let log = dummy_transport::LOG.lock().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].0, "A");
        assert_eq!(log[0].1, "B");
        assert_eq!(log[1].0, "B");
        assert_eq!(log[1].1, "C");
    }

    #[tokio::test]
    async fn test_relay_with_teleport() {
        let mut cfg = QNetConfig::default();
        cfg.enable_teleport = true;
        cfg.k_paths = 1;
        let mut relay = Relay::new(&cfg);
        relay.router.add_edge("X".into(), "Y".into());

        // Stub TeleportCore to record calls
        let packet = Packet::from(vec![9, 9]);
        relay
            .teleport
            .as_mut()
            .unwrap()
            .override_teleport_fn(|_src, _dst, path, pkt| {
                assert_eq!(path, &vec!["X".into(), "Y".into()]);
                assert_eq!(pkt, packet);
                Ok(())
            });

        relay.relay(&"X".into(), &"Y".into(), packet).await.unwrap();
    }
}
