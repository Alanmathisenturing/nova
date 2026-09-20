use nova_types::{Digest, EventId};
#[derive(Clone, Debug, Eq, PartialEq)] pub struct Capability { pub name:String, pub scope:String }
#[derive(Clone, Debug, Eq, PartialEq)] pub struct Action { pub actor:String, pub resource:String, pub capability:String, pub event:EventId }
#[derive(Clone, Debug, Eq, PartialEq)] pub struct Permit { pub action_digest:Digest, pub capability:String }
#[derive(Debug, Clone, Eq, PartialEq)] pub enum PolicyError { Denied, ScopeMismatch }
pub fn issue(cap:&Capability, action:&Action)->Result<Permit,PolicyError>{ if cap.name!=action.capability{return Err(PolicyError::Denied)} if cap.scope!="*" && cap.scope!=action.resource{return Err(PolicyError::ScopeMismatch)} let mut b=Vec::new();b.extend_from_slice(action.actor.as_bytes());b.extend_from_slice(action.resource.as_bytes());b.extend_from_slice(&action.event.0.to_le_bytes());Ok(Permit{action_digest:Digest::of(b"nova.action.v1",&b),capability:cap.name.clone()}) }
#[cfg(test)] mod tests {use super::*; #[test]fn denied_capability_never_gets_permit(){let a=Action{actor:"a".into(),resource:"r".into(),capability:"write".into(),event:EventId(1)};assert_eq!(issue(&Capability{name:"read".into(),scope:"*".into()},&a),Err(PolicyError::Denied));}}
