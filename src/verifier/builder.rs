use super::Verifier;

/// Builder for constructing a Verifier
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
///     .build()
///     .unwrap();
///
/// let payload = verifier.verify(&jwt).unwrap();
/// assert!(payload.contains("https://test.com"));
/// ```
pub struct Builder {
    public_key: Option<String>,
}

impl Builder {
    /// Creates a new Verifier builder
    pub fn new() -> Self {
        Self { public_key: None }
    }

    /// Sets the public key
    ///
    /// # Arguments
    /// * `public_key` - Hex-encoded public key
    pub fn public_key(mut self, public_key: impl Into<String>) -> Self {
        self.public_key = Some(public_key.into());
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
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Verifier, String> {
        let public_key = self.public_key.ok_or("Public key is required")?;
        Ok(Verifier::new(public_key))
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

        let verifier = Builder::new().public_key(&public_key).build().unwrap();

        assert_eq!(verifier.public_key(), &public_key);
    }

    #[test]
    fn test_builder_missing_public_key() {
        let result = Builder::new().build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key is required");
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

        let verifier = Builder::new().public_key(&public_key).build().unwrap();

        let result = verifier.verify(&jwt);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_default() {
        let builder = Builder::default();
        assert_eq!(builder.public_key, None);
    }
}
