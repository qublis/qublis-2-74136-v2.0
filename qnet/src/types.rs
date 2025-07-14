//! Core data types for QNet routing & relay.
//!
//! Defines `NodeId`, `Path`, and `Packet` types used throughout the QNet crate.

use serde::{Deserialize, Serialize};

/// Unique identifier for a network node.
pub type NodeId = String;

/// A route between two nodes, expressed as an ordered list of `NodeId`s.
pub type Path = Vec<NodeId>;

/// A network packet payload.
///  
/// Wraps a vector of bytes; you can construct with `Packet::from(vec![...])`
/// or extract the raw bytes via `.into_inner()`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Packet {
    payload: Vec<u8>,
}

impl Packet {
    /// Create a new `Packet` from raw bytes.
    pub fn new(payload: Vec<u8>) -> Self {
        Packet { payload }
    }

    /// Consume the `Packet`, returning the raw byte vector.
    pub fn into_inner(self) -> Vec<u8> {
        self.payload
    }

    /// Borrow the packet payload as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.payload
    }

    /// Packet length in bytes.
    pub fn len(&self) -> usize {
        self.payload.len()
    }

    /// Returns true if the packet has no payload.
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

impl From<Vec<u8>> for Packet {
    fn from(payload: Vec<u8>) -> Self {
        Packet::new(payload)
    }
}

impl From<Packet> for Vec<u8> {
    fn from(pkt: Packet) -> Vec<u8> {
        pkt.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_from_and_into_vec() {
        let data = vec![1, 2, 3, 255];
        let pkt = Packet::from(data.clone());
        assert_eq!(pkt.as_slice(), &[1, 2, 3, 255]);
        let recovered: Vec<u8> = pkt.into_inner();
        assert_eq!(recovered, data);
    }

    #[test]
    fn packet_len_and_empty() {
        let empty = Packet::new(vec![]);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let pkt = Packet::new(vec![42]);
        assert!(!pkt.is_empty());
        assert_eq!(pkt.len(), 1);
    }

    #[test]
    fn path_and_nodeid_aliases() {
        let a: NodeId = "NodeA".to_string();
        let b: NodeId = "NodeB".to_string();
        let path: Path = vec![a.clone(), b.clone(), a.clone()];
        assert_eq!(path, vec!["NodeA".to_string(), "NodeB".to_string(), "NodeA".to_string()]);
    }
}
