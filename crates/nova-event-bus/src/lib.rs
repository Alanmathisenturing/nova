use nova_types::{Digest, EventId, LogicalTime, Provenance, SchemaVersion};
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventKind { Increment, Observation }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub kind: EventKind,
    pub schema: SchemaVersion,
    pub logical_time: LogicalTime,
    pub parent: Option<EventId>,
    pub payload: Vec<u8>,
    pub provenance: Provenance,
    pub digest: Digest,
}

impl Event {
    pub fn new(id: EventId, kind: EventKind, time: LogicalTime, parent: Option<EventId>, payload: Vec<u8>, provenance: Provenance) -> Self {
        let mut event = Self {
            id, kind, schema: SchemaVersion(1), logical_time: time, parent, payload, provenance,
            digest: Digest([0; 32]),
        };
        event.digest = event.compute_digest();
        event
    }

    pub fn increment(id: EventId, time: LogicalTime, parent: Option<EventId>, delta: i64, provenance: Provenance) -> Self {
        Self::new(id, EventKind::Increment, time, parent, delta.to_le_bytes().to_vec(), provenance)
    }

    /// Computes the digest over every field that affects event semantics.
    pub fn compute_digest(&self) -> Digest {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.0.to_le_bytes());
        bytes.push(match self.kind { EventKind::Increment => 1, EventKind::Observation => 2 });
        bytes.extend_from_slice(&self.schema.0.to_le_bytes());
        bytes.extend_from_slice(&self.logical_time.0.to_le_bytes());
        match self.parent { Some(parent) => { bytes.push(1); bytes.extend_from_slice(&parent.0.to_le_bytes()); }, None => bytes.push(0) }
        bytes.extend_from_slice(&(self.payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&(self.provenance.origin.len() as u64).to_le_bytes());
        bytes.extend_from_slice(self.provenance.origin.as_bytes());
        bytes.extend_from_slice(&(self.provenance.trace_id.len() as u64).to_le_bytes());
        bytes.extend_from_slice(self.provenance.trace_id.as_bytes());
        Digest::of(b"nova.event.v1", &bytes)
    }

    pub fn validate(&self) -> Result<(), EventError> {
        if self.schema.0 != 1 { return Err(EventError::UnknownSchema(self.schema.0)); }
        if self.provenance.origin.is_empty() { return Err(EventError::InvalidOrigin); }
        if self.provenance.trace_id.is_empty() { return Err(EventError::InvalidTraceId); }
        if self.kind == EventKind::Increment && self.payload.len() != 8 { return Err(EventError::MalformedPayload); }
        if self.digest != self.compute_digest() { return Err(EventError::IntegrityViolation); }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventError { UnknownSchema(u16), MalformedPayload, InvalidOrigin, InvalidTraceId, IntegrityViolation }

#[derive(Default)]
pub struct EventBus { queue: VecDeque<Event> }
impl EventBus {
    pub fn publish(&mut self, event: Event) { self.queue.push_back(event); }
    pub fn pop(&mut self) -> Option<Event> { self.queue.pop_front() }
    pub fn len(&self) -> usize { self.queue.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn provenance() -> Provenance { Provenance { origin: "test".into(), trace_id: "trace".into() } }

    #[test]
    fn digest_changes_with_payload() {
        assert_ne!(Event::increment(EventId(1), LogicalTime(1), None, 1, provenance()).digest, Event::increment(EventId(1), LogicalTime(1), None, 2, provenance()).digest);
    }

    #[test]
    fn tampered_payload_is_rejected() {
        let mut event = Event::increment(EventId(1), LogicalTime(1), None, 1, provenance());
        event.payload[0] ^= 1;
        assert_eq!(event.validate(), Err(EventError::IntegrityViolation));
    }

    #[test]
    fn tampered_metadata_is_rejected() {
        let mut event = Event::increment(EventId(1), LogicalTime(1), None, 1, provenance());
        event.logical_time = LogicalTime(2);
        assert_eq!(event.validate(), Err(EventError::IntegrityViolation));
    }
}
