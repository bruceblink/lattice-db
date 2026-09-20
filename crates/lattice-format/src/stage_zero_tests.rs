use super::*;

fn valid_spec() -> FrozenSpec {
    parse_document::<FrozenSpec>(FROZEN_SPEC, "spec").expect("test specification is valid")
}

fn valid_matrix() -> FaultMatrix {
    parse_document::<FaultMatrix>(FAULT_MATRIX, "matrix").expect("test matrix is valid")
}

#[test]
fn validates_embedded_documents_and_hashes_them() {
    let summary = validate_stage_zero_documents().expect("embedded documents are valid");
    assert_eq!(summary.spec_version, "0.1");
    assert_eq!(summary.fault_cases, 16);
    assert_eq!(summary.transition_rules, 5);
    assert_eq!(summary.spec_hash.len(), 16);
}

#[test]
fn formats_parse_and_validation_errors() {
    let parse = StageZeroError::Parse("bad toml".to_owned());
    let validation = StageZeroError::Validation("bad field".to_owned());
    assert!(parse.to_string().contains("parse"));
    assert!(validation.to_string().contains("validation"));
    assert!(parse_document::<FrozenSpec>("spec_version = [", "spec").is_err());
    assert!(parse_document::<FaultMatrix>("cases = [", "matrix").is_err());
    assert!(validate_stage_zero_documents_from("spec_version = [", FAULT_MATRIX).is_err());
    assert!(validate_stage_zero_documents_from(FROZEN_SPEC, "cases = [").is_err());
    let invalid_spec = FROZEN_SPEC.replacen("spec_version = \"0.1\"", "spec_version = \"0.2\"", 1);
    assert!(validate_stage_zero_documents_from(&invalid_spec, FAULT_MATRIX).is_err());
    let invalid_matrix =
        FAULT_MATRIX.replacen("schema_version = \"0.1\"", "schema_version = \"0.2\"", 1);
    assert!(validate_stage_zero_documents_from(FROZEN_SPEC, &invalid_matrix).is_err());
}

#[test]
fn stable_hash_changes_when_documents_change() {
    assert_ne!(stable_hash("a", "b"), stable_hash("a", "c"));
    assert_eq!(stable_hash("", ""), "af63bd4c8601b7df");
}

#[test]
fn require_and_value_lists_cover_success_and_failure() {
    assert!(require(true, "unused").is_ok());
    assert!(require(false, "missing").is_err());
    let values = vec!["one".to_owned(), "one".to_owned()];
    assert!(validate_unique_values("values", &values).is_err());
    assert!(validate_required_values("values", &values, &["missing"]).is_err());
    assert!(validate_required_values("values", &values, &["one"]).is_ok());
}

#[test]
fn rejects_each_scalar_spec_rule() {
    macro_rules! invalid {
        ($field:ident, $value:expr) => {{
            let mut spec = valid_spec();
            spec.$field = $value;
            assert!(validate_spec(&spec).is_err(), stringify!($field));
        }};
    }
    invalid!(spec_version, "0.2".to_owned());
    invalid!(protocol_version, 2);
    invalid!(file_format_version, 2);
    invalid!(encoding, "big-endian".to_owned());
    invalid!(text_encoding, "ascii".to_owned());
    invalid!(checksum, "sha256".to_owned());
    invalid!(max_record_bytes, 0);
    invalid!(max_batch_bytes, 0);
    invalid!(max_transaction_operations, 0);
    invalid!(log_segment_bytes, 0);
    invalid!(snapshot_bytes, 0);
    invalid!(pending_queue_bytes, 0);
    invalid!(node_id_bytes, 0);
    invalid!(dataset_id_max_bytes, 0);
    invalid!(partition_id_max_bytes, 0);
    invalid!(record_id_max_bytes, 0);
    invalid!(sequence_bytes, 0);
    invalid!(frame_magic, "BAD!".to_owned());
    invalid!(frame_header_bytes, 0);

    let mut spec = valid_spec();
    spec.max_batch_bytes = spec.max_record_bytes - 1;
    assert!(validate_spec(&spec).is_err());
    let mut spec = valid_spec();
    spec.log_segment_bytes = spec.max_batch_bytes - 1;
    assert!(validate_spec(&spec).is_err());
    let mut spec = valid_spec();
    spec.snapshot_bytes = spec.max_record_bytes - 1;
    assert!(validate_spec(&spec).is_err());
    let mut spec = valid_spec();
    spec.pending_queue_bytes = spec.max_batch_bytes - 1;
    assert!(validate_spec(&spec).is_err());
}

#[test]
fn propagates_nested_spec_and_matrix_errors() {
    let mut spec = valid_spec();
    spec.frame_types.begin = 2;
    assert!(validate_spec(&spec).is_err());

    let mut spec = valid_spec();
    spec.durability.local_committed.clear();
    assert!(validate_spec(&spec).is_err());

    let mut spec = valid_spec();
    spec.error_codes[0] = spec.error_codes[1].clone();
    assert!(validate_spec(&spec).is_err());

    let mut spec = valid_spec();
    spec.error_codes
        .retain(|code| code != REQUIRED_ERROR_CODES[0]);
    assert!(validate_spec(&spec).is_err());

    let mut spec = valid_spec();
    spec.event_outcomes[0] = spec.event_outcomes[1].clone();
    assert!(validate_spec(&spec).is_err());

    let mut spec = valid_spec();
    spec.event_outcomes
        .retain(|outcome| outcome != REQUIRED_OUTCOMES[0]);
    assert!(validate_spec(&spec).is_err());

    let mut spec = valid_spec();
    spec.transitions.pop();
    assert!(validate_spec(&spec).is_err());

    let spec = valid_spec();
    let mut matrix = valid_matrix();
    matrix.cases[0].expected_outcome = "unknown".to_owned();
    assert!(validate_matrix(&spec, &matrix).is_err());
}

