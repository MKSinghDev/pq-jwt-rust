/// ML-DSA algorithm variants
#[derive(Debug, Clone, Copy)]
pub enum MlDsaAlgo {
    /// ML-DSA-44 - NIST Security Category 2
    Dsa44,
    /// ML-DSA-65 - NIST Security Category 3 (Recommended)
    Dsa65,
    /// ML-DSA-87 - NIST Security Category 5
    Dsa87,
}

impl MlDsaAlgo {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            MlDsaAlgo::Dsa44 => "ML-DSA-44",
            MlDsaAlgo::Dsa65 => "ML-DSA-65",
            MlDsaAlgo::Dsa87 => "ML-DSA-87",
        }
    }

    pub(crate) fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ML-DSA-44" => Ok(MlDsaAlgo::Dsa44),
            "ML-DSA-65" => Ok(MlDsaAlgo::Dsa65),
            "ML-DSA-87" => Ok(MlDsaAlgo::Dsa87),
            _ => Err(format!("Unknown algorithm: {}", s)),
        }
    }
}
