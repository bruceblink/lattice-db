use serde::Deserialize;
use std::{collections::BTreeSet, fmt::Display};

use crate::{CURRENT_FILE_FORMAT_VERSION, CURRENT_PROTOCOL_VERSION};

const FROZEN_SPEC: &str = include_str!("../../../docs/stage-0/frozen-spec-v0.1.toml");
const FAULT_MATRIX: &str = include_str!("../../../docs/stage-0/fault-matrix-v0.1.toml");
const REQUIRED_FRAME_TYPES: [(&str, u8); 6] = [
    ("begin", 1),
    ("operation", 2),
    ("commit", 3),
    ("change_event", 4),
    ("cursor", 5),
    ("snapshot_marker", 6),
];
const REQUIRED_ERROR_CODES: [&str; 9] = [
    "E_FORMAT_VERSION",
    "E_FRAME_TYPE",
    "E_FRAME_LENGTH",
    "E_CHECKSUM",
    "E_TRUNCATED_TAIL",
    "E_DISK_FULL",
    "E_SEQUENCE_GAP",
    "E_DUPLICATE_OPERATION",
    "E_QUARANTINED_EVENT",
];
const REQUIRED_OUTCOMES: [&str; 5] = [
    "accepted",
    "duplicate",
    "pending_gap",
    "rejected",
    "quarantined",
];
const REQUIRED_FAULT_IDS: [&str; 16] = [
    "F-001", "F-002", "F-003", "F-004", "F-005", "F-006", "F-007", "F-008", "F-009", "F-010",
    "F-011", "F-012", "F-013", "F-014", "F-015", "F-016",
];
const REQUIRED_FAULT_CATEGORIES: [&str; 7] = [
    "process_crash",
    "storage_corruption",
    "compatibility",
    "capacity",
    "data_semantics",
    "network",
    "restart",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageZeroSummary {
    pub spec_version: String,
    pub fault_cases: usize,
    pub transition_rules: usize,
    pub spec_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageZeroError {
    Parse(String),
    Validation(String),
}

impl Display for StageZeroError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(detail) => write!(formatter, "stage 0 parse error: {detail}"),
            Self::Validation(detail) => write!(formatter, "stage 0 validation error: {detail}"),
        }
    }
}

impl std::error::Error for StageZeroError {}

#[derive(Debug, Clone, Deserialize)]
struct FrozenSpec {
    spec_version: String,
    protocol_version: u16,
    file_format_version: u16,
    encoding: String,
    text_encoding: String,
    checksum: String,
    max_record_bytes: u64,
    max_batch_bytes: u64,
    max_transaction_operations: u64,
    log_segment_bytes: u64,
    snapshot_bytes: u64,
    pending_queue_bytes: u64,
    node_id_bytes: u8,
    dataset_id_max_bytes: u16,
    partition_id_max_bytes: u16,
    record_id_max_bytes: u16,
    sequence_bytes: u8,
    frame_magic: String,
    frame_header_bytes: u16,
    frame_types: FrameTypes,
    durability: Durability,
    error_codes: Vec<String>,
    event_outcomes: Vec<String>,
    transitions: Vec<TransitionRule>,
}

