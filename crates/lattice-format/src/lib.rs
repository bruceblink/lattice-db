//! Versioned protocol and file-format primitives for Lattice DB.

mod baseline;
mod stage_zero;

pub const CURRENT_FILE_FORMAT_VERSION: u16 = 1;
pub const CURRENT_PROTOCOL_VERSION: u16 = 1;

pub use baseline::{FormatError, ProtocolBaseline, stage_zero_baseline};
pub use stage_zero::{StageZeroError, StageZeroSummary, validate_stage_zero_documents};
