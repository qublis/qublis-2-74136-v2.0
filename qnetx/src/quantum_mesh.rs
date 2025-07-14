//! QuantumMesh — Entangled Overlay Mesh for Qublis v2.0
//!
//! Manages a set of entangled “channels” (QNum states) between logical
//! dimensions.  Supports peer‐to‐peer handshake to establish new channels,
//! stores them by `ChannelId`, and provides access for routing and teleport.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use qublis_qnum::{QNum, entangle};
use crate::config::QNetXConfig;
use crate::metrics::QNetXMetrics;
use crate::error::QNetXError;
use crate::types::{Dimension, ChannelId};

/// QuantumMesh holds entangled channels between dimensions.
#[derive(Clone, Debug)]
pub struct QuantumMesh {
    config: Arc<QNetXConfig>,
    metrics: QNetXMetrics,
    /// Map from channel identifier to its QNum state.
    channels: HashMap<ChannelId, QNum>,
}

impl QuantumMesh {
    /// Create a new `QuantumMesh` with the given config.
    pub fn new(config: &QNetXConfig) -> Self {
        QuantumMesh {
            config: Arc::new(config.clone()),
            metrics: QNetXMetrics::new(),
            channels: HashMap::new(),
        }
    }

    /// Access the underlying configuration.
    pub fn config(&self) -> &QNetXConfig {
        &self.config
    }

    /// Connect to a peer at `addr` and perform overlay handshake.
    pub async fn connect(&self, addr: String) -> Result<(), QNetXError> {
        let mut stream = TcpStream::connect(&addr).await?;
        // Send our local dimension IDs as "dimA,dimB\n"
        let dims = &self.config.bootstrap_nodes; // reuse bootstrap_nodes as dimension names
        if dims.len() < 2 {
            return Err(QNetXError::HandshakeError("need at least two dimensions".into()));
        }
        let payload = format!("{},{}\n", dims[0], dims[1]);
        stream.write_all(payload.as_bytes()).await?;
        // Wait for peer to respond with ChannelId JSON
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        let channel_id: ChannelId = serde_json::from_slice(&buf)?;
        self.metrics.record_channel_created();
        Ok(())
    }

    /// Handle an incoming overlay connection: read two dimension names,
    /// entangle a new channel, and write back the `ChannelId` as JSON.
    pub async fn handle_connection(&self, mut stream: TcpStream) -> Result<(), QNetXError> {
        let mut buf = Vec::new();
        // Read until EOF or newline
        let _ = stream.read_to_end(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf);
        let parts: Vec<&str> = msg.trim_end().split(',').collect();
        if parts.len() != 2 {
            return Err(QNetXError::HandshakeError(format!(
                "invalid handshake payload: {}",
                msg
            )));
        }
        let dim_a = Dimension(parts[0].to_string());
        let dim_b = Dimension(parts[1].to_string());

        // Entangle a new channel between dim_a and dim_b
        let channel_id = self.entangle_channel(&dim_a, &dim_b);

        // Send back the ChannelId as JSON
        let resp = serde_json::to_vec(&channel_id)?;
        stream.write_all(&resp).await?;
        Ok(())
    }

    /// Create and store a new entangled channel between dimensions `a` and `b`.
    ///
    /// Returns a `ChannelId` (vector of digits) that can be used to retrieve the QNum.
    pub fn entangle_channel(&self, a: &Dimension, b: &Dimension) -> ChannelId {
        // Derive initial digit vector from dimension names (take ASCII mods)
        let mut seed: Vec<u8> = a.0.bytes().chain(b.0.bytes())
            .map(|b| (b % 10) as u8)
            .take(4)
            .collect();
        while seed.len() < 4 {
            seed.push(0);
        }

        // Build a QNum from that seed
        let mut channel_q = QNum::from_digits(&seed);

        // Entangle each digit with itself to spread amplitude
        entangle(&mut channel_q, &mut channel_q.clone());

        // Collapse to obtain the ChannelId
        let channel_id = channel_q.clone().measure();

        // Store the entangled state
        self.channels
            .insert(channel_id.clone(), channel_q);

        // Record metric
        self.metrics.record_entanglement();

        channel_id
    }

    /// Retrieve the `QNum` state for the given `ChannelId`, if it exists.
    pub fn get_channel(&self, id: &ChannelId) -> Option<&QNum> {
        self.channels.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use crate::config::QNetXConfig;
    use crate::types::{Dimension, ChannelId};
    use std::net::SocketAddr;
    use serde_json;

    #[test]
    fn test_entangle_channel_generates_id() {
        let cfg = QNetXConfig::default();
        let mesh = QuantumMesh::new(&cfg);
        let dim_a = Dimension("A".into());
        let dim_b = Dimension("B".into());
        let id = mesh.entangle_channel(&dim_a, &dim_b);
        assert_eq!(id.len(), 4);
        // Channel state stored
        assert!(mesh.get_channel(&id).is_some());
    }

    #[test]
    fn test_handle_connection_and_connect() {
        // Run a mini server and client on localhost
        let rt = Runtime::new().unwrap();
        let cfg = QNetXConfig {
            bootstrap_nodes: vec!["A".into(), "B".into()],
            ..Default::default()
        };
        let mesh = QuantumMesh::new(&cfg);
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn server task
        rt.spawn(async move {
            let (sock, _) = listener.accept().unwrap();
            let stream = TcpStream::from_std(sock).unwrap();
            mesh.handle_connection(stream).await.unwrap();
        });

        // Client connects and sends handshake
        rt.block_on(async {
            let client_mesh = QuantumMesh::new(&cfg);
            client_mesh.connect(addr.to_string()).await.unwrap();
        });
    }
}
