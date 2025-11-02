mod builder;

pub use builder::Builder;
use serde::{Deserialize, Serialize};

/// JWT Header structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JwtHeader {
    pub(crate) alg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) kid: Option<String>,
    pub(crate) typ: String,
}

impl JwtHeader {
    /// Creates a new JWT header with algorithm and optional key ID
    ///
    /// This is a convenience function that uses the Builder internally.
    ///
    /// # Arguments
    /// * `alg` - The algorithm identifier (e.g., "ML-DSA-65")
    /// * `kid` - Optional key ID for key rotation
    ///
    /// # Example
    /// ```
    /// use pq_jwt::header::JwtHeader;
    ///
    /// let header = JwtHeader::new("ML-DSA-65", Some("key-2024-01"));
    /// ```
    pub fn new(alg: impl Into<String>, kid: Option<impl Into<String>>) -> Self {
        let mut builder = Builder::new().algorithm(alg);
        if let Some(k) = kid {
            builder = builder.kid(k);
        }
        // Unwrap is safe because algorithm is always set
        builder.build().unwrap()
    }

    /// Gets the algorithm
    pub fn algorithm(&self) -> &str {
        &self.alg
    }

    /// Gets the key ID if present
    pub fn key_id(&self) -> Option<&str> {
        self.kid.as_deref()
    }

    /// Gets the type
    pub fn typ(&self) -> &str {
        &self.typ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_kid() {
        let header = JwtHeader::new("ML-DSA-65", Some("key-123"));
        assert_eq!(header.algorithm(), "ML-DSA-65");
        assert_eq!(header.key_id(), Some("key-123"));
        assert_eq!(header.typ(), "JWT");
    }

    #[test]
    fn test_new_without_kid() {
        let header = JwtHeader::new("ML-DSA-65", None::<String>);
        assert_eq!(header.algorithm(), "ML-DSA-65");
        assert_eq!(header.key_id(), None);
        assert_eq!(header.typ(), "JWT");
    }

    #[test]
    fn test_serialize_with_kid() {
        let header = JwtHeader::new("ML-DSA-65", Some("key-123"));
        let json = serde_json::to_string(&header).unwrap();
        assert!(json.contains("\"kid\":\"key-123\""));
    }

    #[test]
    fn test_serialize_without_kid() {
        let header = JwtHeader::new("ML-DSA-65", None::<String>);
        let json = serde_json::to_string(&header).unwrap();
        assert!(!json.contains("kid"));
    }
}
