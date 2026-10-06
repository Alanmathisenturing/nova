use nova_types::{Digest, LogicalTime, Provenance};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    pub id: u64,
    pub observed_at: LogicalTime,
    pub available_at: LogicalTime,
    pub content: Vec<u8>,
    pub provenance: Provenance,
    pub digest: Digest,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EvidenceError { EmptyContent, InvalidTemporalOrder, InvalidProvenance, IntegrityViolation }

impl Evidence {
    pub fn new(id:u64, observed_at:LogicalTime, available_at:LogicalTime, content:Vec<u8>, provenance:Provenance)->Self {
        let mut e=Self{id,observed_at,available_at,content,provenance,digest:Digest([0;32])};
        e.digest=e.compute_digest(); e
    }
    pub fn compute_digest(&self)->Digest {
        let mut b=Vec::new();
        b.extend_from_slice(&self.id.to_le_bytes());
        b.extend_from_slice(&self.observed_at.0.to_le_bytes());
        b.extend_from_slice(&self.available_at.0.to_le_bytes());
        b.extend_from_slice(&(self.content.len() as u64).to_le_bytes());
        b.extend_from_slice(&self.content);
        b.extend_from_slice(&(self.provenance.origin.len() as u64).to_le_bytes());
        b.extend_from_slice(self.provenance.origin.as_bytes());
        b.extend_from_slice(&(self.provenance.trace_id.len() as u64).to_le_bytes());
        b.extend_from_slice(self.provenance.trace_id.as_bytes());
        Digest::of(b"nova.evidence.v1",&b)
    }
    pub fn validate(&self)->Result<(),EvidenceError>{
        if self.content.is_empty(){return Err(EvidenceError::EmptyContent)}
        if self.available_at < self.observed_at{return Err(EvidenceError::InvalidTemporalOrder)}
        if self.provenance.origin.is_empty()||self.provenance.trace_id.is_empty(){return Err(EvidenceError::InvalidProvenance)}
        if self.digest!=self.compute_digest(){return Err(EvidenceError::IntegrityViolation)}
        Ok(())
    }
}
#[cfg(test)]
mod tests{
 use super::*;
 fn p()->Provenance{Provenance{origin:"test".into(),trace_id:"e1".into()}}
 #[test]fn digest_binds_time(){let a=Evidence::new(1,LogicalTime(2),LogicalTime(3),b"a".to_vec(),p());let b=Evidence::new(1,LogicalTime(4),LogicalTime(5),b"a".to_vec(),p());assert_ne!(a.digest,b.digest)}
 #[test]fn tamper_rejected(){let mut e=Evidence::new(1,LogicalTime(2),LogicalTime(3),b"a".to_vec(),p());e.content[0]^=1;assert_eq!(e.validate(),Err(EvidenceError::IntegrityViolation))}
 #[test]fn invalid_temporal_order_rejected(){let e=Evidence::new(1,LogicalTime(3),LogicalTime(2),b"a".to_vec(),p());assert_eq!(e.validate(),Err(EvidenceError::InvalidTemporalOrder))}
}
