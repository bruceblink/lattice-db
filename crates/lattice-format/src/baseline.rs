use std::fmt::{Display, Formatter};

use crate::{CURRENT_FILE_FORMAT_VERSION, CURRENT_PROTOCOL_VERSION};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolBaseline {
    pub protocol_version: u16,
    pub file_format_version: u16,
    pub max_record_bytes: u64,
    pub max_batch_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatError {
    UnsupportedProtocolVersion(u16),
    UnsupportedFileFormatVersion(u16),
    ZeroRecordCapacity,
    BatchSmallerThanRecord,
}

impl Display for FormatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedProtocolVersion(version) => {
                write!(formatter, "unsupported protocol version: {version}")
            }
            Self::UnsupportedFileFormatVersion(version) => {
                write!(formatter, "unsupported file format version: {version}")
            }
            Self::ZeroRecordCapacity => formatter.write_str("record capacity must be non-zero"),
            Self::BatchSmallerThanRecord => {
                formatter.write_str("batch capacity must be at least record capacity")
            }
        }
    }
}

impl std::error::Error for FormatError {}

/// Returns the initial version and capacity values used by boundary checks.
pub fn stage_zero_baseline() -> ProtocolBaseline {
    ProtocolBaseline {
        protocol_version: CURRENT_PROTOCOL_VERSION,
        file_format_version: CURRENT_FILE_FORMAT_VERSION,
        max_record_bytes: 1024 * 1024,
        max_batch_bytes: 4 * 1024 * 1024,
    }
}

impl ProtocolBaseline {
    /// Checks that protocol versions and capacity relationships are safe to use.
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.protocol_version != CURRENT_PROTOCOL_VERSION {
            return Err(FormatError::UnsupportedProtocolVersion(
                self.protocol_version,
            ));
        }
        if self.file_format_version != CURRENT_FILE_FORMAT_VERSION {
            return Err(FormatError::UnsupportedFileFormatVersion(
                self.file_format_version,
            ));
        }
        if self.max_record_bytes == 0 {
            return Err(FormatError::ZeroRecordCapacity);
        }
        if self.max_batch_bytes < self.max_record_bytes {
            return Err(FormatError::BatchSmallerThanRecord);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_default_baseline() {
        assert!(stage_zero_baseline().validate().is_ok());
    }

    #[test]
    fn rejects_each_invalid_baseline_value() {
        let mut baseline = stage_zero_baseline();
        baseline.protocol_version = 2;
        assert_eq!(
            baseline.validate(),
            Err(FormatError::UnsupportedProtocolVersion(2))
        );

        let mut baseline = stage_zero_baseline();
        baseline.file_format_version = 2;
        assert_eq!(
            baseline.validate(),
            Err(FormatError::UnsupportedFileFormatVersion(2))
        );

        let mut baseline = stage_zero_baseline();
        baseline.max_record_bytes = 0;
        assert_eq!(baseline.validate(), Err(FormatError::ZeroRecordCapacity));

        let mut baseline = stage_zero_baseline();
        baseline.max_batch_bytes = baseline.max_record_bytes - 1;
        assert_eq!(
            baseline.validate(),
            Err(FormatError::BatchSmallerThanRecord)
        );
    }

    #[test]
    fn formats_each_baseline_error() {
        assert!(
            FormatError::UnsupportedProtocolVersion(2)
                .to_string()
                .contains("protocol")
        );
        assert!(
            FormatError::UnsupportedFileFormatVersion(2)
                .to_string()
                .contains("file format")
        );
        assert!(
            FormatError::ZeroRecordCapacity
                .to_string()
                .contains("record")
        );
        assert!(
            FormatError::BatchSmallerThanRecord
                .to_string()
                .contains("batch")
        );
    }
}
