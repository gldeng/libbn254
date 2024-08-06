use core::fmt;

/// Precompile errors.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrecompileErrors {
    Error(PrecompileError),
    Fatal { msg: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrecompileError {
    // Bn128 errors
    Bn128FieldPointNotAMember,
    Bn128AffineGFailedToCreate,
    Bn128PairLength,
    /// Catch-all variant for other errors.
    Other(String),
}


impl PrecompileError {
    /// Returns an other error with the given message.
    pub fn other(err: impl Into<String>) -> Self {
        Self::Other(err.into())
    }
}

impl From<PrecompileError> for PrecompileErrors {
    fn from(err: PrecompileError) -> Self {
        PrecompileErrors::Error(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PrecompileError {}

impl fmt::Display for PrecompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bn128FieldPointNotAMember => "field point not a member of bn128 curve",
            Self::Bn128AffineGFailedToCreate => "failed to create affine g point for bn128 curve",
            Self::Bn128PairLength => "bn128 invalid pair length",
            Self::Other(s) => s,
        };
        f.write_str(s)
    }
}