//! Storage-engine boundary for Lattice DB.

use lattice_format::{FormatError, ProtocolBaseline};

#[derive(Debug, PartialEq, Eq)]
pub enum CoreError {
    InvalidFormat(FormatError),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(error) => write!(formatter, "invalid format baseline: {error}"),
        }
    }
}

impl std::error::Error for CoreError {}

/// Validates the format baseline before storage code is allowed to start.
pub fn validate_baseline(baseline: &ProtocolBaseline) -> Result<(), CoreError> {
    baseline.validate().map_err(CoreError::InvalidFormat)
}

/// Runs the storage boundary check used by the stage 0 executable.
pub fn validate_stage_zero() -> Result<(), CoreError> {
    validate_baseline(&lattice_format::stage_zero_baseline())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_stage_zero_baseline() {
        assert!(validate_stage_zero().is_ok());
    }

    #[test]
    fn preserves_format_validation_error() {
        let mut baseline = lattice_format::stage_zero_baseline();
        baseline.max_record_bytes = 0;
        assert_eq!(
            validate_baseline(&baseline),
            Err(CoreError::InvalidFormat(FormatError::ZeroRecordCapacity))
        );
    }

    #[test]
    fn formats_core_error() {
        let error = CoreError::InvalidFormat(FormatError::ZeroRecordCapacity);
        assert!(error.to_string().contains("invalid format baseline"));
    }
}
