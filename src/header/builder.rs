use super::JwtHeader;

/// Builder for constructing JWT headers with optional fields
///
/// # Example
/// ```
/// use pq_jwt::header::Builder;
///
/// let header = Builder::new()
///     .algorithm("ML-DSA-65")
///     .kid("key-2024-01")
///     .build()
///     .unwrap();
/// ```
pub struct Builder {
    alg: Option<String>,
    kid: Option<String>,
    typ: Option<String>,
}

impl Builder {
    /// Creates a new header builder with default values
    pub fn new() -> Self {
        Self {
            alg: None,
            kid: None,
            typ: Some("JWT".to_string()),
        }
    }

    /// Sets the algorithm (alg) field
    ///
    /// # Arguments
    /// * `alg` - The algorithm identifier
    pub fn algorithm(mut self, alg: impl Into<String>) -> Self {
        self.alg = Some(alg.into());
        self
    }

    /// Sets the key ID (kid) field
    ///
    /// # Arguments
    /// * `kid` - The key identifier for key rotation
    pub fn kid(mut self, kid: impl Into<String>) -> Self {
        self.kid = Some(kid.into());
        self
    }

    /// Sets the type (typ) field
    ///
    /// # Arguments
    /// * `typ` - The token type (default: "JWT")
    pub fn typ(mut self, typ: impl Into<String>) -> Self {
        self.typ = Some(typ.into());
        self
    }

    /// Builds the JwtHeader
    ///
    /// # Returns
    /// * `Ok(JwtHeader)` - Successfully constructed header
    /// * `Err(String)` - Missing required fields
    ///
    /// # Example
    /// ```
    /// use pq_jwt::header::Builder;
    ///
    /// let header = Builder::new()
    ///     .algorithm("ML-DSA-65")
    ///     .kid("key-123")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<JwtHeader, String> {
        let alg = self.alg.ok_or("Algorithm (alg) is required")?;
        let typ = self.typ.unwrap_or_else(|| "JWT".to_string());

        Ok(JwtHeader {
            alg,
            kid: self.kid,
            typ,
        })
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

    #[test]
    fn test_basic_builder() {
        let header = Builder::new().algorithm("ML-DSA-65").build().unwrap();

        assert_eq!(header.algorithm(), "ML-DSA-65");
        assert_eq!(header.typ(), "JWT");
        assert_eq!(header.key_id(), None);
    }

    #[test]
    fn test_builder_with_kid() {
        let header = Builder::new()
            .algorithm("ML-DSA-65")
            .kid("key-2024-01")
            .build()
            .unwrap();

        assert_eq!(header.algorithm(), "ML-DSA-65");
        assert_eq!(header.key_id(), Some("key-2024-01"));
        assert_eq!(header.typ(), "JWT");
    }

    #[test]
    fn test_custom_typ() {
        let header = Builder::new()
            .algorithm("ML-DSA-65")
            .typ("CustomToken")
            .build()
            .unwrap();

        assert_eq!(header.algorithm(), "ML-DSA-65");
        assert_eq!(header.typ(), "CustomToken");
    }

    #[test]
    fn test_missing_algorithm() {
        let result = Builder::new().kid("key-123").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Algorithm (alg) is required");
    }

    #[test]
    fn test_builder_chaining() {
        let header = Builder::new()
            .algorithm("ML-DSA-87")
            .kid("prod-key-001")
            .typ("JWT")
            .build()
            .unwrap();

        assert_eq!(header.algorithm(), "ML-DSA-87");
        assert_eq!(header.key_id(), Some("prod-key-001"));
        assert_eq!(header.typ(), "JWT");
    }

    #[test]
    fn test_default() {
        let builder = Builder::default();
        assert_eq!(builder.typ, Some("JWT".to_string()));
        assert_eq!(builder.alg, None);
        assert_eq!(builder.kid, None);
    }
}
