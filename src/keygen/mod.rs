mod builder;
mod generate;

pub use builder::Builder;
pub use generate::generate_keypair;

use crate::algorithm::MlDsaAlgo;

/// Indicates the source of a keypair when using load_or_generate
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySource {
    /// Successfully loaded existing key from file or string
    Loaded,
    /// Generated new key (file was missing or corrupt)
    Generated,
}
use ml_dsa::{KeyGen as MlDsaKeyGen, KeyPair, MlDsa44, MlDsa65, MlDsa87};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// A key generator that can generate and optionally save keypairs
///
/// # Example
/// ```no_run
/// use pq_jwt::keygen::Builder;
/// use pq_jwt::MlDsaAlgo;
///
/// // Generate and save to default location (keys/)
/// let (priv_key, pub_key) = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .save_to_file()
///     .generate()
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct KeyGenerator {
    algo: MlDsaAlgo,
    save_path: Option<PathBuf>,
}

impl KeyGenerator {
    /// Creates a new KeyGenerator with the specified configuration
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    /// * `save_path` - Optional path to save keys
    pub(crate) fn new(algo: MlDsaAlgo, save_path: Option<PathBuf>) -> Self {
        Self { algo, save_path }
    }

    /// Generates a keypair and optionally saves to file
    ///
    /// # Returns
    /// * `Ok((private_key_hex, public_key_hex))` - Hex-encoded keys
    /// * `Err(String)` - Error message if generation or save fails
    ///
    /// # Example
    /// ```no_run
    /// use pq_jwt::keygen::Builder;
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// let generator = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .save_to_file()
    ///     .build()
    ///     .unwrap();
    ///
    /// let (priv_key, pub_key) = generator.generate().unwrap();
    /// ```
    pub fn generate(&self) -> Result<(String, String), String> {
        // Generate keypair based on algorithm
        let (private_key_hex, public_key_hex) = match self.algo {
            MlDsaAlgo::Dsa44 => self.generate_impl::<MlDsa44>()?,
            MlDsaAlgo::Dsa65 => self.generate_impl::<MlDsa65>()?,
            MlDsaAlgo::Dsa87 => self.generate_impl::<MlDsa87>()?,
        };

        // Save to file if path is specified
        if let Some(path) = &self.save_path {
            self.save_keys_to_file(path, &private_key_hex, &public_key_hex)?;
        }

        Ok((private_key_hex, public_key_hex))
    }

    fn generate_impl<P>(&self) -> Result<(String, String), String>
    where
        P: MlDsaKeyGen<KeyPair = KeyPair<P>>,
    {
        let mut rng = rand::rng();
        let kp = P::key_gen(&mut rng);

        // Extract and encode keys
        let signing_key_encoded = kp.signing_key().encode();
        let verifying_key_encoded = kp.verifying_key().encode();

        Ok((
            hex::encode(&signing_key_encoded[..]),
            hex::encode(&verifying_key_encoded[..]),
        ))
    }

    fn save_keys_to_file(
        &self,
        path: &PathBuf,
        private_key: &str,
        public_key: &str,
    ) -> Result<(), String> {
        // Create directory if it doesn't exist
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory {}: {}", path.display(), e))?;

        // Generate filenames based on algorithm
        let algo_str = self.algo.as_str().to_lowercase().replace("-", "_");
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Failed to get timestamp: {}", e))?
            .as_secs();

        let private_key_file = path.join(format!("{}_{}_private.key", algo_str, timestamp));
        let public_key_file = path.join(format!("{}_{}_public.key", algo_str, timestamp));

        // Save private key
        let mut priv_file = fs::File::create(&private_key_file)
            .map_err(|e| format!("Failed to create private key file: {}", e))?;
        priv_file
            .write_all(private_key.as_bytes())
            .map_err(|e| format!("Failed to write private key: {}", e))?;

        // Save public key
        let mut pub_file = fs::File::create(&public_key_file)
            .map_err(|e| format!("Failed to create public key file: {}", e))?;
        pub_file
            .write_all(public_key.as_bytes())
            .map_err(|e| format!("Failed to write public key: {}", e))?;

        Ok(())
    }

    /// Returns the algorithm being used by this generator
    pub fn algorithm(&self) -> MlDsaAlgo {
        self.algo
    }

    /// Returns the save path if set
    pub fn save_path(&self) -> Option<&PathBuf> {
        self.save_path.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_keygen_basic() {
        let generator = KeyGenerator::new(MlDsaAlgo::Dsa65, None);
        let result = generator.generate();

        assert!(result.is_ok());
        let (priv_key, pub_key) = result.unwrap();
        assert!(!priv_key.is_empty());
        assert!(!pub_key.is_empty());
    }

    #[test]
    fn test_keygen_all_algorithms() {
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let generator = KeyGenerator::new(algo, None);
            let result = generator.generate();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_keygen_getters() {
        let path = PathBuf::from("test/keys");
        let generator = KeyGenerator::new(MlDsaAlgo::Dsa87, Some(path.clone()));

        assert_eq!(generator.algorithm(), MlDsaAlgo::Dsa87);
        assert_eq!(generator.save_path(), Some(&path));
    }

    #[test]
    fn test_keygen_save_to_file() {
        let test_dir = PathBuf::from("test_keys_temp");
        let generator = KeyGenerator::new(MlDsaAlgo::Dsa65, Some(test_dir.clone()));

        let result = generator.generate();
        assert!(result.is_ok());

        // Verify directory was created
        assert!(test_dir.exists());

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }
}
