use super::Signer;
use crate::algorithm::MlDsaAlgo;

/// Builder for constructing a Signer
///
/// The kid (Key ID) is automatically generated from the public key.
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
///
/// let (jwt, _) = signer.sign(r#"{"user":"alice"}"#).unwrap();
/// ```
pub struct Builder {
    algo: Option<MlDsaAlgo>,
    private_key: Option<String>,
}

impl Builder {
    /// Creates a new Signer builder
    pub fn new() -> Self {
        Self {
            algo: None,
            private_key: None,
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

        Ok(Signer::new(algo, private_key))
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
            .build()
            .unwrap();

        let result = signer.sign("test payload");
        assert!(result.is_ok());
    }
}
