use super::Builder;
use crate::algorithm::MlDsaAlgo;

/// Signs JWT claims and returns a JWT string along with the public key
///
/// This is a convenience function that wraps the Builder/Signer internally.
/// The `iat` (issued at) claim defaults to the current time.
///
/// # Arguments
/// * `algo` - The ML-DSA algorithm variant to use
/// * `iss` - Issuer claim (REQUIRED)
/// * `exp` - Expiration time as Unix timestamp in seconds (REQUIRED)
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
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
///
/// let (jwt, public_key) = sign(
///     MlDsaAlgo::Dsa65,
///     "https://myapp.com",
///     now + 3600,
///     &private_key
/// ).unwrap();
/// ```
pub fn sign(
    algo: MlDsaAlgo,
    iss: &str,
    exp: u64,
    private_key_hex: &str,
) -> Result<(String, String), String> {
    let signer = Builder::new()
        .algorithm(algo)
        .private_key(private_key_hex)
        .issuer(iss)
        .expiration(exp)
        .build()?;

    signer.sign()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_keypair;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_sign_basic() {
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        );
        assert!(result.is_ok());

        let (jwt, returned_pub_key) = result.unwrap();
        assert_eq!(public_key, returned_pub_key);
        assert!(jwt.contains('.'));
        assert_eq!(jwt.split('.').count(), 3); // header.payload.signature
    }

    #[test]
    fn test_sign_with_issuer_and_expiration() {
        let (private_key, _) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = sign(
            MlDsaAlgo::Dsa65,
            "https://myapp.com",
            now + 3600,
            &private_key,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_sign_with_invalid_key() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let result = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            "invalid_hex",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_all_algorithms() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let (private_key, _) = generate_keypair(algo).unwrap();
            let result = sign(algo, "https://test.com", now + 3600, &private_key);
            assert!(result.is_ok());
        }
    }
}
