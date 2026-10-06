use nova_types::{Digest, LogicalTime, StateRoot};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CaseId(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClaimId(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AgencyId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerdictClass { Supported, WeaklySupported, Contradicted, Underdetermined, Invalid, Rejected }

impl VerdictClass { fn tag(self) -> u8 { match self { Self::Supported=>0, Self::WeaklySupported=>1, Self::Contradicted=>2, Self::Underdetermined=>3, Self::Invalid=>4, Self::Rejected=>5 } } }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claim {
    pub id: ClaimId, pub subject: String, pub predicate: String, pub object: String,
    pub evidence: Vec<Digest>, pub issuer: AgencyId, pub created_at: LogicalTime,
    pub prior_state_root: StateRoot,
}
impl Claim {
    pub fn digest(&self) -> Digest {
        let mut b=Vec::new();
        b.extend_from_slice(&self.id.0.to_le_bytes());
        for s in [&self.subject,&self.predicate,&self.object] { b.extend_from_slice(&(s.len() as u64).to_le_bytes()); b.extend_from_slice(s.as_bytes()); }
        for e in &self.evidence { b.extend_from_slice(&e.0); }
        b.extend_from_slice(&self.issuer.0.to_le_bytes());
        b.extend_from_slice(&self.created_at.0.to_le_bytes());
        b.extend_from_slice(&self.prior_state_root.0);
        Digest::of(b"nova.claim.v1",&b)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjudicationCase {
    pub id: CaseId, pub claim: ClaimId, pub evidence: Vec<Digest>,
    pub evaluator: AgencyId, pub adjudicator: AgencyId, pub opened_at: LogicalTime,
    pub prior_state_root: StateRoot,
}
impl AdjudicationCase {
    pub fn digest(&self)->Digest {
        let mut b=Vec::new();
        b.extend_from_slice(&self.id.0.to_le_bytes()); b.extend_from_slice(&self.claim.0.to_le_bytes());
        for e in &self.evidence { b.extend_from_slice(&e.0); }
        b.extend_from_slice(&self.evaluator.0.to_le_bytes()); b.extend_from_slice(&self.adjudicator.0.to_le_bytes());
        b.extend_from_slice(&self.opened_at.0.to_le_bytes()); b.extend_from_slice(&self.prior_state_root.0);
        Digest::of(b"nova.adjudication_case.v1",&b)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dissent { pub author: AgencyId, pub digest: Digest }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verdict {
    pub case_id: CaseId, pub classification: VerdictClass, pub adjudicator: AgencyId,
    pub evidence: Vec<Digest>, pub reasoning_digest: Digest, pub dissents: Vec<Dissent>,
    pub prior_state_root: StateRoot,
}
impl Verdict {
    pub fn digest(&self)->Digest {
        let mut b=Vec::new();
        b.extend_from_slice(&self.case_id.0.to_le_bytes()); b.push(self.classification.tag());
        b.extend_from_slice(&self.adjudicator.0.to_le_bytes());
        for e in &self.evidence { b.extend_from_slice(&e.0); }
        b.extend_from_slice(&self.reasoning_digest.0);
        for d in &self.dissents { b.extend_from_slice(&d.author.0.to_le_bytes()); b.extend_from_slice(&d.digest.0); }
        b.extend_from_slice(&self.prior_state_root.0);
        Digest::of(b"nova.verdict.v1",&b)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdjudicationRoot(pub Digest);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjudicationRecord { pub case_digest: Digest, pub verdict_digest: Digest, pub root: AdjudicationRoot }

impl AdjudicationRecord {
    pub fn seal(case_: &AdjudicationCase, verdict: &Verdict)->Self {
        let case_digest=case_.digest(); let verdict_digest=verdict.digest();
        let mut b=Vec::with_capacity(64); b.extend_from_slice(&case_digest.0); b.extend_from_slice(&verdict_digest.0);
        Self { case_digest, verdict_digest, root: AdjudicationRoot(Digest::of(b"nova.adjudication_root.v1",&b)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn root()->StateRoot { StateRoot([7u8;32]) }
    fn claim()->Claim { Claim { id:ClaimId(1), subject:"agency:1".into(), predicate:"qualified_for".into(), object:"research".into(), evidence:vec![Digest::of(b"evidence",b"E1")], issuer:AgencyId(1), created_at:LogicalTime(10), prior_state_root:root() } }

    #[test] fn claim_digest_is_deterministic(){ assert_eq!(claim().digest(),claim().digest()); }
    #[test] fn claim_digest_binds_evidence(){ let a=claim(); let mut b=claim(); b.evidence.push(Digest::of(b"evidence",b"E2")); assert_ne!(a.digest(),b.digest()); }
    #[test] fn verdict_digest_binds_prior_state(){ let c=claim(); let case=AdjudicationCase{id:CaseId(1),claim:c.id,evidence:c.evidence.clone(),evaluator:AgencyId(2),adjudicator:AgencyId(3),opened_at:LogicalTime(11),prior_state_root:root()}; let v=Verdict{case_id:case.id,classification:VerdictClass::Supported,adjudicator:AgencyId(3),evidence:case.evidence.clone(),reasoning_digest:Digest::of(b"reasoning",b"R1"),dissents:vec![],prior_state_root:root()}; let mut changed=v.clone(); changed.prior_state_root=StateRoot([8u8;32]); assert_ne!(v.digest(),changed.digest()); }
    #[test] fn sealing_binds_case_and_verdict(){ let c=claim(); let case=AdjudicationCase{id:CaseId(1),claim:c.id,evidence:c.evidence.clone(),evaluator:AgencyId(2),adjudicator:AgencyId(3),opened_at:LogicalTime(11),prior_state_root:root()}; let v=Verdict{case_id:case.id,classification:VerdictClass::Supported,adjudicator:AgencyId(3),evidence:case.evidence.clone(),reasoning_digest:Digest::of(b"reasoning",b"R1"),dissents:vec![],prior_state_root:root()}; let r=AdjudicationRecord::seal(&case,&v); assert_eq!(r.case_digest,case.digest()); assert_eq!(r.verdict_digest,v.digest()); }
}
