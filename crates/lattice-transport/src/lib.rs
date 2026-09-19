//! Transport boundary kept independent from storage semantics.

pub const DEFAULT_MAX_FRAME_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportConfig {
    pub max_frame_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    ZeroFrameCapacity,
}

/// Returns the initial transport configuration for the stage 0 executable.
pub fn stage_zero_config() -> TransportConfig {
    TransportConfig {
        max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
    }
}

impl TransportConfig {
    /// Ensures the transport cannot accept an unbounded or empty frame size.
    pub fn validate(self) -> Result<(), TransportError> {
        if self.max_frame_bytes == 0 {
            return Err(TransportError::ZeroFrameCapacity);
        }
        Ok(())
    }
}

/// Checks the transport boundary used by the stage 0 executable.
pub fn validate_stage_zero() -> Result<(), TransportError> {
    stage_zero_config().validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_zero_configuration_is_valid() {
        assert!(validate_stage_zero().is_ok());
    }

    #[test]
    fn rejects_zero_frame_capacity() {
        assert_eq!(
            (TransportConfig { max_frame_bytes: 0 }).validate(),
            Err(TransportError::ZeroFrameCapacity)
        );
    }
}
