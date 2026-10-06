use nova_event_bus::Event;
use nova_state::State;
use nova_types::{EventId, LogicalTime, Provenance};
use nova_verifier::verify_history;

fn events() -> Vec<Event> {
    vec![
        Event::increment(
            EventId(1),
            LogicalTime(1),
            None,
            4,
            Provenance { origin: "test".into(), trace_id: "one".into() },
        ),
        Event::increment(
            EventId(2),
            LogicalTime(2),
            Some(EventId(1)),
            -1,
            Provenance { origin: "test".into(), trace_id: "two".into() },
        ),
    ]
}

#[test]
fn independent_reconstruction_matches_canonical_state_root() {
    let history = events();
    let mut state = State::default();
    for event in &history {
        state = nova_state::transition(&state, event).unwrap().next;
    }

    let checked = verify_history(&history, Some(state.root())).unwrap();
    assert_eq!(checked.events, 2);
    assert_eq!(checked.counter, 3);
    assert_eq!(checked.root, state.root());
}

#[test]
fn modified_history_is_rejected() {
    let history = events();
    let expected = verify_history(&history, None).unwrap().root;
    let mut mutated = history;
    mutated[1].payload[0] ^= 1;
    assert!(verify_history(&mutated, Some(expected)).is_err());
}
