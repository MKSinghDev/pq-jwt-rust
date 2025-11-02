use crate::algorithm::MlDsaAlgo;
use crate::header::JwtHeader;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ml_dsa::{
    EncodedVerifyingKey, KeyGen, MlDsa44, MlDsa65, MlDsa87, Signature, VerifyingKey,
    signature::Verifier,
};

/// Verifies a JWT and returns the decoded payload
///
/// # Arguments
/// * `jwt` - The JWT string to verify
/// * `public_key_hex` - Hex-encoded public verifying key
///
/// # Returns
/// * `Ok(payload)` - The decoded payload string if verification succeeds
/// * `Err(String)` - Error message if verification fails
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, sign, verify, MlDsaAlgo};
///
/// let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let payload = r#"{"user":"alice"}"#;
/// let (jwt, _) = sign(MlDsaAlgo::Dsa65, payload, &private_key).unwrap();
/// let verified_payload = verify(&jwt, &public_key).unwrap();
/// assert_eq!(payload, verified_payload);
/// ```
pub fn verify(jwt: &str, public_key_hex: &str) -> Result<String, String> {
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
    let header: JwtHeader = serde_json::from_slice(&header_json)
        .map_err(|e| format!("Failed to parse header: {}", e))?;

    // Get algorithm
    let algo = MlDsaAlgo::from_str(&header.alg)?;

    // Verify signature based on algorithm
    let signing_input = format!("{}.{}", header_b64, payload_b64);
    let signature_bytes = URL_SAFE_NO_PAD
        .decode(signature_b64)
        .map_err(|e| format!("Failed to decode signature: {}", e))?;

    match algo {
        MlDsaAlgo::Dsa44 => {
            verify_impl::<MlDsa44>(&signing_input, &signature_bytes, public_key_hex)?
        }
        MlDsaAlgo::Dsa65 => {
            verify_impl::<MlDsa65>(&signing_input, &signature_bytes, public_key_hex)?
        }
        MlDsaAlgo::Dsa87 => {
            verify_impl::<MlDsa87>(&signing_input, &signature_bytes, public_key_hex)?
        }
    }

    // Decode payload
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|e| format!("Failed to decode payload: {}", e))?;
    let payload =
        String::from_utf8(payload_bytes).map_err(|e| format!("Invalid UTF-8 in payload: {}", e))?;

    Ok(payload)
}

fn verify_impl<P>(
    signing_input: &str,
    signature_bytes: &[u8],
    public_key_hex: &str,
) -> Result<(), String>
where
    P: KeyGen,
{
    // Decode public key
    let key_bytes =
        hex::decode(public_key_hex).map_err(|e| format!("Invalid hex public key: {}", e))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;
    use crate::sign::sign;

    #[test]
    fn test_verify_valid_jwt() {
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let payload = "test payload";
        let (jwt, _) = sign(MlDsaAlgo::Dsa65, payload, &private_key).unwrap();

        let result = verify(&jwt, &public_key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), payload);
    }

    #[test]
    fn test_verify_with_wrong_key() {
        let (private_key1, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (_, public_key2) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let (jwt, _) = sign(MlDsaAlgo::Dsa65, "test", &private_key1).unwrap();
        let result = verify(&jwt, &public_key2);

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_invalid_jwt_format() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let result = verify("invalid.jwt", &public_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_json_payload() {
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let payload = r#"{"sub":"123","name":"Alice"}"#;
        let (jwt, _) = sign(MlDsaAlgo::Dsa65, payload, &private_key).unwrap();

        let result = verify(&jwt, &public_key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), payload);
    }
}
