mod builder;
mod sign;

pub use builder::Builder;
pub use sign::sign;

use crate::algorithm::MlDsaAlgo;
use crate::header::JwtHeader;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ml_dsa::{
    EncodedSigningKey, KeyGen, MlDsa44, MlDsa65, MlDsa87, Signature, SigningKey,
    signature::Signer as MlDsaSigner,
};

/// A stateful signer that holds configuration for signing JWTs
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
/// let payload = r#"{"user":"alice"}"#;
/// let (jwt, pub_key) = signer.sign(payload).unwrap();
/// ```
#[derive(Debug)]
pub struct Signer {
    algo: MlDsaAlgo,
    private_key: String,
    header: JwtHeader,
}

impl Signer {
    /// Creates a new Signer with the specified configuration
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    /// * `private_key` - Hex-encoded private key
    /// * `header` - Pre-configured JWT header
    pub(crate) fn new(algo: MlDsaAlgo, private_key: String, header: JwtHeader) -> Self {
        Self {
            algo,
            private_key,
            header,
        }
    }

    /// Signs a payload and returns a JWT string with the public key
    ///
    /// # Arguments
    /// * `payload` - The payload to sign (will be base64url encoded)
    ///
    /// # Returns
    /// * `Ok((jwt, public_key_hex))` - JWT string and hex-encoded public key
    /// * `Err(String)` - Error message if signing fails
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
    /// let (jwt1, _) = signer.sign(r#"{"user":"alice"}"#).unwrap();
    /// let (jwt2, _) = signer.sign(r#"{"user":"bob"}"#).unwrap();
    /// ```
    pub fn sign(&self, payload: &str) -> Result<(String, String), String> {
        match self.algo {
            MlDsaAlgo::Dsa44 => self.sign_impl::<MlDsa44>(payload),
            MlDsaAlgo::Dsa65 => self.sign_impl::<MlDsa65>(payload),
            MlDsaAlgo::Dsa87 => self.sign_impl::<MlDsa87>(payload),
        }
    }

    fn sign_impl<P>(&self, payload: &str) -> Result<(String, String), String>
    where
        P: KeyGen,
    {
        // Decode private key from hex
        let key_bytes = hex::decode(&self.private_key)
            .map_err(|e| format!("Invalid hex private key: {}", e))?;

        // Convert to EncodedSigningKey
        let encoded_key = EncodedSigningKey::<P>::try_from(key_bytes.as_slice())
            .map_err(|e| format!("Invalid signing key length: {:?}", e))?;

        // Decode to SigningKey
        let signing_key = SigningKey::<P>::decode(&encoded_key);

        // Serialize header
        let header_json = serde_json::to_string(&self.header)
            .map_err(|e| format!("Failed to serialize header: {}", e))?;
        let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());

        // Encode payload
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload.as_bytes());

        // Create signing input
        let signing_input = format!("{}.{}", header_b64, payload_b64);

        // Sign
        let signature: Signature<P> = signing_key.sign(signing_input.as_bytes());

        // Encode signature
        let sig_encoded = signature.encode();
        let signature_b64 = URL_SAFE_NO_PAD.encode(&sig_encoded[..]);

        // Create JWT
        let jwt = format!("{}.{}", signing_input, signature_b64);

        // Get public key
        let verifying_key = signing_key.verifying_key();
        let pub_key_encoded = verifying_key.encode();
        let pub_key_hex = hex::encode(&pub_key_encoded[..]);

        Ok((jwt, pub_key_hex))
    }

    /// Returns the algorithm being used by this signer
    pub fn algorithm(&self) -> MlDsaAlgo {
        self.algo
    }

    /// Returns the key ID if set
    pub fn key_id(&self) -> Option<&str> {
        self.header.key_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;

    #[test]
    fn test_signer_basic() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let header = JwtHeader::new("ML-DSA-65", None::<String>);

        let signer = Signer::new(MlDsaAlgo::Dsa65, private_key, header);

        let result = signer.sign("test payload");
        assert!(result.is_ok());
    }

    #[test]
    fn test_signer_reuse() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let header = JwtHeader::new("ML-DSA-65", Some("key-123"));

        let signer = Signer::new(MlDsaAlgo::Dsa65, private_key, header);

        let (jwt1, _) = signer.sign("payload1").unwrap();
        let (jwt2, _) = signer.sign("payload2").unwrap();

        assert_ne!(jwt1, jwt2);
        assert_eq!(signer.key_id(), Some("key-123"));
    }

    #[test]
    fn test_signer_getters() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa87).unwrap();
        let header = JwtHeader::new("ML-DSA-87", Some("prod-key"));

        let signer = Signer::new(MlDsaAlgo::Dsa87, private_key, header);

        assert_eq!(signer.algorithm(), MlDsaAlgo::Dsa87);
        assert_eq!(signer.key_id(), Some("prod-key"));
    }

    #[test]
    fn test_signer_with_invalid_key() {
        let header = JwtHeader::new("ML-DSA-65", None::<String>);
        let signer = Signer::new(MlDsaAlgo::Dsa65, "invalid_hex".to_string(), header);

        let result = signer.sign("test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid hex private key"));
    }
}
