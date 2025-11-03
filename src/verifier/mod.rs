mod builder;
mod verify;

pub use builder::Builder;
pub use verify::verify;

use crate::algorithm::MlDsaAlgo;
use crate::signer::Claims;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ml_dsa::{
    EncodedVerifyingKey, KeyGen, MlDsa44, MlDsa65, MlDsa87, Signature, VerifyingKey,
    signature::Verifier as MlDsaVerifier,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// A stateful verifier that holds configuration for verifying JWTs
///
/// Automatically validates (always performed):
/// - Signature validity
/// - Token expiration (`exp` claim must be in the future)
/// - Issuer presence (`iss` claim must be non-empty)
///
/// Optional validations (configured via Builder):
/// - Issuer matching (`iss` claim matches expected value, if configured)
/// - Expected audience value
/// - Expected subject value
/// - Not before time (`nbf` claim if present)
/// - Time leeway for clock skew
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
///     .issuer("https://test.com")  // Validate issuer matches
///     .build()
///     .unwrap();
///
/// let payload = verifier.verify(&jwt).unwrap();
/// assert!(payload.contains("https://test.com"));
/// ```
#[derive(Debug)]
pub struct Verifier {
    public_key: String,
    expected_issuer: Option<String>,
    expected_audience: Option<String>,
    expected_subject: Option<String>,
    leeway: u64,
}

impl Verifier {
    /// Creates a new Verifier with the specified configuration
    ///
    /// # Arguments
    /// * `public_key` - Hex-encoded public key
    /// * `expected_issuer` - Optional expected issuer for validation
    /// * `expected_audience` - Optional expected audience for validation
    /// * `expected_subject` - Optional expected subject for validation
    /// * `leeway` - Time leeway in seconds for exp/nbf validation
    pub(crate) fn new(
        public_key: String,
        expected_issuer: Option<String>,
        expected_audience: Option<String>,
        expected_subject: Option<String>,
        leeway: u64,
    ) -> Self {
        Self {
            public_key,
            expected_issuer,
            expected_audience,
            expected_subject,
            leeway,
        }
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
    ///     .issuer("https://test.com")
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

        // Parse and validate claims
        let claims: Claims =
            serde_json::from_str(&payload).map_err(|e| format!("Failed to parse claims: {}", e))?;

        // Validate claims
        self.validate_claims(&claims)?;

        Ok(payload)
    }

    /// Validates JWT claims according to configured rules
    fn validate_claims(&self, claims: &Claims) -> Result<(), String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to get current time: {}", e))?
            .as_secs();

        // REQUIRED: Validate expiration (exp must exist and be in the future)
        if claims.exp <= now.saturating_sub(self.leeway) {
            return Err(format!(
                "Token has expired (exp={}, now={})",
                claims.exp, now
            ));
        }

        // REQUIRED: Validate issuer exists
        if claims.iss.is_empty() {
            return Err("Issuer (iss) claim is missing or empty".to_string());
        }

        // OPTIONAL: Validate expected issuer if configured
        if let Some(ref expected_iss) = self.expected_issuer
            && &claims.iss != expected_iss
        {
            return Err(format!(
                "Invalid issuer: expected '{}', got '{}'",
                expected_iss, claims.iss
            ));
        }

        // OPTIONAL: Validate expected audience if configured
        if let Some(ref expected_aud) = self.expected_audience {
            match &claims.aud {
                Some(aud) if aud == expected_aud => {}
                Some(aud) => {
                    return Err(format!(
                        "Invalid audience: expected '{}', got '{}'",
                        expected_aud, aud
                    ));
                }
                None => {
                    return Err(format!(
                        "Audience (aud) claim is missing, expected '{}'",
                        expected_aud
                    ));
                }
            }
        }

        // OPTIONAL: Validate expected subject if configured
        if let Some(ref expected_sub) = self.expected_subject {
            match &claims.sub {
                Some(sub) if sub == expected_sub => {}
                Some(sub) => {
                    return Err(format!(
                        "Invalid subject: expected '{}', got '{}'",
                        expected_sub, sub
                    ));
                }
                None => {
                    return Err(format!(
                        "Subject (sub) claim is missing, expected '{}'",
                        expected_sub
                    ));
                }
            }
        }

        // OPTIONAL: Validate not before (nbf) if present
        if let Some(nbf) = claims.nbf
            && nbf > now.saturating_add(self.leeway)
        {
            return Err(format!("Token not yet valid (nbf={}, now={})", nbf, now));
        }

        Ok(())
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

        let verifier = Verifier::new(
            public_key,
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );
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

        // Both JWTs must have the same issuer since verifier validates issuer
        let (jwt1, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();
        let (jwt2, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 7200,
            &private_key,
        )
        .unwrap();

        let verifier = Verifier::new(
            public_key,
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );

        // Verify both JWTs succeed (testing verifier reuse)
        let result1 = verifier.verify(&jwt1).unwrap();
        let result2 = verifier.verify(&jwt2).unwrap();

        assert!(result1.contains("https://test.com"));
        assert!(result2.contains("https://test.com"));
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

        let verifier = Verifier::new(
            public_key2,
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );
        let result = verifier.verify(&jwt);

        assert!(result.is_err());
    }

    #[test]
    fn test_verifier_invalid_jwt_format() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let verifier = Verifier::new(
            public_key,
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );

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

        let verifier = Verifier::new(
            public_key,
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );
        let result = verifier.verify(&jwt);

        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_verifier_getter() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let verifier = Verifier::new(
            public_key.clone(),
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );

        assert_eq!(verifier.public_key(), &public_key);
    }

    #[test]
    fn test_verifier_with_issuer_validation() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://myapp.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        // Should pass with correct issuer
        let verifier = Verifier::new(
            public_key.clone(),
            Some("https://myapp.com".to_string()),
            None,
            None,
            0,
        );
        assert!(verifier.verify(&jwt).is_ok());

        // Should fail with wrong issuer
        let verifier = Verifier::new(
            public_key,
            Some("https://wrong.com".to_string()),
            None,
            None,
            0,
        );
        let result = verifier.verify(&jwt);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid issuer"));
    }

    #[test]
    fn test_verifier_expired_token() {
        use crate::signer::Builder as SignerBuilder;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        // Create expired token (exp in the past, iat also in past to make it valid for signing)
        let old_time = now - 7200; // 2 hours ago
        let signer = SignerBuilder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://test.com")
            .expiration(old_time + 3600) // Expired 1 hour ago
            .issued_at(old_time)
            .build()
            .unwrap();

        let (jwt, _) = signer.sign().unwrap();

        let verifier = Verifier::new(
            public_key,
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );
        let result = verifier.verify(&jwt);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token has expired"));
    }

    #[test]
    fn test_verifier_with_leeway() {
        use crate::signer::Builder as SignerBuilder;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        // Create token that expired 30 seconds ago
        let old_time = now - 60; // 60 seconds ago
        let signer = SignerBuilder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://test.com")
            .expiration(now - 30) // Expired 30 seconds ago
            .issued_at(old_time)
            .build()
            .unwrap();

        let (jwt, _) = signer.sign().unwrap();

        // Should fail without leeway
        let verifier = Verifier::new(
            public_key.clone(),
            Some("https://test.com".to_string()),
            None,
            None,
            0,
        );
        assert!(verifier.verify(&jwt).is_err());

        // Should pass with 60 seconds leeway
        let verifier = Verifier::new(
            public_key,
            Some("https://test.com".to_string()),
            None,
            None,
            60,
        );
        assert!(verifier.verify(&jwt).is_ok());
    }

    #[test]
    fn test_verifier_audience_required_but_missing() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        // Create JWT WITHOUT audience claim using simple sign function
        let (jwt, _) = crate::signer::sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        // Verifier requires audience - should FAIL
        let verifier = Verifier::new(
            public_key,
            None,
            Some("https://api.myapp.com".to_string()),
            None,
            0,
        );
        let result = verifier.verify(&jwt);
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("Audience (aud) claim is missing"),
            "Expected missing audience error, got: {}",
            err_msg
        );
    }
}
