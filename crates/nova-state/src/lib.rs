use nova_event_bus::{Event, EventKind};
use nova_types::{Digest, EventId, StateRoot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub counter: i64,
    pub last_event: Option<EventId>,
}

impl Default for State {
    fn default() -> Self { Self { counter: 0, last_event: None } }
}

impl State {
    pub fn root(&self) -> StateRoot {
        let mut b = Vec::new();
        b.extend_from_slice(&self.counter.to_le_bytes());
        b.extend_from_slice(&self.last_event.map(|x| x.0).unwrap_or(0).to_le_bytes());
        StateRoot(Digest::of(b"nova.state.v1", &b).0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal { pub next: State, pub event: EventId, pub digest: Digest }

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StateError { InvalidEvent, UnsupportedEvent, InvalidPayload, Overflow }

pub fn transition(previous: &State, event: &Event) -> Result<Proposal, StateError> {
    event.validate().map_err(|_| StateError::InvalidEvent)?;
    if let Some(last) = previous.last_event {
        if event.id <= last { return Err(StateError::InvalidEvent); }
    }
    let mut next = previous.clone();
    match event.kind {
        EventKind::Increment => {
            if event.payload.len() != 8 { return Err(StateError::InvalidPayload); }
            let d = i64::from_le_bytes(event.payload.clone().try_into().map_err(|_| StateError::InvalidPayload)?);
            next.counter = next.counter.checked_add(d).ok_or(StateError::Overflow)?;
        }
        EventKind::Observation => {}
    }
    next.last_event = Some(event.id);
    let mut b = Vec::new();
    b.extend_from_slice(&next.counter.to_le_bytes());
    b.extend_from_slice(&event.id.0.to_le_bytes());
    Ok(Proposal { next, event: event.id, digest: Digest::of(b"nova.proposal.v1", &b) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_types::{LogicalTime, Provenance};

    fn event(id: u64, delta: i64) -> Event {
        Event::increment(EventId(id), LogicalTime(id), None, delta, Provenance { origin: "x".into(), trace_id: format!("t-{id}") })
    }

    #[test]
    fn transition_is_deterministic() {
        let e = event(1, 3);
        let a = transition(&State::default(), &e).unwrap();
        let b = transition(&State::default(), &e).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.next.counter, 3);
    }

    #[test]
    fn malformed_event_cannot_mutate_state() {
        let mut e = event(1, 3);
        e.payload[0] ^= 1;
        assert_eq!(transition(&State::default(), &e), Err(StateError::InvalidEvent));
    }

    #[test]
    fn duplicate_or_reordered_event_is_rejected() {
        let first = event(2, 1);
        let state = transition(&State::default(), &first).unwrap().next;
        assert_eq!(transition(&state, &event(2, 4)), Err(StateError::InvalidEvent));
        assert_eq!(transition(&state, &event(1, 4)), Err(StateError::InvalidEvent));
    }
}
