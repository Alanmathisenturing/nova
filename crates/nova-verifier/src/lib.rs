use nova_event_bus::{Event, EventKind};
use nova_types::{Digest, EventId, StateRoot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    EmptyHistory,
    InvalidEvent { index: usize },
    NonMonotonicEvent { index: usize },
    InvalidParent { index: usize },
    InvalidPayload { index: usize },
    Overflow { index: usize },
    RootMismatch { expected: StateRoot, actual: StateRoot },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationResult {
    pub events: usize,
    pub counter: i64,
    pub root: StateRoot,
}

/// Independently reconstructs the canonical state root from the event history.
///
/// This intentionally does not call nova_state::transition or nova_replay::replay.
/// The state transition semantics are reconstructed here from the public event
/// contract so a verifier failure can expose a shared runtime/replay defect.
pub fn verify_history(
    events: &[Event],
    expected: Option<StateRoot>,
) -> Result<VerificationResult, VerificationError> {
    if events.is_empty() {
        let root = state_root(0, None);
        if let Some(expected_root) = expected {
            if expected_root != root {
                return Err(VerificationError::RootMismatch { expected: expected_root, actual: root });
            }
        }
        return Ok(VerificationResult { events: 0, counter: 0, root });
    }

    let mut counter = 0i64;
    let mut last: Option<EventId> = None;

    for (index, event) in events.iter().enumerate() {
        event.validate().map_err(|_| VerificationError::InvalidEvent { index })?;

        if last.map(|previous| event.id <= previous).unwrap_or(false) {
            return Err(VerificationError::NonMonotonicEvent { index });
        }

        if event.parent != last {
            return Err(VerificationError::InvalidParent { index });
        }

        match event.kind {
            EventKind::Increment => {
                if event.payload.len() != 8 {
                    return Err(VerificationError::InvalidPayload { index });
                }
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&event.payload);
                let delta = i64::from_le_bytes(bytes);
                counter = counter.checked_add(delta)
                    .ok_or(VerificationError::Overflow { index })?;
            }
            EventKind::Observation => {}
        }

        last = Some(event.id);
    }

    let root = state_root(counter, last);
    if let Some(expected_root) = expected {
        if expected_root != root {
            return Err(VerificationError::RootMismatch { expected: expected_root, actual: root });
        }
    }

    Ok(VerificationResult { events: events.len(), counter, root })
}

fn state_root(counter: i64, last_event: Option<EventId>) -> StateRoot {
    let mut bytes = Vec::with_capacity(16);
    bytes.extend_from_slice(&counter.to_le_bytes());
    bytes.extend_from_slice(&last_event.map(|id| id.0).unwrap_or(0).to_le_bytes());
    StateRoot(Digest::of(b"nova.state.v1", &bytes).0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_types::{LogicalTime, Provenance};

    fn provenance() -> Provenance {
        Provenance { origin: "verifier-test".into(), trace_id: "trace".into() }
    }

    fn history() -> Vec<Event> {
        vec![
            Event::increment(EventId(1), LogicalTime(1), None, 4, provenance()),
            Event::increment(EventId(2), LogicalTime(2), Some(EventId(1)), -1, provenance()),
        ]
    }

    #[test]
    fn independently_reconstructs_runtime_root() {
        let events = history();
        let result = verify_history(&events, None).unwrap();
        assert_eq!(result.counter, 3);
        assert_eq!(result.events, 2);
        assert_eq!(result.root, nova_state_root(&events));
    }

    #[test]
    fn expected_root_is_checked() {
        let events = history();
        let root = verify_history(&events, None).unwrap().root;
        assert!(verify_history(&events, Some(root)).is_ok());
        assert!(matches!(
            verify_history(&events, Some(StateRoot([0; 32]))),
            Err(VerificationError::RootMismatch { .. })
        ));
    }

    #[test]
    fn event_tampering_fails() {
        let mut events = history();
        events[1].payload[0] ^= 1;
        assert!(matches!(
            verify_history(&events, None),
            Err(VerificationError::InvalidEvent { index: 1 })
        ));
    }

    #[test]
    fn reordering_fails() {
        let mut events = history();
        events.swap(0, 1);
        assert!(matches!(
            verify_history(&events, None),
            Err(VerificationError::NonMonotonicEvent { .. } | VerificationError::InvalidParent { .. })
        ));
    }

    #[test]
    fn duplicate_fails() {
        let mut events = history();
        events.push(events[1].clone());
        assert!(matches!(
            verify_history(&events, None),
            Err(VerificationError::NonMonotonicEvent { index: 2 })
        ));
    }

    fn nova_state_root(events: &[Event]) -> StateRoot {
        let mut counter = 0i64;
        let mut last = None;
        for event in events {
            if event.kind == EventKind::Increment {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&event.payload);
                counter += i64::from_le_bytes(bytes);
            }
            last = Some(event.id);
        }
        state_root(counter, last)
    }
}
