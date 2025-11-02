use super::Builder;

/// Verifies a JWT and returns the decoded payload
///
/// This is a convenience function that wraps the Builder/Verifier internally.
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
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
/// use pq_jwt::signer::sign;
/// use pq_jwt::verifier::verify;
///
/// let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let payload = r#"{"user":"alice"}"#;
/// let (jwt, _) = sign(MlDsaAlgo::Dsa65, payload, &private_key).unwrap();
/// let verified_payload = verify(&jwt, &public_key).unwrap();
/// assert_eq!(payload, verified_payload);
/// ```
pub fn verify(jwt: &str, public_key_hex: &str) -> Result<String, String> {
    let verifier = Builder::new().public_key(public_key_hex).build()?;
    verifier.verify(jwt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MlDsaAlgo;
    use crate::keygen::generate_keypair;
    use crate::signer::sign;

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

    #[test]
    fn test_verify_all_algorithms() {
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let (private_key, public_key) = generate_keypair(algo).unwrap();
            let (jwt, _) = sign(algo, "test", &private_key).unwrap();
            let result = verify(&jwt, &public_key);
            assert!(result.is_ok());
        }
    }
}
