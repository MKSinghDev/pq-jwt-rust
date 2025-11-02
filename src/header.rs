use serde::{Deserialize, Serialize};

/// JWT Header structure
#[derive(Serialize, Deserialize)]
pub(crate) struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

impl JwtHeader {
    pub fn new(alg: String) -> Self {
        Self {
            alg,
            typ: "JWT".to_string(),
        }
    }
}
