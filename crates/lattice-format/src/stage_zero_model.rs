use serde::Deserialize;

/// Deserialized stage-0 specification model.
///
/// These types are deliberately private to the validator: runtime crates must
/// consume validated configuration rather than construct partially checked
/// protocol values. Text rules remain explicit strings so the frozen document
/// records the implementation decision before the runtime types exist.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct FrozenSpec {
    /// Human-readable revision of this machine-readable document.
    pub(super) spec_version: String,
    /// Wire protocol revision accepted by this implementation.
    pub(super) protocol_version: u16,
    /// On-disk frame and segment revision accepted by this implementation.
    pub(super) file_format_version: u16,
    /// Byte order for all fixed-width integers.
    pub(super) encoding: String,
    /// Encoding for text identifiers and document text.
    pub(super) text_encoding: String,
    /// Checksum algorithm used by frame and event records.
    pub(super) checksum: String,
    /// Maximum complete encoded record size.
    pub(super) max_record_bytes: u64,
    /// Maximum complete encoded batch size.
    pub(super) max_batch_bytes: u64,
    /// Maximum operations in one single-partition transaction.
    pub(super) max_transaction_operations: u64,
    /// Segment rotation size in complete encoded bytes.
    pub(super) log_segment_bytes: u64,
    /// Maximum encoded snapshot size.
    pub(super) snapshot_bytes: u64,
    /// Maximum pending synchronization queue size.
    pub(super) pending_queue_bytes: u64,
    /// Fixed binary node identifier width.
    pub(super) node_id_bytes: u8,
    /// Maximum UTF-8 dataset identifier length.
    pub(super) dataset_id_max_bytes: u16,
    /// Maximum UTF-8 partition identifier length.
    pub(super) partition_id_max_bytes: u16,
    /// Maximum UTF-8 record identifier length.
    pub(super) record_id_max_bytes: u16,
    /// Width of sequence values in bytes.
    pub(super) sequence_bytes: u8,
    /// Four-byte frame magic value rendered as text in the specification.
    pub(super) frame_magic: String,
    /// Fixed frame-header width; no ABI padding is implied.
    pub(super) frame_header_bytes: u16,
    /// Identifier and generation rules used by all record types.
    pub(super) identifiers: Identifiers,
    /// Sequence allocation and cursor invariants.
    pub(super) sequences: Sequences,
    /// Canonical paths and atomic replacement procedure.
    pub(super) filesystem: Filesystem,
    /// Exact byte offsets and rejection policy for frame headers.
    pub(super) frame_layout: FrameLayout,
    pub(super) frame_types: FrameTypes,
    pub(super) durability: Durability,
    pub(super) error_codes: Vec<String>,
    pub(super) event_outcomes: Vec<String>,
    pub(super) transitions: Vec<TransitionRule>,
}

/// Identifier encoding and deduplication rules.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct Identifiers {
    /// Encoding and width of the persistent node identity.
    pub(super) node_encoding: String,
    /// Node identity byte width.
    pub(super) node_bytes: u8,
    /// Cryptographically secure node identity generator.
    pub(super) node_generator: String,
    /// File boundary at which a new node identity becomes durable.
    pub(super) node_persistence: String,
    /// Whether restart may replace the node identity.
    pub(super) node_immutable: bool,
    /// Transaction identity encoding.
    pub(super) transaction_encoding: String,
    /// Transaction identity byte width.
    pub(super) transaction_bytes: u8,
    /// Transaction identity generator.
    pub(super) transaction_generator: String,
    /// Behavior when a transaction identity is reused.
    pub(super) transaction_reuse: String,
    /// Operation identity encoding.
    pub(super) operation_encoding: String,
    /// Operation identity byte width.
    pub(super) operation_bytes: u8,
    /// Operation identity generator.
    pub(super) operation_generator: String,
    /// Behavior when an operation identity is reused.
    pub(super) operation_reuse: String,
    /// Tuple used by the receiver for idempotent deduplication.
    pub(super) operation_dedup_key: String,
    /// Text identifier encoding.
    pub(super) text_encoding: String,
    /// Minimum text identifier length in encoded bytes.
    pub(super) text_min_bytes: u16,
    /// Maximum text identifier length in encoded bytes.
    pub(super) text_max_bytes: u16,
    /// Length-prefix width for variable text fields.
    pub(super) text_length_prefix: String,
    /// Equality rule; no locale or case folding is allowed.
    pub(super) text_equality: String,
    /// Unicode normalization policy.
    pub(super) text_normalization: String,
    /// NUL-byte policy for path-safe text identifiers.
    pub(super) text_nul: String,
}

