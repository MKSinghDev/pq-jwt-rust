use super::Builder;
use crate::algorithm::MlDsaAlgo;

/// Generates a new keypair for the specified ML-DSA algorithm variant
///
/// This is a convenience function that wraps the Builder internally.
///
/// # Arguments
/// * `algo` - The ML-DSA algorithm variant to use
///
/// # Returns
/// * `Ok((private_key_hex, public_key_hex))` - Hex-encoded private and public keys
/// * `Err(String)` - Error message if key generation fails
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
///
/// let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
/// ```
pub fn generate_keypair(algo: MlDsaAlgo) -> Result<(String, String), String> {
    Builder::new().algorithm(algo).generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair_all_variants() {
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let result = generate_keypair(algo);
            assert!(result.is_ok());

            let (private_key, public_key) = result.unwrap();
            assert!(!private_key.is_empty());
            assert!(!public_key.is_empty());

            // Verify they are valid hex strings
            assert!(hex::decode(&private_key).is_ok());
            assert!(hex::decode(&public_key).is_ok());
        }
    }

    #[test]
    fn test_keypairs_are_different() {
        let (priv1, pub1) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();
        let (priv2, pub2) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        assert_ne!(priv1, priv2);
        assert_ne!(pub1, pub2);
    }
}
