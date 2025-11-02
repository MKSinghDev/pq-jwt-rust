use serde::{Deserialize, Serialize};

/// JWT Header structure (internal use only)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct JwtHeader {
    pub alg: String,
    pub kid: String, // Always present - either manual or auto-generated
    pub typ: String,
}

impl JwtHeader {
    /// Creates a new JWT header
    pub fn new(alg: impl Into<String>, kid: impl Into<String>) -> Self {
        Self {
            alg: alg.into(),
            kid: kid.into(),
            typ: "JWT".to_string(),
        }
    }

    /// Gets the algorithm
    pub fn algorithm(&self) -> &str {
        &self.alg
    }
}
