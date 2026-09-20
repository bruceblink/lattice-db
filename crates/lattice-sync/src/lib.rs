//! Synchronization state-machine boundary for Lattice DB.

/// Monotonic synchronization milestones visible to callers.
///
/// `LocalCommitted` means the source has a durable commit, `HubReceived` means
/// the hub has durably stored the event and deduplication record, and
/// `HubApplied` means current state plus the apply cursor are durable. The enum
/// deliberately contains no transient network state, so it cannot imply that
/// an ACK was sent merely because a connection exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncState {
    LocalCommitted,
    HubReceived,
    HubApplied,
}

/// Identifies a rejected edge in the monotonic synchronization graph.
///
/// Keeping both endpoints lets callers report the exact invalid transition;
/// no state is mutated by validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    /// State observed before the attempted transition.
    pub from: SyncState,
    /// State requested by the caller.
    pub to: SyncState,
}

/// Returns whether a synchronization status transition is allowed.
///
/// The relation is a constant-size membership test, so it is O(1) time and
/// O(1) space. Self-transitions are allowed for idempotent retries; forward
/// edges are allowed only in the order local -> received -> applied.
pub fn is_valid_transition(from: SyncState, to: SyncState) -> bool {
    matches!(
        (from, to),
        (SyncState::LocalCommitted, SyncState::LocalCommitted)
            | (SyncState::LocalCommitted, SyncState::HubReceived)
            | (SyncState::HubReceived, SyncState::HubReceived)
            | (SyncState::HubReceived, SyncState::HubApplied)
            | (SyncState::HubApplied, SyncState::HubApplied)
    )
}

/// Validates a supplied transition list for the stage 0 executable.
///
/// The scan stops at the first invalid edge and returns it unchanged. This is
/// O(N) time and O(1) space, and it does not partially apply any transition.
pub fn validate_transitions(
    transitions: &[(SyncState, SyncState)],
) -> Result<(), InvalidTransition> {
    for &(from, to) in transitions {
        if !is_valid_transition(from, to) {
            return Err(InvalidTransition { from, to });
        }
    }
    Ok(())
}

/// Checks the complete five-edge status path required by stage 0.
///
/// The fixed list covers three idempotent self-edges and two forward edges;
/// omitting one would leave a legal ACK/retry behavior undocumented.
pub fn validate_stage_zero() -> Result<(), InvalidTransition> {
    validate_transitions(&[
        (SyncState::LocalCommitted, SyncState::LocalCommitted),
        (SyncState::LocalCommitted, SyncState::HubReceived),
        (SyncState::HubReceived, SyncState::HubReceived),
        (SyncState::HubReceived, SyncState::HubApplied),
        (SyncState::HubApplied, SyncState::HubApplied),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_stage_zero_transitions() {
        assert!(validate_stage_zero().is_ok());
    }

    #[test]
    fn rejects_transition_that_skips_received_state() {
        let invalid = (SyncState::LocalCommitted, SyncState::HubApplied);
        assert_eq!(
            validate_transitions(&[invalid]),
            Err(InvalidTransition {
                from: invalid.0,
                to: invalid.1,
            })
        );
    }

    #[test]
    fn rejects_backward_transition() {
        let invalid = (SyncState::HubApplied, SyncState::LocalCommitted);
        assert!(!is_valid_transition(invalid.0, invalid.1));
    }
}
