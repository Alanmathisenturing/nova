use nova_event_bus::{Event, EventBus};
use nova_glasswing::{authorize, Action, Permit};
use nova_provenance::Witness;
use nova_replay::replay;
use nova_state::{transition, State};
use nova_types::{EventId, LogicalTime, Provenance, StateRoot};

#[derive(Debug)]
pub struct Runtime {
    state: State,
    history: Vec<Event>,
    bus: EventBus,
    pub witnesses: Vec<Witness>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            state: State::default(),
            history: Vec::new(),
            bus: EventBus::default(),
            witnesses: Vec::new(),
        }
    }
}

impl Runtime {
    pub fn submit(&mut self, event: Event) -> Result<StateRoot, String> {
        event.validate().map_err(|e| format!("event validation: {e:?}"))?;
        self.bus.publish(event);
        let e = self.bus.pop().ok_or("empty bus")?;
        if self.history.last().map(|x| x.id >= e.id).unwrap_or(false) {
            return Err("duplicate or out-of-order event".into());
        }
        if e.parent != self.history.last().map(|x| x.id) {
            return Err("invalid parent".into());
        }
        if let Some(previous) = self.history.last() {
            if e.logical_time < previous.logical_time {
                return Err("logical time moved backwards".into());
            }
        }
        let previous = self.state.root();
        let proposal = transition(&self.state, &e).map_err(|x| format!("transition: {x:?}"))?;
        let next = proposal.next;
        let root = next.root();
        self.witnesses.push(Witness {
            event: e.id,
            previous,
            next: root,
            event_digest: e.digest,
            transition_digest: proposal.digest,
        });
        self.state = next;
        self.history.push(e);
        Ok(root)
    }

    pub fn submit_authorized(
        &mut self,
        event: Event,
        action: Action,
        permit: &Permit,
        now: u64,
    ) -> Result<StateRoot, String> {
        if action.event != event.id {
            return Err("permit action/event mismatch".into());
        }
        if action.state_root != self.state.root() {
            return Err("action is bound to a stale StateRoot".into());
        }
        authorize(permit, &action, now).map_err(|error| format!("authorization: {error:?}"))?;
        self.submit(event)
    }

    pub fn replay(&self) -> Result<StateRoot, String> {
        replay(&self.history, Some(self.state.root()))
            .map(|r| r.root)
            .map_err(|e| format!("{e:?}"))
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn history(&self) -> &[Event] {
        &self.history
    }

    pub fn increment(&mut self, id: u64, delta: i64) -> Result<StateRoot, String> {
        let parent = self.history.last().map(|e| e.id);
        self.submit(Event::increment(
            EventId(id),
            LogicalTime(id),
            parent,
            delta,
            Provenance {
                origin: "runtime".into(),
                trace_id: format!("trace-{id}"),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_glasswing::{issue, Authority, Capability};

    fn authority() -> Authority {
        Authority {
            actor: "agent-1".into(),
            capability: Capability {
                name: "increment".into(),
                scope: "counter".into(),
            },
            expires_at: Some(100),
            revoked: false,
        }
    }

    fn authorized_action(root: StateRoot, event: EventId) -> Action {
        Action {
            actor: "agent-1".into(),
            resource: "counter".into(),
            capability: "increment".into(),
            event,
            state_root: root,
        }
    }

    #[test]
    fn end_to_end_authorized_transition_and_replay() {
        let mut r = Runtime::default();
        let event = Event::increment(
            EventId(1),
            LogicalTime(1),
            None,
            4,
            Provenance {
                origin: "test".into(),
                trace_id: "authorized-1".into(),
            },
        );
        let action = authorized_action(r.state().root(), event.id);
        let permit = issue(&authority(), &action, 10).unwrap();

        let root = r.submit_authorized(event, action, &permit, 10).unwrap();

        assert_eq!(r.state().counter, 4);
        assert_eq!(r.replay().unwrap(), root);
    }

    #[test]
    fn stale_state_root_cannot_execute() {
        let mut r = Runtime::default();
        r.increment(1, 1).unwrap();

        let event = Event::increment(
            EventId(2),
            LogicalTime(2),
            Some(EventId(1)),
            4,
            Provenance {
                origin: "test".into(),
                trace_id: "stale".into(),
            },
        );
        let stale_action = authorized_action(StateRoot([0; 32]), event.id);
        let permit = issue(
            &authority(),
            &stale_action,
            10,
        )
        .unwrap();

        assert!(r.submit_authorized(event, stale_action, &permit, 10).is_err());
        assert_eq!(r.state().counter, 1);
    }

    #[test]
    fn revoked_authority_cannot_execute() {
        let mut authority = authority();
        authority.revoked = true;

        let mut r = Runtime::default();
        let event = Event::increment(
            EventId(1),
            LogicalTime(1),
            None,
            4,
            Provenance {
                origin: "test".into(),
                trace_id: "revoked".into(),
            },
        );
        let action = authorized_action(r.state().root(), event.id);

        assert!(issue(&authority, &action, 10).is_err());
        assert_eq!(r.state().counter, 0);
    }

    #[test]
    fn end_to_end_commit_and_replay() {
        let mut r = Runtime::default();
        r.increment(1, 4).unwrap();
        r.increment(2, -1).unwrap();
        assert_eq!(r.state().counter, 3);
        assert_eq!(r.replay().unwrap(), r.state().root());
    }

    #[test]
    fn tampered_history_is_rejected() {
        let mut r = Runtime::default();
        r.increment(1, 4).unwrap();
        r.history[0].payload[0] ^= 1;
        assert!(r.replay().is_err());
    }

    #[test]
    fn logical_time_cannot_move_backwards() {
        let mut r = Runtime::default();
        r.increment(1, 4).unwrap();
        let event = Event::increment(
            EventId(2), LogicalTime(0), Some(EventId(1)), 1,
            Provenance { origin: "test".into(), trace_id: "time".into() },
        );
        assert!(r.submit(event).is_err());
        assert_eq!(r.state().counter, 4);
    }
}

fn main() {
    let mut r = Runtime::default();
    let root = r.increment(1, 1).expect("commit");
    println!(
        "NOVA M0 PASS: root={root}, replay={}",
        r.replay().expect("replay")
    );
}
