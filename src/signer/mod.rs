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
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Generates a kid (key ID) from a public key using SHA-256 thumbprint
fn generate_kid_from_pubkey(public_key_hex: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(public_key_hex.as_bytes());
    let hash = hasher.finalize();
    // Take first 16 bytes (32 hex chars) for readability
    hex::encode(&hash[..16])
}

/// JWT Claims structure containing standard and custom claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer (REQUIRED)
    pub iss: String,
    /// Expiration time (REQUIRED) - Unix timestamp
    pub exp: u64,
    /// Issued at (optional, defaults to signing time) - Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<u64>,
    /// Subject (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// Audience (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// Not before (optional) - Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
    /// JWT ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    /// Additional custom claims
    #[serde(flatten)]
    pub custom: HashMap<String, JsonValue>,
}

impl Claims {
    /// Creates a new Claims with required fields
    pub fn new(iss: impl Into<String>, exp: u64) -> Self {
        Self {
            iss: iss.into(),
            exp,
            iat: None,
            sub: None,
            aud: None,
            nbf: None,
            jti: None,
            custom: HashMap::new(),
        }
    }

    /// Validates the claims
    pub fn validate(&self) -> Result<(), String> {
        // Validate exp > iat (if iat is present)
        if let Some(iat) = self.iat
            && self.exp <= iat
        {
            return Err(format!(
                "Expiration (exp={}) must be after issued at (iat={})",
                self.exp, iat
            ));
        }

        // Validate nbf < exp (if nbf is present)
        // Ensures the token can be used before it expires
        if let Some(nbf) = self.nbf
            && nbf >= self.exp
        {
            return Err(format!(
                "Not before (nbf={}) must be before expiration (exp={})",
                nbf, self.exp
            ));
        }

        Ok(())
    }

    /// Converts claims to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Failed to serialize claims: {}", e))
    }
}

/// A stateful signer that holds configuration for signing JWTs
///
/// The kid (Key ID) is automatically generated from the public key using SHA-256.
/// Claims are configured via the Builder and validated before signing.
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
///
/// let (jwt, pub_key) = signer.sign().unwrap();
/// ```
#[derive(Debug)]
pub struct Signer {
    algo: MlDsaAlgo,
    private_key: String,
    claims: Claims,
    auto_issue_iat: bool, // If true, auto-populate iat when None
}

impl Signer {
    /// Creates a new Signer with the specified configuration
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    /// * `private_key` - Hex-encoded private key
    /// * `claims` - JWT claims to sign
    /// * `auto_issue_iat` - If true, auto-populate iat when None (default: true)
    pub(crate) fn new(
        algo: MlDsaAlgo,
        private_key: String,
        claims: Claims,
        auto_issue_iat: bool,
    ) -> Self {
        Self {
            algo,
            private_key,
            claims,
            auto_issue_iat,
        }
    }

    /// Signs the configured claims and returns a JWT string with the public key
    ///
    /// If `iat` (issued at) is not set in claims and `auto_issue_iat` is true,
    /// it defaults to the current signing time.
    ///
    /// # Returns
    /// * `Ok((jwt, public_key_hex))` - JWT string and hex-encoded public key
    /// * `Err(String)` - Error message if signing fails
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
    ///
    /// let (jwt, pub_key) = signer.sign().unwrap();
    /// ```
    pub fn sign(&self) -> Result<(String, String), String> {
        // Clone claims and set iat to now if not set and auto_issue_iat is enabled
        let mut claims = self.claims.clone();
        if claims.iat.is_none() && self.auto_issue_iat {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| format!("Failed to get current time: {}", e))?
                .as_secs();
            claims.iat = Some(now);
        }

        // Validate claims
        claims.validate()?;

        // Convert claims to JSON
        let payload = claims.to_json()?;

