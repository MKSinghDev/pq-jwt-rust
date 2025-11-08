use super::Builder;

/// Verifies a JWT and returns the decoded payload
///
/// This is a convenience function that wraps the Builder/Verifier internally.
///
/// # Arguments
/// * `jwt` - The JWT string to verify
/// * `public_key_hex` - Hex-encoded public verifying key
/// * `expected_issuer` - Expected issuer that must match the JWT's `iss` claim
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
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
/// let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let (jwt, _, _) = sign(MlDsaAlgo::Dsa65, "https://test.com", now + 3600, &private_key).unwrap();
/// let verified_payload = verify(&jwt, &public_key, "https://test.com").unwrap();
/// assert!(verified_payload.contains("https://test.com"));
/// ```
pub fn verify(jwt: &str, public_key_hex: &str, expected_issuer: &str) -> Result<String, String> {
    let verifier = Builder::new()
        .public_key(public_key_hex)
        .issuer(expected_issuer)
        .build()?;
    verifier.verify(jwt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MlDsaAlgo;
    use crate::keygen::generate_keypair;
    use crate::signer::sign;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_verify_valid_jwt() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt, _, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        let result = verify(&jwt, &public_key, "https://test.com");
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_verify_with_wrong_key() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key1, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (_, public_key2) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        let (jwt, _, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key1,
        )
        .unwrap();
        let result = verify(&jwt, &public_key2, "https://test.com");

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_invalid_jwt_format() {
        let (_, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let result = verify("invalid.jwt", &public_key, "https://test.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_json_payload() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (jwt, _, _) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        let result = verify(&jwt, &public_key, "https://test.com");
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.contains("https://test.com"));
    }

    #[test]
    fn test_verify_all_algorithms() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let (private_key, public_key) = generate_keypair(algo).unwrap();
            let (jwt, _, _) = sign(algo, "https://test.com", now + 3600, &private_key).unwrap();
            let result = verify(&jwt, &public_key, "https://test.com");
            assert!(result.is_ok());
        }
    }
}