/// Sequence allocation and contiguous cursor rules.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct Sequences {
    /// Tuple that owns an origin sequence space.
    pub(super) origin_scope: String,
    /// Origin sequence width in bits.
    pub(super) origin_width_bits: u8,
    /// First valid origin sequence; zero means absent.
    pub(super) origin_initial: u64,
    /// Commit boundary at which origin sequence allocation occurs.
    pub(super) origin_assignment: String,
    /// Durability boundary shared by sequence and commit.
    pub(super) origin_persistence: String,
    /// Restart rule for the next origin sequence.
    pub(super) origin_restart: String,
    /// Overflow result; sequence values never wrap.
    pub(super) origin_overflow: String,
    /// Tuple that owns a partition commit sequence space.
    pub(super) partition_scope: String,
    /// Partition sequence width in bits.
    pub(super) partition_width_bits: u8,
    /// First valid partition commit sequence.
    pub(super) partition_initial: u64,
    /// Commit boundary at which the partition sequence advances.
    pub(super) partition_assignment: String,
    /// Rule for failed transactions and sequence gaps.
    pub(super) partition_gaps: String,
    /// Overflow result for partition sequence values.
    pub(super) partition_overflow: String,
    /// Full identity of one persisted synchronization cursor.
    pub(super) cursor_key: String,
    /// Cursor kinds and their allowed progression order.
    pub(super) cursor_kinds: Vec<String>,
    /// Meaning of a cursor value.
    pub(super) cursor_value: String,
    /// Initial cursor value before any event is confirmed.
    pub(super) cursor_initial: u64,
    /// Single-step contiguous advancement rule.
    pub(super) cursor_advance: String,
    /// Effect of a duplicate event on a cursor.
    pub(super) duplicate_effect: String,
    /// Effect of a missing sequence on a cursor.
    pub(super) gap_effect: String,
}

/// Canonical on-disk names and atomic replacement steps.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct Filesystem {
    /// Relative root selected by the runtime configuration.
    pub(super) data_root: String,
    /// Atomic metadata file containing format and node identity.
    pub(super) format_meta: String,
    /// Single-process ownership lock.
    pub(super) lock_file: String,
    /// Partition directory template after identifier digesting.
    pub(super) partition_directory: String,
    /// Segment filename template keyed by first sequence.
    pub(super) segment_pattern: String,
    /// Snapshot filename template keyed by commit sequence.
    pub(super) snapshot_pattern: String,
    /// Cursor filename template keyed by peer identity.
    pub(super) cursor_pattern: String,
    /// Quarantine filename template keyed by event identity.
    pub(super) quarantine_pattern: String,
    /// Same-filesystem temporary directory for atomic writes.
    pub(super) temporary_directory: String,
    /// Temporary suffix that is never treated as committed data.
    pub(super) temporary_suffix: String,
    /// Rules rejecting unsafe caller-supplied path fragments.
    pub(super) path_input_rule: String,
    /// Ordered write, sync, rename, and parent-sync operations.
    pub(super) atomic_write_steps: Vec<String>,
}

