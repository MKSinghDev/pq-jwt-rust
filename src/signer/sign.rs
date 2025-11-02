use super::Builder;
use crate::algorithm::MlDsaAlgo;

/// Signs a payload and returns a JWT string along with the public key
///
/// This is a convenience function that wraps the Builder/Signer internally.
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
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
/// use pq_jwt::signer::sign;
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
    let signer = Builder::new()
        .algorithm(algo)
        .private_key(private_key_hex)
        .build()?;

    signer.sign(payload)
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

    #[test]
    fn test_sign_all_algorithms() {
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let (private_key, _) = generate_keypair(algo).unwrap();
            let result = sign(algo, "test", &private_key);
            assert!(result.is_ok());
        }
    }
}
