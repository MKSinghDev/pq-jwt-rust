use super::{Claims, Signer};
use crate::algorithm::MlDsaAlgo;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Builder for constructing a Signer with JWT claims
///
/// The kid (Key ID) is automatically generated from the public key.
/// Claims (iss, exp, etc.) are configured via builder methods.
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
/// use pq_jwt::signer::Builder;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
///
/// let signer = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .private_key(&private_key)
///     .issuer("https://myapp.com")
///     .expiration(now + 3600)
///     .subject("user@example.com")
///     .build()
///     .unwrap();
///
/// let (jwt, _) = signer.sign().unwrap();
/// ```
pub struct Builder {
    algo: Option<MlDsaAlgo>,
    private_key: Option<String>,
    // Claims fields
    issuer: Option<String>,
    expiration: Option<u64>,
    subject: Option<String>,
    audience: Option<String>,
    issued_at: Option<u64>, // None = default to signing time, Some(ts) = use ts
    not_before: Option<u64>,
    jwt_id: Option<String>,
    custom: HashMap<String, JsonValue>,
}

impl Builder {
    /// Creates a new Signer builder
    pub fn new() -> Self {
        Self {
            algo: None,
            private_key: None,
            issuer: None,
            expiration: None,
            subject: None,
            audience: None,
            issued_at: None,
            not_before: None,
            jwt_id: None,
            custom: HashMap::new(),
        }
    }

    /// Sets the algorithm
    pub fn algorithm(mut self, algo: MlDsaAlgo) -> Self {
        self.algo = Some(algo);
        self
    }

    /// Sets the private key
    pub fn private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }

    /// Sets the issuer (REQUIRED)
    pub fn issuer(mut self, iss: impl Into<String>) -> Self {
        self.issuer = Some(iss.into());
        self
    }

    /// Sets the expiration time (REQUIRED) - Unix timestamp in seconds
    pub fn expiration(mut self, exp: u64) -> Self {
        self.expiration = Some(exp);
        self
    }

    /// Sets the subject (optional)
    pub fn subject(mut self, sub: impl Into<String>) -> Self {
        self.subject = Some(sub.into());
        self
    }

    /// Sets the audience (optional)
    pub fn audience(mut self, aud: impl Into<String>) -> Self {
        self.audience = Some(aud.into());
        self
    }

    /// Sets the issued at time (optional)
    ///
    /// # Arguments
    /// * `iat` - Unix timestamp in seconds for the issued at time
    ///
    /// # Behavior
    /// - Not calling this method: `iat` defaults to signing time (auto-populated by `Signer::sign()`)
    /// - Calling this method: Uses the provided timestamp
    pub fn issued_at(mut self, iat: u64) -> Self {
        self.issued_at = Some(iat);
        self
    }

    /// Sets the not before time (optional) - Unix timestamp in seconds
    pub fn not_before(mut self, nbf: u64) -> Self {
        self.not_before = Some(nbf);
        self
    }

    /// Sets the JWT ID (optional)
    pub fn jwt_id(mut self, jti: impl Into<String>) -> Self {
        self.jwt_id = Some(jti.into());
        self
    }

    /// Adds custom claims to the JWT payload
    ///
    /// If a custom claim key matches a standard claim (iss, exp, sub, etc.),
    /// the value set via the specific method takes precedence.
    ///
    /// # Example
    /// ```
    /// use pq_jwt::signer::Builder;
    /// use serde_json::json;
    ///
    /// let builder = Builder::new()
    ///     .custom_claims(json!({
    ///         "role": "admin",
    ///         "permissions": ["read", "write"],
    ///         "user_id": 12345
    ///     }));
    /// ```
    pub fn custom_claims(mut self, claims: JsonValue) -> Self {
        if let JsonValue::Object(map) = claims {
            for (key, value) in map {
                // Skip standard JWT claims - they're handled by specific methods
                if !matches!(
                    key.as_str(),
                    "iss" | "exp" | "iat" | "sub" | "aud" | "nbf" | "jti"
                ) {
                    self.custom.insert(key, value);
                }
            }
        }
        self
    }

    /// Builds the Signer
    ///
    /// # Returns
    /// * `Ok(Signer)` - Successfully constructed signer
    /// * `Err(String)` - Missing required fields (algo, private_key, issuer, expiration)
    ///
    /// # Example
    /// ```
    /// use pq_jwt::{generate_keypair, MlDsaAlgo};
    /// use pq_jwt::signer::Builder;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
    /// let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    ///
    /// let signer = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .private_key(&private_key)
    ///     .issuer("https://myapp.com")
    ///     .expiration(now + 3600)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Signer, String> {
        let algo = self.algo.ok_or("Algorithm is required")?;
        let private_key = self.private_key.ok_or("Private key is required")?;
        let iss = self.issuer.ok_or("Issuer (iss) is required")?;
        let exp = self.expiration.ok_or("Expiration (exp) is required")?;

        // Build claims
        let mut claims = Claims::new(iss, exp);
        claims.sub = self.subject;
        claims.aud = self.audience;
        claims.nbf = self.not_before;
        claims.jti = self.jwt_id;
        claims.custom = self.custom;

        // Handle iat: None = default to signing time (auto-populated), Some(ts) = use ts
        claims.iat = self.issued_at;

        Ok(Signer::new(algo, private_key, claims))
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_builder_basic() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://test.com")
            .expiration(now + 3600)
            .build()
            .unwrap();

        assert_eq!(signer.algorithm(), MlDsaAlgo::Dsa65);
    }

    #[test]
    fn test_builder_missing_algorithm() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = Builder::new()
            .private_key(&private_key)
            .issuer("https://test.com")
            .expiration(now + 3600)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Algorithm is required");
    }

    #[test]
    fn test_builder_missing_private_key() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .issuer("https://test.com")
            .expiration(now + 3600)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Private key is required");
    }

    #[test]
    fn test_builder_missing_issuer() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .expiration(now + 3600)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Issuer (iss) is required");
    }

    #[test]
    fn test_builder_missing_expiration() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let result = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://test.com")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expiration (exp) is required");
    }

    #[test]
    fn test_builder_and_sign() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa44).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa44)
            .private_key(&private_key)
            .issuer("https://test.com")
            .expiration(now + 3600)
            .build()
            .unwrap();

        let result = signer.sign();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_all_claims() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://myapp.com")
            .expiration(now + 3600)
            .subject("user@example.com")
            .audience("https://api.example.com")
            .not_before(now)
            .jwt_id("unique-jwt-id")
            .build()
            .unwrap();

        let result = signer.sign();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_custom_claims() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://myapp.com")
            .expiration(now + 3600)
            .custom_claims(json!({
                "role": "admin",
                "permissions": ["read", "write"]
            }))
            .build()
            .unwrap();

        let result = signer.sign();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_custom_claims_ignore_standard() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://myapp.com")
            .expiration(now + 3600)
            .custom_claims(json!({
                "iss": "https://should-be-ignored.com",
                "role": "admin"
            }))
            .build()
            .unwrap();

        let result = signer.sign();
        assert!(result.is_ok());
    }
}
