mod builder;
mod verify;

pub use builder::Builder;
pub use verify::verify;

use crate::algorithm::MlDsaAlgo;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ml_dsa::{
    EncodedVerifyingKey, KeyGen, MlDsa44, MlDsa65, MlDsa87, Signature, VerifyingKey,
    signature::Verifier as MlDsaVerifier,
};

/// A stateful verifier that holds configuration for verifying JWTs
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
#[derive(Debug)]
pub struct Verifier {
    public_key: String,
}

impl Verifier {
    /// Creates a new Verifier with the specified public key
    ///
    /// # Arguments
    /// * `public_key` - Hex-encoded public key
    pub(crate) fn new(public_key: String) -> Self {
        Self { public_key }
    }

    /// Verifies a JWT and returns the decoded payload
    ///
    /// # Arguments
    /// * `jwt` - The JWT string to verify
    ///
    /// # Returns
    /// * `Ok(payload)` - The decoded payload string if verification succeeds
    /// * `Err(String)` - Error message if verification fails
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
    pub fn verify(&self, jwt: &str) -> Result<String, String> {
        // Split JWT into parts
        let parts: Vec<&str> = jwt.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid JWT format: expected 3 parts".to_string());
        }

        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let signature_b64 = parts[2];

        // Decode and parse header
        let header_json = URL_SAFE_NO_PAD
            .decode(header_b64)
            .map_err(|e| format!("Failed to decode header: {}", e))?;
        let header: crate::header::JwtHeader = serde_json::from_slice(&header_json)
            .map_err(|e| format!("Failed to parse header: {}", e))?;

        // Get algorithm
        let algo = MlDsaAlgo::from_str(header.algorithm())?;

        // Verify signature based on algorithm
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let signature_bytes = URL_SAFE_NO_PAD
            .decode(signature_b64)
            .map_err(|e| format!("Failed to decode signature: {}", e))?;

        match algo {
            MlDsaAlgo::Dsa44 => self.verify_impl::<MlDsa44>(&signing_input, &signature_bytes)?,
            MlDsaAlgo::Dsa65 => self.verify_impl::<MlDsa65>(&signing_input, &signature_bytes)?,
            MlDsaAlgo::Dsa87 => self.verify_impl::<MlDsa87>(&signing_input, &signature_bytes)?,
        }

        // Decode payload
        let payload_bytes = URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|e| format!("Failed to decode payload: {}", e))?;
        let payload = String::from_utf8(payload_bytes)
            .map_err(|e| format!("Invalid UTF-8 in payload: {}", e))?;

        Ok(payload)
    }

    fn verify_impl<P>(&self, signing_input: &str, signature_bytes: &[u8]) -> Result<(), String>
    where
        P: KeyGen,
    {
        // Decode public key
        let key_bytes =
            hex::decode(&self.public_key).map_err(|e| format!("Invalid hex public key: {}", e))?;

        // Convert to EncodedVerifyingKey
        let encoded_key = EncodedVerifyingKey::<P>::try_from(key_bytes.as_slice())
            .map_err(|e| format!("Invalid verifying key length: {:?}", e))?;

        // Decode to VerifyingKey
        let verifying_key = VerifyingKey::<P>::decode(&encoded_key);

        // Decode signature
        let signature = Signature::<P>::try_from(signature_bytes)
            .map_err(|e| format!("Invalid signature: {:?}", e))?;

        // Verify
        verifying_key
            .verify(signing_input.as_bytes(), &signature)
            .map_err(|e| format!("Signature verification failed: {:?}", e))?;

        Ok(())
    }

    /// Returns the public key being used by this verifier
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;
    use crate::signer::sign;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_verifier_basic() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        let verifier = Verifier::new(public_key);
        let result = verifier.verify(&jwt);

        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_verifier_reuse() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt1, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test1.com",
            now + 3600,
            &private_key,
        )
        .unwrap();
        let (jwt2, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test2.com",
            now + 7200,
            &private_key,
        )
        .unwrap();

        let verifier = Verifier::new(public_key);

        let result1 = verifier.verify(&jwt1).unwrap();
        let result2 = verifier.verify(&jwt2).unwrap();

        assert!(result1.contains("https://test1.com"));
        assert!(result2.contains("https://test2.com"));
    }

    #[test]
    fn test_verifier_with_wrong_key() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key1, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (_, public_key2) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key1,
        )
        .unwrap();

        let verifier = Verifier::new(public_key2);
        let result = verifier.verify(&jwt);

        assert!(result.is_err());
    }

    #[test]
    fn test_verifier_invalid_jwt_format() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let verifier = Verifier::new(public_key);

        let result = verifier.verify("invalid.jwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_verifier_json_payload() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        let verifier = Verifier::new(public_key);
        let result = verifier.verify(&jwt);

        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_verifier_getter() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let verifier = Verifier::new(public_key.clone());

        assert_eq!(verifier.public_key(), &public_key);
    }
}