#[derive(Debug, Clone, Deserialize)]
struct FrameTypes {
    begin: u8,
    operation: u8,
    commit: u8,
    change_event: u8,
    cursor: u8,
    snapshot_marker: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct Durability {
    local_committed: String,
    hub_received: String,
    hub_applied: String,
}

#[derive(Debug, Clone, Deserialize)]
struct TransitionRule {
    from: String,
    to: String,
    trigger: String,
    persisted: String,
    cursor_effect: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FaultMatrix {
    schema_version: String,
    cases: Vec<FaultCase>,
}

#[derive(Debug, Clone, Deserialize)]
struct FaultCase {
    id: String,
    category: String,
    injection_point: String,
    precondition: String,
    input: String,
    expected_outcome: String,
    durable_artifacts: String,
    error_code: String,
    retry_action: String,
    cursor_effect: String,
    isolation_scope: String,
}

/// Parses and validates the versioned stage 0 documents used by the CLI.
pub fn validate_stage_zero_documents() -> Result<StageZeroSummary, StageZeroError> {
    validate_stage_zero_documents_from(FROZEN_SPEC, FAULT_MATRIX)
}

fn validate_stage_zero_documents_from(
    spec_document: &str,
    matrix_document: &str,
) -> Result<StageZeroSummary, StageZeroError> {
    let spec = parse_document::<FrozenSpec>(spec_document, "frozen specification")?;
    let matrix = parse_document::<FaultMatrix>(matrix_document, "fault matrix")?;
    validate_spec(&spec)?;
    validate_matrix(&spec, &matrix)?;
    Ok(StageZeroSummary {
        spec_version: spec.spec_version,
        fault_cases: matrix.cases.len(),
        transition_rules: spec.transitions.len(),
        spec_hash: stable_hash(spec_document, matrix_document),
    })
}

/// Decodes one embedded TOML document and adds its role to parse failures.
fn parse_document<T: for<'de> Deserialize<'de>>(
    document: &str,
    role: &str,
) -> Result<T, StageZeroError> {
    toml::from_str(document).map_err(|error| StageZeroError::Parse(format!("{role}: {error}")))
}

/// Checks the frozen protocol, frame, capacity, and transition rules.
fn validate_spec(spec: &FrozenSpec) -> Result<(), StageZeroError> {
    require(spec.spec_version == "0.1", "spec_version must be 0.1")?;
    require(
        spec.protocol_version == CURRENT_PROTOCOL_VERSION,
        "protocol_version mismatch",
    )?;
    require(
        spec.file_format_version == CURRENT_FILE_FORMAT_VERSION,
        "file_format_version mismatch",
    )?;
    require(
        spec.encoding == "little-endian",
        "encoding must be little-endian",
    )?;
    require(spec.text_encoding == "utf-8", "text_encoding must be utf-8")?;
    require(spec.checksum == "crc32c", "checksum must be crc32c")?;
    require(
        spec.max_record_bytes > 0,
        "max_record_bytes must be positive",
    )?;
    require(
        spec.max_batch_bytes >= spec.max_record_bytes,
        "batch capacity is too small",
    )?;
    require(
        spec.max_transaction_operations > 0,
        "max_transaction_operations must be positive",
    )?;
    require(
        spec.log_segment_bytes >= spec.max_batch_bytes,
        "log segment is too small",
    )?;
    require(
        spec.snapshot_bytes >= spec.max_record_bytes,
        "snapshot capacity is too small",
    )?;
    require(
        spec.pending_queue_bytes >= spec.max_batch_bytes,
        "pending queue is too small",
    )?;
    require(spec.node_id_bytes == 16, "node_id_bytes must be 16")?;
    require(
        spec.dataset_id_max_bytes > 0,
        "dataset_id_max_bytes must be positive",
    )?;
    require(
        spec.partition_id_max_bytes > 0,
        "partition_id_max_bytes must be positive",
    )?;
    require(
        spec.record_id_max_bytes > 0,
        "record_id_max_bytes must be positive",
    )?;
    require(spec.sequence_bytes == 8, "sequence_bytes must be 8")?;
    require(spec.frame_magic == "LDB1", "frame_magic must be LDB1")?;
    require(
        spec.frame_header_bytes == 26,
        "frame_header_bytes must be 26",
    )?;
    validate_frame_types(&spec.frame_types)?;
    validate_durability(&spec.durability)?;
    validate_unique_values("error_codes", &spec.error_codes)?;
    validate_required_values("error_codes", &spec.error_codes, &REQUIRED_ERROR_CODES)?;
    validate_unique_values("event_outcomes", &spec.event_outcomes)?;
    validate_required_values("event_outcomes", &spec.event_outcomes, &REQUIRED_OUTCOMES)?;
    validate_transitions(&spec.transitions)
}

/// Checks all frame type names and values against the frozen wire format.
fn validate_frame_types(types: &FrameTypes) -> Result<(), StageZeroError> {
    let actual = [
        ("begin", types.begin),
        ("operation", types.operation),
        ("commit", types.commit),
        ("change_event", types.change_event),
        ("cursor", types.cursor),
        ("snapshot_marker", types.snapshot_marker),
    ];
    let values: Vec<u8> = actual.iter().map(|(_, value)| *value).collect();
    require(
        values.iter().collect::<BTreeSet<_>>().len() == values.len(),
        "frame type values duplicate",
    )?;
    for (name, expected) in REQUIRED_FRAME_TYPES {
        let value = actual
            .iter()
            .find(|(actual_name, _)| *actual_name == name)
            .map(|(_, value)| *value);
        require(
            value == Some(expected),
            &format!("frame type {name} must be {expected}"),
        )?;
    }
    Ok(())
}

/// Checks that every durability state has a non-empty persistence rule.
fn validate_durability(durability: &Durability) -> Result<(), StageZeroError> {
    for (name, value) in [
        ("local_committed", &durability.local_committed),
        ("hub_received", &durability.hub_received),
        ("hub_applied", &durability.hub_applied),
    ] {
        require(
            !value.trim().is_empty(),
            &format!("durability rule {name} is empty"),
        )?;
    }
    Ok(())
}

/// Checks the exact monotonic state transitions required by the baseline.
fn validate_transitions(transitions: &[TransitionRule]) -> Result<(), StageZeroError> {
    let required = [
        ("local_committed", "local_committed"),
        ("local_committed", "hub_received"),
        ("hub_received", "hub_received"),
        ("hub_received", "hub_applied"),
        ("hub_applied", "hub_applied"),
    ];
    require(
        transitions.len() == required.len(),
        "transition rule count mismatch",
    )?;
    let mut seen = BTreeSet::new();
    for transition in transitions {
        require(
            !transition.trigger.trim().is_empty(),
            "transition trigger is empty",
        )?;
        require(
            !transition.persisted.trim().is_empty(),
            "transition persistence rule is empty",
        )?;
        require(
            !transition.cursor_effect.trim().is_empty(),
            "transition cursor effect is empty",
        )?;
        require(
            seen.insert((transition.from.as_str(), transition.to.as_str())),
            "transition pair is duplicated",
        )?;
    }
    for pair in required {
        require(
            seen.contains(&pair),
            &format!("missing transition {} -> {}", pair.0, pair.1),
        )?;
    }
    Ok(())
}

/// Checks the fault matrix schema, case identities, categories, and references.
fn validate_matrix(spec: &FrozenSpec, matrix: &FaultMatrix) -> Result<(), StageZeroError> {
    require(
        matrix.schema_version == "0.1",
        "fault matrix schema_version must be 0.1",
    )?;
    require(
        matrix.cases.len() == REQUIRED_FAULT_IDS.len(),
        "fault case count mismatch",
    )?;
    let ids: BTreeSet<&str> = matrix.cases.iter().map(|case| case.id.as_str()).collect();
    require(ids.len() == matrix.cases.len(), "fault case IDs duplicate")?;
    for id in REQUIRED_FAULT_IDS {
        require(ids.contains(id), &format!("missing fault case {id}"))?;
    }
    let categories: BTreeSet<&str> = matrix
        .cases
        .iter()
        .map(|case| case.category.as_str())
        .collect();
    for category in REQUIRED_FAULT_CATEGORIES {
        require(
            categories.contains(category),
            &format!("missing fault category {category}"),
        )?;
    }
    let error_codes: BTreeSet<&str> = spec.error_codes.iter().map(String::as_str).collect();
    let outcomes: BTreeSet<&str> = spec.event_outcomes.iter().map(String::as_str).collect();
    for case in &matrix.cases {
        validate_fault_case(case, &error_codes, &outcomes)?;
    }
    Ok(())
}

/// Checks required text and references for one fault case.
fn validate_fault_case(
    case: &FaultCase,
    error_codes: &BTreeSet<&str>,
    outcomes: &BTreeSet<&str>,
) -> Result<(), StageZeroError> {
    for (field, value) in [
        ("category", &case.category),
        ("injection_point", &case.injection_point),
        ("precondition", &case.precondition),
        ("input", &case.input),
        ("expected_outcome", &case.expected_outcome),
        ("durable_artifacts", &case.durable_artifacts),
        ("retry_action", &case.retry_action),
        ("cursor_effect", &case.cursor_effect),
        ("isolation_scope", &case.isolation_scope),
    ] {
        require(
            !value.trim().is_empty(),
            &format!("{} has empty {field}", case.id),
        )?;
    }
    require(
        outcomes.contains(case.expected_outcome.as_str()),
        &format!("{} references unknown outcome", case.id),
    )?;
    require(
        case.error_code.is_empty() || error_codes.contains(case.error_code.as_str()),
        &format!("{} references unknown error code", case.id),
    )?;
    Ok(())
}

/// Returns a stable FNV-1a hash over the exact frozen documents.
fn stable_hash(spec: &str, matrix: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in spec.bytes().chain([0_u8]).chain(matrix.bytes()) {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn validate_unique_values(name: &str, values: &[String]) -> Result<(), StageZeroError> {
    require(
        values.iter().collect::<BTreeSet<_>>().len() == values.len(),
        &format!("{name} contain duplicate values"),
    )
}

fn validate_required_values(
    name: &str,
    values: &[String],
    required: &[&str],
) -> Result<(), StageZeroError> {
    let actual: BTreeSet<&str> = values.iter().map(String::as_str).collect();
    for item in required {
        require(actual.contains(item), &format!("{name} missing {item}"))?;
    }
    Ok(())
}

fn require(condition: bool, detail: &str) -> Result<(), StageZeroError> {
    if condition {
        Ok(())
    } else {
        Err(StageZeroError::Validation(detail.to_owned()))
    }
}

#[cfg(test)]
#[path = "stage_zero_tests.rs"]
mod tests;
