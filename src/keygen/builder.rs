use super::{KeyGenerator, KeySource};
use crate::algorithm::MlDsaAlgo;
use ml_dsa::{EncodedSigningKey, KeyGen, MlDsa44, MlDsa65, MlDsa87, SigningKey};
use std::fs;
use std::path::PathBuf;

/// Mode for the builder operation
#[derive(Debug, Clone, PartialEq)]
enum BuilderMode {
    GenerateOnly,   // Builder::new() - just generate
    LoadOnly,       // Builder::from() - just load (error if missing)
    LoadOrGenerate, // Builder::load_or_generate() - try load, generate if missing
}

/// Builder for constructing a KeyGenerator with optional file saving and loading
///
/// # Example
/// ```
/// use pq_jwt::keygen::{Builder, KeySource};
/// use pq_jwt::MlDsaAlgo;
///
/// // Generate keypair
/// let (priv_key, pub_key) = Builder::new()
///     .algorithm(MlDsaAlgo::Dsa65)
///     .generate()
///     .unwrap();
///
/// // Load from string (e.g., from database)
/// let (loaded_priv, loaded_pub, source) = Builder::from(MlDsaAlgo::Dsa65)
///     .private_key_str(&priv_key)
///     .unwrap();
///
/// assert_eq!(priv_key, loaded_priv);
/// assert_eq!(pub_key, loaded_pub);
/// assert_eq!(source, KeySource::Loaded);
/// ```
///
/// File loading example (requires files to exist):
/// ```no_run
/// use pq_jwt::keygen::{Builder, KeySource};
/// use pq_jwt::MlDsaAlgo;
///
/// // Load from default location (keys/)
/// let (priv_key, pub_key, _source) = Builder::from(MlDsaAlgo::Dsa65)
///     .file()
///     .unwrap();
///
/// // Load from custom path
/// let (priv_key, pub_key, source) = Builder::from(MlDsaAlgo::Dsa65)
///     .file_at("./my-keys")
///     .unwrap();
/// ```
pub struct Builder {
    algo: Option<MlDsaAlgo>,
    save_path: Option<PathBuf>,
    mode: BuilderMode,
}

impl Builder {
    /// Creates a new KeyGenerator builder for generation
    pub fn new() -> Self {
        Self {
            algo: None,
            save_path: None,
            mode: BuilderMode::GenerateOnly,
        }
    }

    /// Creates a builder from an algorithm (for loading keys)
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    ///
    /// # Example
    /// ```no_run
    /// use pq_jwt::keygen::{Builder, KeySource};
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// let (priv_key, pub_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    ///     .file()
    ///     .unwrap();
    /// ```
    pub fn from(algo: MlDsaAlgo) -> Self {
        Self {
            algo: Some(algo),
            save_path: None,
            mode: BuilderMode::LoadOnly,
        }
    }