/// Fixed frame-header byte offsets and checksum scope.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct FrameLayout {
    /// Offset of the four-byte magic.
    pub(super) magic_offset: u8,
    /// Width of the magic field.
    pub(super) magic_width: u8,
    /// Offset of the format version.
    pub(super) version_offset: u8,
    /// Width of the format version.
    pub(super) version_width: u8,
    /// Offset of the frame type.
    pub(super) type_offset: u8,
    /// Offset of flags; all currently defined flags are zero.
    pub(super) flags_offset: u8,
    /// Offset of the payload length.
    pub(super) payload_length_offset: u8,
    /// Width of the payload length.
    pub(super) payload_length_width: u8,
    /// Offset of the sequence value.
    pub(super) sequence_offset: u8,
    /// Width of the sequence value.
    pub(super) sequence_width: u8,
    /// Offset of the CRC32C field.
    pub(super) checksum_offset: u8,
    /// Width of the CRC32C field.
    pub(super) checksum_width: u8,
    /// Offset of reserved bytes.
    pub(super) reserved_offset: u8,
    /// Width of reserved bytes.
    pub(super) reserved_width: u8,
    /// Required value for all reserved bytes.
    pub(super) reserved_value: u32,
    /// Exact byte range covered by the checksum.
    pub(super) checksum_coverage: String,
    /// Failure behavior for unknown frame types.
    pub(super) unknown_type: String,
    /// Failure behavior for nonzero flags.
    pub(super) nonzero_flags: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FrameTypes {
    /// Numeric identifier for the transaction-begin frame.
    pub(super) begin: u8,
    /// Numeric identifier for an operation frame.
    pub(super) operation: u8,
    /// Numeric identifier for the transaction-commit frame.
    pub(super) commit: u8,
    /// Numeric identifier for a replicated change event.
    pub(super) change_event: u8,
    /// Numeric identifier for a synchronization cursor update.
    pub(super) cursor: u8,
    /// Numeric identifier for a snapshot boundary marker.
    pub(super) snapshot_marker: u8,
}

/// Persistence explanations for each externally visible synchronization state.
///
/// These descriptions are validated before runtime code can emit an
/// acknowledgement. The runtime must not claim a state until its corresponding
/// durable boundary has completed.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct Durability {
    /// Durable boundary required before local commit is reported.
    pub(super) local_committed: String,
    /// Durable boundary required before a hub reports receipt.
    pub(super) hub_received: String,
    /// Durable boundary required before a hub reports application.
    pub(super) hub_applied: String,
}

/// One allowed synchronization-state transition.
///
/// The `(from, to)` pair is unique and must belong to the frozen five-edge
/// state graph. The explanatory fields make the trigger, persistence boundary,
/// and contiguous-cursor effect reviewable before implementation begins.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct TransitionRule {
    /// State from which this transition starts.
    pub(super) from: String,
    /// State reached after the transition succeeds.
    pub(super) to: String,
    /// Event or condition that permits the transition.
    pub(super) trigger: String,
    /// Data that must be durable before the new state is visible.
    pub(super) persisted: String,
    /// How the contiguous cursor changes for this transition.
    pub(super) cursor_effect: String,
}

/// Versioned collection of deterministic fault-injection cases.
///
/// The validator requires the complete frozen set, so a document cannot pass by
/// omitting a difficult failure mode. Each case is design data for a later
/// fault-injection test and does not execute an injection at this stage.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct FaultMatrix {
    /// Schema revision for this matrix document.
    pub(super) schema_version: String,
    /// Complete list of required fault cases.
    pub(super) cases: Vec<FaultCase>,
}

/// One reproducible fault condition and its externally visible result.
///
/// Text fields identify the injection point, precondition, input, durable
/// artifacts, retry action, cursor effect, and isolation boundary. Error and
/// outcome values are cross-checked against the frozen specification so later
/// runtime tests use one vocabulary and cannot silently invent a result.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct FaultCase {
    /// Stable identifier used by acceptance reports and fault tests.
    pub(super) id: String,
    /// One of the required fault categories.
    pub(super) category: String,
    /// Boundary at which the fault is injected.
    pub(super) injection_point: String,
    /// State that must exist before the injection.
    pub(super) precondition: String,
    /// Deterministic input supplied to the failing operation.
    pub(super) input: String,
    /// Declared result after recovery or rejection.
    pub(super) expected_outcome: String,
    /// Records that must remain durable after the fault.
    pub(super) durable_artifacts: String,
    /// Declared error code, or empty when no error is returned.
    pub(super) error_code: String,
    /// Retry, repair, or operator action allowed after the result.
    pub(super) retry_action: String,
    /// Deterministic effect on the relevant contiguous cursor.
    pub(super) cursor_effect: String,
    /// Smallest state scope isolated by this case.
    pub(super) isolation_scope: String,
}
