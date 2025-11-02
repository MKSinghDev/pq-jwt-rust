use super::KeyGenerator;
use crate::algorithm::MlDsaAlgo;
use std::path::PathBuf;

/// Builder for constructing a KeyGenerator with optional file saving
///
/// # Example
/// ```no_run
/// use pq_jwt::keygen::Builder;
/// use pq_jwt::MlDsaAlgo;
///
/// // Generate only (no save)
/// let (priv_key, pub_key) = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .generate()
///     .unwrap();
///
/// // Generate and save to default location (keys/)
/// let (priv_key, pub_key) = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .save_to_file()
///     .generate()
///     .unwrap();
///
/// // Generate and save to custom path
/// let (priv_key, pub_key) = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .save_to_file_at("./my-keys")
///     .generate()
///     .unwrap();
/// ```
pub struct Builder {
    algo: Option<MlDsaAlgo>,
    save_path: Option<PathBuf>,
}

impl Builder {
    /// Creates a new KeyGenerator builder
    pub fn new() -> Self {
        Self {
            algo: None,
            save_path: None,
        }
    }

    /// Sets the algorithm
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    pub fn algorithm(mut self, algo: MlDsaAlgo) -> Self {
        self.algo = Some(algo);
        self
    }

    /// Enable saving keys to the default location (keys/)
    pub fn save_to_file(mut self) -> Self {
        self.save_path = Some(PathBuf::from("keys"));
        self
    }

    /// Enable saving keys to a custom path
    ///
    /// # Arguments
    /// * `path` - Directory path where keys will be saved
    ///
    /// # Example
    /// ```no_run
    /// use pq_jwt::keygen::Builder;
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// let (priv_key, pub_key) = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .save_to_file_at("./my-custom-keys")
    ///     .generate()
    ///     .unwrap();
    /// ```
    pub fn save_to_file_at(mut self, path: impl Into<PathBuf>) -> Self {
        self.save_path = Some(path.into());
        self
    }

    /// Builds the KeyGenerator
    ///
    /// # Returns
    /// * `Ok(KeyGenerator)` - Successfully constructed generator
    /// * `Err(String)` - Missing required fields
    pub fn build(self) -> Result<KeyGenerator, String> {
        let algo = self.algo.ok_or("Algorithm is required")?;
        Ok(KeyGenerator::new(algo, self.save_path))
    }

    /// Convenience method to build and generate in one step
    ///
    /// # Returns
    /// * `Ok((private_key_hex, public_key_hex))` - Generated keys
    /// * `Err(String)` - Error message if generation fails
    ///
    /// # Example
    /// ```
    /// use pq_jwt::keygen::Builder;
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// let (priv_key, pub_key) = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .generate()
    ///     .unwrap();
    /// ```
    pub fn generate(self) -> Result<(String, String), String> {
        let generator = self.build()?;
        generator.generate()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_builder_basic() {
        let result = Builder::new().algorithm(MlDsaAlgo::Dsa65).generate();

        assert!(result.is_ok());
        let (priv_key, pub_key) = result.unwrap();
        assert!(!priv_key.is_empty());
        assert!(!pub_key.is_empty());
    }

    #[test]
    fn test_builder_missing_algorithm() {
        let result = Builder::new().generate();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Algorithm is required");
    }

    #[test]
    fn test_builder_with_default_save_path() {
        let test_dir = PathBuf::from("keys");

        let result = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .save_to_file()
            .generate();

        assert!(result.is_ok());

        // Verify directory was created
        assert!(test_dir.exists());

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_builder_with_custom_save_path() {
        let test_dir = PathBuf::from("custom_test_keys");

        let result = Builder::new()
            .algorithm(MlDsaAlgo::Dsa44)
            .save_to_file_at(&test_dir)
            .generate();

        assert!(result.is_ok());

        // Verify directory was created
        assert!(test_dir.exists());

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_builder_all_algorithms() {
        for algo in [MlDsaAlgo::Dsa44, MlDsaAlgo::Dsa65, MlDsaAlgo::Dsa87] {
            let result = Builder::new().algorithm(algo).generate();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_builder_build_then_generate() {
        let generator = Builder::new().algorithm(MlDsaAlgo::Dsa65).build().unwrap();

        let result = generator.generate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_default() {
        let builder = Builder::default();
        assert_eq!(builder.algo, None);
        assert_eq!(builder.save_path, None);
    }
}