#[test]
fn validates_frame_types_and_durability_rules() {
    let valid = FrameTypes {
        begin: 1,
        operation: 2,
        commit: 3,
        change_event: 4,
        cursor: 5,
        snapshot_marker: 6,
    };
    assert!(validate_frame_types(&valid).is_ok());
    let invalid_frames = [
        FrameTypes { begin: 2, ..valid },
        FrameTypes {
            operation: 7,
            ..valid
        },
        FrameTypes { commit: 7, ..valid },
        FrameTypes {
            change_event: 7,
            ..valid
        },
        FrameTypes { cursor: 7, ..valid },
        FrameTypes {
            snapshot_marker: 7,
            ..valid
        },
        FrameTypes {
            operation: 1,
            ..valid
        },
    ];
    for frame_types in invalid_frames {
        assert!(validate_frame_types(&frame_types).is_err());
    }

    let valid = Durability {
        local_committed: "local".to_owned(),
        hub_received: "received".to_owned(),
        hub_applied: "applied".to_owned(),
    };
    assert!(validate_durability(&valid).is_ok());
    assert!(
        validate_durability(&Durability {
            local_committed: String::new(),
            ..valid.clone()
        })
        .is_err()
    );
    assert!(
        validate_durability(&Durability {
            hub_received: String::new(),
            ..valid.clone()
        })
        .is_err()
    );
    assert!(
        validate_durability(&Durability {
            hub_applied: String::new(),
            ..valid
        })
        .is_err()
    );
}

#[test]
fn rejects_invalid_transition_rules() {
    let mut spec = valid_spec();
    spec.transitions.pop();
    assert!(validate_transitions(&spec.transitions).is_err());

    let mut spec = valid_spec();
    spec.transitions[0].trigger.clear();
    assert!(validate_transitions(&spec.transitions).is_err());
    let mut spec = valid_spec();
    spec.transitions[0].persisted.clear();
    assert!(validate_transitions(&spec.transitions).is_err());
    let mut spec = valid_spec();
    spec.transitions[0].cursor_effect.clear();
    assert!(validate_transitions(&spec.transitions).is_err());

    let mut spec = valid_spec();
    spec.transitions[1].from = spec.transitions[0].from.clone();
    spec.transitions[1].to = spec.transitions[0].to.clone();
    assert!(validate_transitions(&spec.transitions).is_err());
    let mut spec = valid_spec();
    spec.transitions[0].from = "unknown".to_owned();
    assert!(validate_transitions(&spec.transitions).is_err());
}

#[test]
fn rejects_invalid_fault_matrix_structure() {
    let spec = valid_spec();
    let mut matrix = valid_matrix();
    matrix.schema_version = "0.2".to_owned();
    assert!(validate_matrix(&spec, &matrix).is_err());
    let mut matrix = valid_matrix();
    matrix.cases.pop();
    assert!(validate_matrix(&spec, &matrix).is_err());
    let mut matrix = valid_matrix();
    matrix.cases[1].id = matrix.cases[0].id.clone();
    assert!(validate_matrix(&spec, &matrix).is_err());
    let mut matrix = valid_matrix();
    matrix.cases[0].id = "F-999".to_owned();
    assert!(validate_matrix(&spec, &matrix).is_err());
    let mut matrix = valid_matrix();
    for case in &mut matrix.cases {
        if case.category == "network" {
            case.category = "capacity".to_owned();
        }
    }
    assert!(validate_matrix(&spec, &matrix).is_err());
}

#[test]
fn rejects_invalid_fault_case_fields_and_references() {
    let spec = valid_spec();
    let mut base = valid_matrix().cases[0].clone();
    macro_rules! empty_field {
        ($field:ident) => {{
            let mut case = base.clone();
            case.$field.clear();
            assert!(
                validate_fault_case(
                    &case,
                    &spec.error_codes.iter().map(String::as_str).collect(),
                    &spec.event_outcomes.iter().map(String::as_str).collect(),
                )
                .is_err()
            );
        }};
    }
    empty_field!(category);
    empty_field!(injection_point);
    empty_field!(precondition);
    empty_field!(input);
    empty_field!(expected_outcome);
    empty_field!(durable_artifacts);
    empty_field!(retry_action);
    empty_field!(cursor_effect);
    empty_field!(isolation_scope);

    base.expected_outcome = "unknown".to_owned();
    let outcomes = spec.event_outcomes.iter().map(String::as_str).collect();
    let errors = spec.error_codes.iter().map(String::as_str).collect();
    assert!(validate_fault_case(&base, &errors, &outcomes).is_err());
    base.expected_outcome = "accepted".to_owned();
    base.error_code = "E_UNKNOWN".to_owned();
    assert!(validate_fault_case(&base, &errors, &outcomes).is_err());
}