        match self.algo {
            MlDsaAlgo::Dsa44 => self.sign_impl::<MlDsa44>(&payload),
            MlDsaAlgo::Dsa65 => self.sign_impl::<MlDsa65>(&payload),
            MlDsaAlgo::Dsa87 => self.sign_impl::<MlDsa87>(&payload),
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

        // Get public key
        let verifying_key = signing_key.verifying_key();
        let pub_key_encoded = verifying_key.encode();
        let pub_key_hex = hex::encode(&pub_key_encoded[..]);

        // Generate kid from public key
        let kid = generate_kid_from_pubkey(&pub_key_hex);

        // Create header with kid
        let header = JwtHeader::new(self.algo.as_str(), kid);
        let header_json = serde_json::to_string(&header)
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

        Ok((jwt, pub_key_hex))
    }

    /// Returns the algorithm being used by this signer
    pub fn algorithm(&self) -> MlDsaAlgo {
        self.algo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_signer_basic() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims::new("https://test.com", now + 3600);
        let signer = Signer::new(MlDsaAlgo::Dsa65, private_key, claims, true);

        let result = signer.sign();
        assert!(result.is_ok());
    }

    #[test]
    fn test_signer_reuse() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims1 = Claims::new("https://test.com", now + 3600);
        let signer1 = Signer::new(MlDsaAlgo::Dsa65, private_key.clone(), claims1, true);

        let claims2 = Claims::new("https://test.com", now + 7200);
        let signer2 = Signer::new(MlDsaAlgo::Dsa65, private_key, claims2, true);

        let (jwt1, _) = signer1.sign().unwrap();
        let (jwt2, _) = signer2.sign().unwrap();

        assert_ne!(jwt1, jwt2);
    }

    #[test]
    fn test_signer_getters() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa87).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims::new("https://test.com", now + 3600);
        let signer = Signer::new(MlDsaAlgo::Dsa87, private_key, claims, true);

        assert_eq!(signer.algorithm(), MlDsaAlgo::Dsa87);
    }

    #[test]
    fn test_signer_with_invalid_key() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let claims = Claims::new("https://test.com", now + 3600);
        let signer = Signer::new(MlDsaAlgo::Dsa65, "invalid_hex".to_string(), claims, true);

        let result = signer.sign();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid hex private key"));
    }

    #[test]
    fn test_claims_validation_exp_after_iat() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims::new("https://test.com", now + 3600);
        claims.iat = Some(now);

        // Should pass: exp > iat
        assert!(claims.validate().is_ok());
    }

    #[test]
    fn test_claims_validation_exp_before_iat_fails() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims::new("https://test.com", now);
        claims.iat = Some(now + 3600);

        // Should fail: exp <= iat
        let result = claims.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expiration"));
    }

    #[test]
    fn test_claims_validation_nbf_before_exp() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims::new("https://test.com", now + 3600);
        claims.iat = Some(now);
        claims.nbf = Some(now + 1800); // nbf is after iat but before exp

        // Should pass: nbf < exp (even if nbf > iat)
        assert!(claims.validate().is_ok());
    }

    #[test]
    fn test_claims_validation_nbf_after_exp_fails() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims::new("https://test.com", now + 3600);
        claims.iat = Some(now);
        claims.nbf = Some(now + 3600); // nbf equals exp

        // Should fail: nbf >= exp
        let result = claims.validate();
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Not before"));
        assert!(err_msg.contains("expiration"));
    }

    #[test]
    fn test_claims_with_custom_data() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims::new("https://test.com", now + 3600);
        claims.custom.insert("role".to_string(), json!("admin"));
        claims
            .custom
            .insert("permissions".to_string(), json!(["read", "write"]));

        let json = claims.to_json().unwrap();
        assert!(json.contains("role"));
        assert!(json.contains("admin"));
    }

    #[test]
    fn test_skip_issued_at() {
        use super::Builder;

        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Build signer with skip_issued_at
        let signer = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .private_key(&private_key)
            .issuer("https://test.com")
            .expiration(now + 3600)
            .skip_issued_at()
            .build()
            .unwrap();

        let (jwt, _) = signer.sign().unwrap();

        // Decode and check that iat is not present
        let parts: Vec<&str> = jwt.split('.').collect();
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .unwrap();
        let payload_str = String::from_utf8(payload).unwrap();

        assert!(!payload_str.contains("\"iat\""));
    }
}

