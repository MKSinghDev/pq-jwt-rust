use crate::algorithm::MlDsaAlgo;
use crate::header::JwtHeader;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ml_dsa::{
    signature::Signer, EncodedSigningKey, KeyGen, MlDsa44, MlDsa65, MlDsa87, Signature, SigningKey,
};

/// Signs a payload and returns a JWT string along with the public key
///
/// # Arguments
/// * `algo` - The ML-DSA algorithm variant to use
/// * `payload` - The payload to sign (will be base64url encoded in JWT)
/// * `private_key_hex` - Hex-encoded private signing key
///
/// # Returns
/// * `Ok((jwt, public_key_hex))` - JWT string and hex-encoded public key
/// * `Err(String)` - Error message if signing fails
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, sign, MlDsaAlgo};
///
/// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let payload = r#"{"user":"alice","role":"admin"}"#;
/// let (jwt, public_key) = sign(MlDsaAlgo::Dsa65, payload, &private_key).unwrap();
/// ```
pub fn sign(
    algo: MlDsaAlgo,
    payload: &str,
    private_key_hex: &str,
) -> Result<(String, String), String> {
    match algo {
        MlDsaAlgo::Dsa44 => sign_impl::<MlDsa44>(algo, payload, private_key_hex),
        MlDsaAlgo::Dsa65 => sign_impl::<MlDsa65>(algo, payload, private_key_hex),
        MlDsaAlgo::Dsa87 => sign_impl::<MlDsa87>(algo, payload, private_key_hex),
    }
}

fn sign_impl<P>(
    algo: MlDsaAlgo,
    payload: &str,
    private_key_hex: &str,
) -> Result<(String, String), String>
where
    P: KeyGen,
{
    // Decode private key from hex
    let key_bytes =
        hex::decode(private_key_hex).map_err(|e| format!("Invalid hex private key: {}", e))?;

    // Convert to EncodedSigningKey
    let encoded_key = EncodedSigningKey::<P>::try_from(key_bytes.as_slice())
        .map_err(|e| format!("Invalid signing key length: {:?}", e))?;

    // Decode to SigningKey
    let signing_key = SigningKey::<P>::decode(&encoded_key);

    // Create JWT header
    let header = JwtHeader::new(algo.as_str().to_string());
    let header_json =
        serde_json::to_string(&header).map_err(|e| format!("Failed to serialize header: {}", e))?;
    let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());

    // Encode payload
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload.as_bytes());

    // Create signing input
    let signing_input = format!("{}.{}", header_b64, payload_b64);

    // Sign - returns Signature<P>
    let signature: Signature<P> = signing_key.sign(signing_input.as_bytes());

    // Encode signature to bytes
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;

    #[test]
    fn test_sign_basic() {
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let payload = "test payload";

        let result = sign(MlDsaAlgo::Dsa65, payload, &private_key);
        assert!(result.is_ok());

        let (jwt, returned_pub_key) = result.unwrap();
        assert_eq!(public_key, returned_pub_key);
        assert!(jwt.contains('.'));
        assert_eq!(jwt.split('.').count(), 3); // header.payload.signature
    }

    #[test]
    fn test_sign_json_payload() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let payload = r#"{"sub":"1234567890","name":"John Doe"}"#;

        let result = sign(MlDsaAlgo::Dsa65, payload, &private_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sign_with_invalid_key() {
        let result = sign(MlDsaAlgo::Dsa65, "test", "invalid_hex");
        assert!(result.is_err());
    }
}
