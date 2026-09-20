use nova_event_bus::Event;
use nova_state::{transition, State};
use nova_types::StateRoot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayResult {
    pub applied: usize,
    pub root: StateRoot,
    pub expected: Option<StateRoot>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReplayError {
    Event(String),
    IntegrityViolation { event_index: usize, event_id: nova_types::EventId },
    Transition(String),
    RootMismatch { expected: StateRoot, actual: StateRoot },
}

pub fn replay(events: &[Event], expected: Option<StateRoot>) -> Result<ReplayResult, ReplayError> {
    let mut state = State::default();
    let mut last = None;
    for (index, event) in events.iter().enumerate() {
        match event.validate() {
            Ok(()) => {}
            Err(nova_event_bus::EventError::IntegrityViolation) => {
                return Err(ReplayError::IntegrityViolation { event_index: index, event_id: event.id });
            }
            Err(error) => return Err(ReplayError::Event(format!("{error:?}"))),
        }
        if last.map(|previous| previous >= event.id).unwrap_or(false) {
            return Err(ReplayError::Event("non-monotonic event id".into()));
        }
        if event.parent != last {
            return Err(ReplayError::Event("invalid parent".into()));
        }
        state = transition(&state, event).map_err(|error| ReplayError::Transition(format!("{error:?}")))?.next;
        last = Some(event.id);
    }

    let root = state.root();
    if let Some(expected_root) = expected {
        if expected_root != root { return Err(ReplayError::RootMismatch { expected: expected_root, actual: root }); }
    }
    Ok(ReplayResult { applied: events.len(), root, expected })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_types::{EventId, LogicalTime, Provenance};

    fn history() -> Vec<Event> {
        let provenance = Provenance { origin: "test".into(), trace_id: "trace".into() };
        vec![
            Event::increment(EventId(1), LogicalTime(1), None, 2, provenance.clone()),
            Event::increment(EventId(2), LogicalTime(2), Some(EventId(1)), 3, provenance),
        ]
    }

    #[test]
    fn same_history_same_root() { assert_eq!(replay(&history(), None).unwrap().root, replay(&history(), None).unwrap().root); }

    #[test]
    fn tampering_is_detected_before_transition() {
        let mut events = history();
        events[1].payload[0] ^= 1;
        assert!(matches!(replay(&events, None), Err(ReplayError::IntegrityViolation { event_index: 1, .. })));
    }

    #[test]
    fn expected_root_is_verified() {
        let events = history();
        let root = replay(&events, None).unwrap().root;
        assert!(replay(&events, Some(root)).is_ok());
        assert!(matches!(replay(&events, Some(StateRoot([0; 32]))), Err(ReplayError::RootMismatch { .. })));
    }
}
