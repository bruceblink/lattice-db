//! Synchronization state-machine boundary for Lattice DB.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncState {
    LocalCommitted,
    HubReceived,
    HubApplied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    pub from: SyncState,
    pub to: SyncState,
}

/// Returns whether a synchronization status transition is allowed.
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

/// Checks the monotonic status path required by the initial synchronization contract.
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
