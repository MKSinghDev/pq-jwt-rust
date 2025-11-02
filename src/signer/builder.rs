use super::Signer;
use crate::algorithm::MlDsaAlgo;
use crate::header::JwtHeader;

/// Builder for constructing a Signer with optional configuration
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
/// use pq_jwt::signer::Builder;
///
/// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
///
/// let signer = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .private_key(&private_key)
///     .kid("key-2024-01")
///     .build()
///     .unwrap();
///
/// let (jwt, _) = signer.sign(r#"{"user":"alice"}"#).unwrap();
/// ```
pub struct Builder {
    algo: Option<MlDsaAlgo>,
    private_key: Option<String>,
    kid: Option<String>,
    header: Option<JwtHeader>,
}

impl Builder {
    /// Creates a new Signer builder
    pub fn new() -> Self {
        Self {
            algo: None,
            private_key: None,
            kid: None,
            header: None,
        }
    }

    /// Sets the algorithm
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    pub fn algorithm(mut self, algo: MlDsaAlgo) -> Self {
        self.algo = Some(algo);
        self
    }

    /// Sets the private key
    ///
    /// # Arguments
    /// * `private_key` - Hex-encoded private key
    pub fn private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }

    /// Sets the key ID (kid) for key rotation
    ///
    /// # Arguments
    /// * `kid` - Key identifier
    pub fn kid(mut self, kid: impl Into<String>) -> Self {
        self.kid = Some(kid.into());
        self
    }

    /// Sets a custom header (overrides algorithm and kid settings)
    ///
    /// # Arguments
    /// * `header` - Pre-configured JWT header
    ///
    /// # Example
    /// ```
    /// use pq_jwt::{generate_keypair, MlDsaAlgo};
    /// use pq_jwt::signer::Builder;
    /// use pq_jwt::header;
    ///
    /// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
    ///
    /// let custom_header = header::Builder::new()
    ///     .algorithm("ML-DSA-65")
    ///     .kid("custom-key")
    ///     .typ("CustomJWT")
    ///     .build()
    ///     .unwrap();
    ///
    /// let signer = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .private_key(&private_key)
    ///     .header(custom_header)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn header(mut self, header: JwtHeader) -> Self {
        self.header = Some(header);
        self
    }

    /// Builds the Signer
    ///
    /// # Returns
    /// * `Ok(Signer)` - Successfully constructed signer
    /// * `Err(String)` - Missing required fields
    ///
    /// # Example
    /// ```
    /// use pq_jwt::{generate_keypair, MlDsaAlgo};
    /// use pq_jwt::signer::Builder;
    ///
    /// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
    ///
    /// let signer = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .private_key(&private_key)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Signer, String> {
        let algo = self.algo.ok_or("Algorithm is required")?;
        let private_key = self.private_key.ok_or("Private key is required")?;

        // Use custom header if provided, otherwise build one
        let header = if let Some(h) = self.header {
            h
        } else {
            JwtHeader::new(algo.as_str(), self.kid)
        };

        Ok(Signer::new(algo, private_key, header))
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

    #[test]
    fn test_builder_basic() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .build()
            .unwrap();

        assert_eq!(signer.algorithm(), MlDsaAlgo::Dsa65);
        assert_eq!(signer.key_id(), None);
    }

    #[test]
    fn test_builder_with_kid() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .kid("key-123")
            .build()
            .unwrap();

        assert_eq!(signer.key_id(), Some("key-123"));
    }

    #[test]
    fn test_builder_with_custom_header() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa87).unwrap();

        let custom_header = JwtHeader::new("ML-DSA-87", Some("custom-key"));

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa87)
            .private_key(&private_key)
            .header(custom_header)
            .build()
            .unwrap();

        assert_eq!(signer.key_id(), Some("custom-key"));
    }

    #[test]
    fn test_builder_missing_algorithm() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let result = Builder::new().private_key(&private_key).build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Algorithm is required");
    }

    #[test]
    fn test_builder_missing_private_key() {
        let result = Builder::new().algorithm(MlDsaAlgo::Dsa65).build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Private key is required");
    }

    #[test]
    fn test_builder_and_sign() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa44).unwrap();

        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa44)
            .private_key(&private_key)
            .kid("test-key")
            .build()
            .unwrap();

        let result = signer.sign("test payload");
        assert!(result.is_ok());
    }
}
