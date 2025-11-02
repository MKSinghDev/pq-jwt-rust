use crate::algorithm::MlDsaAlgo;
use ml_dsa::{KeyGen, KeyPair, MlDsa44, MlDsa65, MlDsa87};

/// Generates a new keypair for the specified ML-DSA algorithm variant
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
    match algo {
        MlDsaAlgo::Dsa44 => generate_keypair_impl::<MlDsa44>(),
        MlDsaAlgo::Dsa65 => generate_keypair_impl::<MlDsa65>(),
        MlDsaAlgo::Dsa87 => generate_keypair_impl::<MlDsa87>(),
    }
}

fn generate_keypair_impl<P>() -> Result<(String, String), String>
where
    P: KeyGen<KeyPair = KeyPair<P>>,
{
    let mut rng = rand::rng();
    let kp = P::key_gen(&mut rng);

    // Extract and encode the signing key
    let signing_key_encoded = kp.signing_key().encode();
    let verifying_key_encoded = kp.verifying_key().encode();

    Ok((
        hex::encode(&signing_key_encoded[..]),
        hex::encode(&verifying_key_encoded[..]),
    ))
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
