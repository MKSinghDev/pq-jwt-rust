//! # pq-jwt
//!
//! A post-quantum JWT implementation using ML-DSA (Module-Lattice Digital Signature Algorithm)
//! signatures for quantum-resistant authentication tokens.
//!
//! ## Features
//!
//! - **Quantum-Resistant**: Uses ML-DSA (FIPS 204) signatures that are secure against quantum attacks
//! - **Multiple Security Levels**: Support for ML-DSA-44, ML-DSA-65, and ML-DSA-87
//! - **Standards Compliant**: JWT format following RFC 7519
//! - **Easy to Use**: Simple API for key generation, signing, and verification
//!
//! ## Quick Start
//!
//! ```rust
//! use pq_jwt::{generate_keypair, sign, verify, MlDsaAlgo};
//! use std::time::{SystemTime, UNIX_EPOCH};
//!
//! // Generate a keypair
//! let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65)?;
//!
//! // Sign with issuer and expiration
//! let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
//! let (jwt, _public_key) = sign(
//!     MlDsaAlgo::Dsa65,
//!     "https://myapp.com",
//!     now + 3600,
//!     &private_key
//! )?;
//!
//! // Verify the JWT
//! let verified_payload = verify(&jwt, &public_key, "https://myapp.com")?;
//! assert!(verified_payload.contains("https://myapp.com"));
//! # Ok::<(), String>(())
//! ```
//!
//! ## Security Levels
//!
//! | Variant | NIST Level | Signature Size | Use Case |
//! |---------|-----------|----------------|----------|
//! | ML-DSA-44 | Category 2 | ~2.4 KB | IoT, constrained devices |
//! | ML-DSA-65 | Category 3 | ~3.3 KB | Recommended for most uses |
//! | ML-DSA-87 | Category 5 | ~4.6 KB | High security requirements |

mod algorithm;
mod header; // Internal only
pub mod keygen;
pub mod signer;
pub mod verifier;

// Re-export public API
pub use algorithm::MlDsaAlgo;
pub use keygen::{KeySource, generate_keypair};
pub use signer::sign;
pub use verifier::verify;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_full_workflow() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Generate keypair
        let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65).unwrap();

        // Sign with issuer and expiration
        let (jwt, returned_pub_key) = sign(
            MlDsaAlgo::Dsa65,
            "https://test.com",
            now + 3600,
            &private_key,
        )
        .unwrap();

        // Verify public key matches
        assert_eq!(public_key, returned_pub_key);

        // Verify JWT
        let verified_payload = verify(&jwt, &public_key, "https://test.com").unwrap();
        assert!(verified_payload.contains("https://test.com"));
    }

    #[test]
    fn test_all_algorithms() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let (private_key, public_key) = generate_keypair(algo).unwrap();
            let (jwt, _) = sign(algo, "https://test.com", now + 3600, &private_key).unwrap();
            let verified_payload = verify(&jwt, &public_key, "https://test.com").unwrap();
            assert!(verified_payload.contains("https://test.com"));
        }
    }

    #[test]
    fn test_verification_fails_with_wrong_key() {
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
        let result = verify(&jwt, &public_key2, "https://test.com");

        assert!(result.is_err());
    }
}
