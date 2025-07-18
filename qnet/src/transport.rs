//! Transport layer for QNet (2-74136).
//!
//! Configures and instantiates the underlying libp2p transport
//! (TCP + Noise XX + Yamux), and provides convenience methods
//! to listen and dial peers.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::str::FromStr;

use libp2p::{
    core::upgrade,
    identity,
    multiaddr::Multiaddr,
    noise::{self, AuthenticKeypair, X25519Spec, NoiseConfig},
    tcp::TokioTcpConfig,
    yamux::YamuxConfig,
    Transport,
    PeerId,
};

use crate::{
    config::Config,
    error::NetworkError,
};

/// Opaque boxed transport type.
pub type BoxedTransport = libp2p::core::transport::Boxed;

/// QNetTransport wraps a libp2p transport stack plus the local PeerId.
#[derive(Debug)]
pub struct QNetTransport {
    /// The composed, boxed transport.
    pub transport: BoxedTransport,
    /// Local node's PeerId derived from the identity keypair.
    pub local_peer_id: PeerId,
}

impl QNetTransport {
    /// Build a new transport from the given `Config`.
    ///
    /// Generates an ED25519 identity, configures TCP(no-delay) + Noise XX + Yamux.
    pub fn new(cfg: &Config) -> Result<Self, NetworkError> {
        // 1. Generate ed25519 identity keypair
        let id_keys = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(id_keys.public());

        // 2. Build Noise XX keypair for authenticated encryption
        let noise_keys = noise::Keypair::<X25519Spec>::new()
            .into_authentic(&id_keys)
            .map_err(|e| NetworkError::Transport(format!("noise handshake failed: {}", e)))?;

        // 3. Assemble TCP + Noise XX + Yamux transport stack
        let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(YamuxConfig::default())
            .boxed();

        Ok(QNetTransport { transport, local_peer_id })
    }

    /// Listen on the configured address from `cfg.listen_addr`.
    ///
    /// Returns the parsed listen Multiaddr on success.
    pub fn listen(&mut self, addr: &str) -> Result<Multiaddr, NetworkError> {
        let ma = Multiaddr::from_str(addr)
            .map_err(|e| NetworkError::Transport(format!("invalid listen addr `{}`: {}", addr, e)))?;
        self.transport
            .listen_on(ma.clone())
            .map_err(|e| NetworkError::Transport(format!("listen_on error: {}", e)))?;
        Ok(ma)
    }

    /// Dial a remote peer at the given multiaddr.
    pub fn dial(&mut self, addr: &str) -> Result<(), NetworkError> {
        let ma = Multiaddr::from_str(addr)
            .map_err(|e| NetworkError::Transport(format!("invalid dial addr `{}`: {}", addr, e)))?;
        self.transport
            .dial(ma.clone())
            .map_err(|e| NetworkError::Transport(format!("dial error: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    /// Construct a dummy Config for testing.
    fn dummy_config() -> Config {
        Config {
            listen_addr: "/ip4/0.0.0.0/tcp/0".into(),
            bootstrap_peers: Vec::new(),
            max_connections: 32,
        }
    }

    #[test]
    fn new_transport_succeeds() {
        let cfg = dummy_config();
        let t = QNetTransport::new(&cfg).expect("should build transport");
        // local_peer_id should be non-empty
        assert!(!t.local_peer_id.to_base58().is_empty());
    }

    #[test]
    fn listen_invalid_addr_returns_error() {
        let cfg = dummy_config();
        let mut t = QNetTransport::new(&cfg).unwrap();
        assert!(t.listen("not-a-multiaddr").is_err());
    }

    #[test]
    fn dial_invalid_addr_returns_error() {
        let cfg = dummy_config();
        let mut t = QNetTransport::new(&cfg).unwrap();
        assert!(t.dial("also-not-ma").is_err());
    }
}
