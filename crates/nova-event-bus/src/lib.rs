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
        let mut bytes = Vec::new(); bytes.extend_from_slice(&id.0.to_le_bytes()); bytes.push(match kind { EventKind::Increment => 1, EventKind::Observation => 2 }); bytes.extend_from_slice(&1u16.to_le_bytes()); bytes.extend_from_slice(&time.0.to_le_bytes()); bytes.extend_from_slice(&parent.map(|p| p.0).unwrap_or(0).to_le_bytes()); bytes.extend_from_slice(&payload); bytes.extend_from_slice(provenance.origin.as_bytes()); bytes.push(0); bytes.extend_from_slice(provenance.trace_id.as_bytes());
        let digest = Digest::of(b"nova.event.v1", &bytes);
        Self { id, kind, schema: SchemaVersion(1), logical_time: time, parent, payload, provenance, digest }
    }
    pub fn increment(id: EventId, time: LogicalTime, parent: Option<EventId>, delta: i64, provenance: Provenance) -> Self { Self::new(id, EventKind::Increment, time, parent, delta.to_le_bytes().to_vec(), provenance) }
    pub fn validate(&self) -> Result<(), EventError> { if self.schema.0 != 1 { return Err(EventError::UnknownSchema(self.schema.0)); } if self.kind == EventKind::Increment && self.payload.len() != 8 { return Err(EventError::MalformedPayload); } if self.provenance.origin.is_empty() { return Err(EventError::InvalidOrigin); } Ok(()) }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventError { UnknownSchema(u16), MalformedPayload, InvalidOrigin }

#[derive(Default)]
pub struct EventBus { queue: VecDeque<Event> }
impl EventBus { pub fn publish(&mut self, event: Event) { self.queue.push_back(event); } pub fn pop(&mut self) -> Option<Event> { self.queue.pop_front() } pub fn len(&self) -> usize { self.queue.len() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn digest_changes_with_payload() { let p=Provenance{origin:"test".into(),trace_id:"t".into()}; assert_ne!(Event::increment(EventId(1),LogicalTime(1),None,1,p.clone()).digest, Event::increment(EventId(1),LogicalTime(1),None,2,p).digest); } #[test] fn malformed_increment_rejected() { let e=Event::new(EventId(1),EventKind::Increment,LogicalTime(1),None,vec![1],Provenance{origin:"x".into(),trace_id:"t".into()}); assert_eq!(e.validate(),Err(EventError::MalformedPayload)); } }
