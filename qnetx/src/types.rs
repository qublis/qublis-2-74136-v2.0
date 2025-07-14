//! Core types for QNetX entangled overlay mesh (Qublis v2.0)
//!
//! Defines `Dimension` (a named logical dimension) and `ChannelId`
//! (the identifier for an entangled channel, as a sequence of digits).

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;

/// A logical “dimension” in the QNetX mesh, identified by name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimension(pub String);

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dimension({})", self.0)
    }
}

impl From<String> for Dimension {
    fn from(s: String) -> Self {
        Dimension(s)
    }
}

impl From<&str> for Dimension {
    fn from(s: &str) -> Self {
        Dimension(s.to_string())
    }
}

/// Identifier for an entangled channel: a sequence of decimal digits.
pub type ChannelId = Vec<u8>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn dimension_roundtrip_serde() {
        let dim = Dimension("alpha".into());
        let json = serde_json::to_string(&dim).unwrap();
        assert_eq!(json, r#""alpha""#);
        let de: Dimension = serde_json::from_str(&json).unwrap();
        assert_eq!(de, dim);
    }

    #[test]
    fn channel_id_alias_behaves_as_vec() {
        let id: ChannelId = vec![1, 2, 3, 4];
        assert_eq!(id.len(), 4);
        assert_eq!(id, vec![1, 2, 3, 4]);
        // JSON serialize a ChannelId
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "[1,2,3,4]");
        let de: ChannelId = serde_json::from_str(&json).unwrap();
        assert_eq!(de, id);
    }

    #[test]
    fn display_dimension_shows_wrapper() {
        let dim = Dimension("beta".into());
        assert_eq!(format!("{}", dim), "Dimension(beta)");
    }
}
