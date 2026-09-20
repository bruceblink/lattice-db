//! Transport boundary kept independent from storage semantics.

pub const DEFAULT_MAX_FRAME_BYTES: usize = 1024 * 1024;

/// Transport limits used before a real wire adapter is selected.
///
/// The limit is an encoded frame bound, not a hint for an unbounded buffer.
/// Later adapters must preserve this invariant while adding framing and I/O
/// behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportConfig {
    /// Maximum accepted frame size in bytes; zero is never valid.
    pub max_frame_bytes: usize,
}

/// Failures that would make a transport buffer unusable or unbounded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    ZeroFrameCapacity,
}

/// Returns the initial transport configuration for the stage 0 executable.
///
/// Construction is O(1) and does not open a socket or reserve a buffer.
pub fn stage_zero_config() -> TransportConfig {
    TransportConfig {
        max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
    }
}

impl TransportConfig {
    /// Ensures the transport cannot accept an empty frame size.
    ///
    /// This is an O(1) invariant check. A zero limit is rejected before any
    /// adapter can allocate; nonzero values are returned unchanged.
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