    /// Creates a builder for load-or-generate mode
    ///
    /// Attempts to load existing keys. If not found or corrupt, generates new keys and saves them.
    ///
    /// # Arguments
    /// * `algo` - The ML-DSA algorithm variant
    ///
    /// # Example
    /// ```no_run
    /// use pq_jwt::keygen::{Builder, KeySource};
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// let (priv_key, pub_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    ///     .file()
    ///     .unwrap();
    ///
    /// match source {
    ///     KeySource::Loaded => println!("Loaded existing key"),
    ///     KeySource::Generated => println!("Generated new key"),
    /// }
    /// ```
    pub fn load_or_generate(algo: MlDsaAlgo) -> Self {
        Self {
            algo: Some(algo),
            save_path: None,
            mode: BuilderMode::LoadOrGenerate,
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

    /// Loads keypair from file in default location (keys/)
    ///
    /// Behavior depends on builder mode:
    /// - `Builder::from()`: Load only (error if missing)
    /// - `Builder::load_or_generate()`: Load or generate + save if missing
    ///
    /// # Returns
    /// - `Builder::from()`: `Ok((private_key_hex, public_key_hex))`
    /// - `Builder::load_or_generate()`: `Ok((private_key_hex, public_key_hex, KeySource))`
    ///
    /// # Example
    /// ```no_run
    /// use pq_jwt::keygen::{Builder, KeySource};
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// // Load only
    /// let (priv_key, pub_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    ///     .file()
    ///     .unwrap();
    ///
    /// // Load or generate
    /// let (priv_key, pub_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    ///     .file()
    ///     .unwrap();
    /// ```
    pub fn file(self) -> Result<(String, String, KeySource), String> {
        self.file_at("keys")
    }

    /// Loads keypair from file in custom location
    ///
    /// Behavior depends on builder mode:
    /// - `Builder::from()`: Load only (error if missing)
    /// - `Builder::load_or_generate()`: Load or generate + save if missing
    ///
    /// # Arguments
    /// * `path` - Directory path where keys are stored
    ///
    /// # Returns
    /// Returns `(private_key_hex, public_key_hex, KeySource)`
    ///
    /// # Example
    /// ```no_run
    /// use pq_jwt::keygen::{Builder, KeySource};
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// // Load only mode
    /// let (priv_key, pub_key, _) = Builder::from(MlDsaAlgo::Dsa65)
    ///     .file_at("./my-keys")
    ///     .unwrap();
    ///
    /// // Load or generate mode
    /// let (priv_key, pub_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    ///     .file_at("./my-keys")
    ///     .unwrap();
    /// ```
    pub fn file_at(self, path: impl Into<PathBuf>) -> Result<(String, String, KeySource), String> {
        let algo = self.algo.ok_or("Algorithm is required")?;
        let path = path.into();

        // Try to load existing key
        match find_latest_key_file(&path, &algo) {
            Ok(private_key_path) => {
                // Key file exists, try to load it
                match fs::read_to_string(&private_key_path) {
                    Ok(private_key_hex) => {
                        // Successfully read private key, try to derive public key
                        match derive_public_key_from_private(&private_key_hex, algo) {
                            Ok(public_key_hex) => {
                                // Successfully loaded and derived
                                Ok((private_key_hex, public_key_hex, KeySource::Loaded))
                            }
                            Err(_) if self.mode == BuilderMode::LoadOrGenerate => {
                                // Corrupt key, generate new one
                                let generator = KeyGenerator::new(algo, Some(path));
                                let (priv_key, pub_key) = generator.generate()?;
                                Ok((priv_key, pub_key, KeySource::Generated))
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(_) if self.mode == BuilderMode::LoadOrGenerate => {
                        // Can't read file, generate new one
                        let generator = KeyGenerator::new(algo, Some(path));
                        let (priv_key, pub_key) = generator.generate()?;
                        Ok((priv_key, pub_key, KeySource::Generated))
                    }
                    Err(e) => Err(format!(
                        "Failed to read private key from {:?}: {}",
                        private_key_path, e
                    )),
                }
            }
            Err(_) if self.mode == BuilderMode::LoadOrGenerate => {
                // No key found, generate new one
                let generator = KeyGenerator::new(algo, Some(path));
                let (priv_key, pub_key) = generator.generate()?;
                Ok((priv_key, pub_key, KeySource::Generated))
            }
            Err(e) => Err(e),
        }
    }

    /// Loads keypair from private key string
    ///
    /// Public key is derived from the private key.
    /// Useful for loading keys from database or environment variables.
    /// Always returns `KeySource::Loaded` (no save functionality for string mode).
    ///
    /// # Arguments
    /// * `private_key_hex` - Hex-encoded private key string
    ///
    /// # Returns
    /// * `Ok((private_key_hex, public_key_hex, KeySource::Loaded))` - Keys and source
    /// * `Err(String)` - Error if derivation fails
    ///
    /// # Example
    /// ```
    /// use pq_jwt::keygen::{Builder, KeySource};
    /// use pq_jwt::MlDsaAlgo;
    ///
    /// // First generate a key
    /// let (orig_priv, orig_pub) = Builder::new()
    ///     .algorithm(MlDsaAlgo::Dsa65)
    ///     .generate()
    ///     .unwrap();
    ///
    /// // Load from string (e.g., retrieved from database)
    /// let (priv_key, pub_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    ///     .private_key_str(&orig_priv)
    ///     .unwrap();
    ///
    /// assert_eq!(orig_pub, pub_key);
    /// assert_eq!(source, KeySource::Loaded);
    /// ```
    pub fn private_key_str(
        self,
        private_key_hex: &str,
    ) -> Result<(String, String, KeySource), String> {
        let algo = self.algo.ok_or("Algorithm is required")?;
        let public_key_hex = derive_public_key_from_private(private_key_hex, algo)?;
        Ok((
            private_key_hex.to_string(),
            public_key_hex,
            KeySource::Loaded,
        ))
    }
}

/// Finds the latest private key file for the given algorithm in the specified path
fn find_latest_key_file(path: &PathBuf, algo: &MlDsaAlgo) -> Result<PathBuf, String> {
    let algo_str = match algo {
        MlDsaAlgo::Dsa44 => "ml_dsa_44",
        MlDsaAlgo::Dsa65 => "ml_dsa_65",
        MlDsaAlgo::Dsa87 => "ml_dsa_87",
    };

    // Read directory
    let entries =
        fs::read_dir(path).map_err(|e| format!("Failed to read directory {:?}: {}", path, e))?;

    // Find all matching private key files
    let mut matching_files: Vec<(u64, PathBuf)> = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Check if file matches pattern: ml_dsa_XX_TIMESTAMP_private.key
        if file_name_str.starts_with(algo_str) && file_name_str.ends_with("_private.key") {
            // Extract timestamp from filename
            let parts: Vec<&str> = file_name_str.split('_').collect();
            if parts.len() >= 4
                && let Ok(timestamp) = parts[3].parse::<u64>()
            {
                matching_files.push((timestamp, entry.path()));
            }
        }
    }

    if matching_files.is_empty() {
        return Err(format!(
            "No {} private key found in directory {:?}",
            algo.as_str(),
            path
        ));
    }

    // Sort by timestamp (descending) - latest first
    matching_files.sort_by(|a, b| b.0.cmp(&a.0));

    // Return the latest file
    Ok(matching_files[0].1.clone())
}

/// Derives public key from private key for the given algorithm
fn derive_public_key_from_private(
    private_key_hex: &str,
    algo: MlDsaAlgo,
) -> Result<String, String> {
    match algo {
        MlDsaAlgo::Dsa44 => derive_public_key_impl::<MlDsa44>(private_key_hex),
        MlDsaAlgo::Dsa65 => derive_public_key_impl::<MlDsa65>(private_key_hex),
        MlDsaAlgo::Dsa87 => derive_public_key_impl::<MlDsa87>(private_key_hex),
    }
}

/// Generic implementation to derive public key from private key
fn derive_public_key_impl<P>(private_key_hex: &str) -> Result<String, String>
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

    // Get public key
    let verifying_key = signing_key.verifying_key();
    let pub_key_encoded = verifying_key.encode();
    let pub_key_hex = hex::encode(&pub_key_encoded[..]);

    Ok(pub_key_hex)
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

    #[test]
    fn test_from_constructor() {
        let builder = Builder::from(MlDsaAlgo::Dsa65);
        assert_eq!(builder.algo, Some(MlDsaAlgo::Dsa65));
        assert_eq!(builder.save_path, None);
        assert_eq!(builder.mode, BuilderMode::LoadOnly);
    }

    #[test]
    fn test_load_or_generate_constructor() {
        let builder = Builder::load_or_generate(MlDsaAlgo::Dsa65);
        assert_eq!(builder.algo, Some(MlDsaAlgo::Dsa65));
        assert_eq!(builder.save_path, None);
        assert_eq!(builder.mode, BuilderMode::LoadOrGenerate);
    }

    #[test]
    fn test_private_key_str() {
        // First generate a keypair
        let (priv_key, pub_key) = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .generate()
            .unwrap();

        // Load from string
        let result = Builder::from(MlDsaAlgo::Dsa65).private_key_str(&priv_key);

        assert!(result.is_ok());
        let (loaded_priv, loaded_pub, source) = result.unwrap();
        assert_eq!(loaded_priv, priv_key);
        assert_eq!(loaded_pub, pub_key);
        assert_eq!(source, KeySource::Loaded);
    }

    #[test]
    fn test_file_load() {
        let test_dir = PathBuf::from("test_load_keys");

        // Generate and save a keypair
        let (orig_priv, orig_pub) = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .save_to_file_at(&test_dir)
            .generate()
            .unwrap();

        // Load it back
        let result = Builder::from(MlDsaAlgo::Dsa65).file_at(&test_dir);

        assert!(result.is_ok());
        let (loaded_priv, loaded_pub, source) = result.unwrap();
        assert_eq!(loaded_priv, orig_priv);
        assert_eq!(loaded_pub, orig_pub);
        assert_eq!(source, KeySource::Loaded);

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_file_load_picks_latest() {
        let test_dir = PathBuf::from("test_latest_keys");

        // Generate first keypair
        let _ = Builder::new()
            .algorithm(MlDsaAlgo::Dsa44)
            .save_to_file_at(&test_dir)
            .generate()
            .unwrap();

        // Wait a moment to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Generate second keypair (should be picked as latest)
        let (latest_priv, latest_pub) = Builder::new()
            .algorithm(MlDsaAlgo::Dsa44)
            .save_to_file_at(&test_dir)
            .generate()
            .unwrap();

        // Load - should get the latest one
        let result = Builder::from(MlDsaAlgo::Dsa44).file_at(&test_dir);

        assert!(result.is_ok());
        let (loaded_priv, loaded_pub, source) = result.unwrap();
        assert_eq!(loaded_priv, latest_priv);
        assert_eq!(loaded_pub, latest_pub);
        assert_eq!(source, KeySource::Loaded);

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_file_load_no_key_found() {
        let test_dir = PathBuf::from("empty_keys_dir");
        fs::create_dir_all(&test_dir).ok();

        let result = Builder::from(MlDsaAlgo::Dsa65).file_at(&test_dir);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("No ML-DSA-65 private key found")
        );

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_private_key_str_invalid_key() {
        let result = Builder::from(MlDsaAlgo::Dsa65).private_key_str("invalid_hex");

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid hex private key"));
    }

    #[test]
    fn test_load_or_generate_missing_key() {
        let test_dir = PathBuf::from("test_load_or_gen_missing");

        // Ensure directory doesn't exist
        fs::remove_dir_all(&test_dir).ok();

        // Should generate new key since none exists
        let result = Builder::load_or_generate(MlDsaAlgo::Dsa65).file_at(&test_dir);

        assert!(result.is_ok());
        let (priv_key, pub_key, source) = result.unwrap();
        assert!(!priv_key.is_empty());
        assert!(!pub_key.is_empty());
        assert_eq!(source, KeySource::Generated);

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_load_or_generate_existing_key() {
        let test_dir = PathBuf::from("test_load_or_gen_existing");

        // Generate and save a keypair first
        let (orig_priv, orig_pub) = Builder::new()
            .algorithm(MlDsaAlgo::Dsa65)
            .save_to_file_at(&test_dir)
            .generate()
            .unwrap();

        // Should load existing key
        let result = Builder::load_or_generate(MlDsaAlgo::Dsa65).file_at(&test_dir);

        assert!(result.is_ok());
        let (loaded_priv, loaded_pub, source) = result.unwrap();
        assert_eq!(loaded_priv, orig_priv);
        assert_eq!(loaded_pub, orig_pub);
        assert_eq!(source, KeySource::Loaded);

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }
}
