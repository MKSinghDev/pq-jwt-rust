use super::Verifier;

/// Builder for constructing a Verifier with optional claim validations
///
/// The verifier automatically validates `exp` (expiration) and `iss` (issuer) by default.
/// Additional optional validations can be configured via builder methods.
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
/// use pq_jwt::signer::sign;
/// use pq_jwt::verifier::Builder;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
/// let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let (jwt, _) = sign(MlDsaAlgo::Dsa65, "https://test.com", now + 3600, &private_key).unwrap();
///
/// let verifier = Builder::new()
///     .public_key(&public_key)
///     .issuer("https://test.com")  // Required: expected issuer
///     .build()
///     .unwrap();
///
/// let payload = verifier.verify(&jwt).unwrap();
/// assert!(payload.contains("https://test.com"));
/// ```
pub struct Builder {
    public_key: Option<String>,
    expected_issuer: Option<String>, // REQUIRED
    // Optional claim validations
    expected_audience: Option<String>,
    expected_subject: Option<String>,
    leeway: u64, // Time leeway in seconds for exp/nbf validation (default: 0)
}

impl Builder {
    /// Creates a new Verifier builder
    pub fn new() -> Self {
        Self {
            public_key: None,
            expected_issuer: None,
            expected_audience: None,
            expected_subject: None,
            leeway: 0,
        }
    }

    /// Sets the public key (REQUIRED)
    ///
    /// # Arguments
    /// * `public_key` - Hex-encoded public key
    pub fn public_key(mut self, public_key: impl Into<String>) -> Self {
        self.public_key = Some(public_key.into());
        self
    }

    /// Sets the expected issuer for validation (REQUIRED)
    ///
    /// The JWT's `iss` claim must match this value, otherwise verification will fail.
    /// This is a required field and `build()` will return an error if not set.
    ///
    /// # Arguments
    /// * `issuer` - Expected issuer value
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.expected_issuer = Some(issuer.into());
        self
    }

    /// Sets the expected audience for validation (optional)
    ///
    /// If set, the JWT's `aud` claim must match this value.
    ///
    /// # Arguments
    /// * `audience` - Expected audience value
    pub fn audience(mut self, audience: impl Into<String>) -> Self {
        self.expected_audience = Some(audience.into());
        self
    }

    /// Sets the expected subject for validation (optional)
    ///
    /// If set, the JWT's `sub` claim must match this value.
    ///
    /// # Arguments
    /// * `subject` - Expected subject value
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.expected_subject = Some(subject.into());
        self
    }

    /// Sets the time leeway for exp/nbf validation (optional)
    ///
    /// Adds a time buffer (in seconds) to account for clock skew between systems.
    /// Default is 0 seconds.
    ///
    /// # Arguments
    /// * `leeway` - Time leeway in seconds
    ///
    /// # Example
    /// ```
    /// use pq_jwt::verifier::Builder;
    /// use pq_jwt::{generate_keypair, MlDsaAlgo};
    ///
    /// let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
    ///
    /// let verifier = Builder::new()
    ///     .public_key(&public_key)
    ///     .issuer("https://myapp.com")
    ///     .leeway(60)  // Allow 60 seconds of clock skew
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn leeway(mut self, leeway: u64) -> Self {
        self.leeway = leeway;
        self
    }

    /// Builds the Verifier
    ///
    /// # Returns
    /// * `Ok(Verifier)` - Successfully constructed verifier
    /// * `Err(String)` - Missing required fields
    ///
    /// # Example
    /// ```
    /// use pq_jwt::{generate_keypair, MlDsaAlgo};
    /// use pq_jwt::verifier::Builder;
    ///
    /// let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
    ///
    /// let verifier = Builder::new()
    ///     .public_key(&public_key)
    ///     .issuer("https://myapp.com")
    ///     .audience("https://api.myapp.com")
    ///     .leeway(60)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Verifier, String> {
        let public_key = self.public_key.ok_or("Public key is required")?;
        let expected_issuer = self.expected_issuer.ok_or("Issuer is required")?;
        Ok(Verifier::new(
            public_key,
            Some(expected_issuer),
            self.expected_audience,
            self.expected_subject,
            self.leeway,
        ))
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
    use crate::MlDsaAlgo;
    use crate::keygen::generate_keypair;
    use crate::signer::sign;

    #[test]
    fn test_builder_basic() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let verifier = Builder::new()
            .public_key(&public_key)
            .issuer("https://test.com")
            .build()
            .unwrap();

        assert_eq!(verifier.public_key(), &public_key);
    }

    #[test]
    fn test_builder_missing_public_key() {
        let result = Builder::new().issuer("https://test.com").build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key is required");
    }

    #[test]
    fn test_builder_missing_issuer() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let result = Builder::new().public_key(&public_key).build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Issuer is required");
    }

    #[test]
    fn test_builder_and_verify() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa44).unwrap();
        let (jwt, _) = sign(
            MlDsaAlgo::Dsa44,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        let verifier = Builder::new()
            .public_key(&public_key)
            .issuer("https://test.com")
            .build()
            .unwrap();

        let result = verifier.verify(&jwt);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_default() {
        let builder = Builder::default();
        assert_eq!(builder.public_key, None);
        assert_eq!(builder.expected_issuer, None);
    }
}
