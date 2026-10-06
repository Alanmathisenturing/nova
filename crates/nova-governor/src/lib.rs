use nova_types::Digest;

#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd)]
pub enum VerificationLevel{NotImplemented,ImplementedUntested,Tested,Verified,AdversariallyVerified}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct ProofRecord{
 pub artifact:Digest,
 pub invariant:Digest,
 pub test:Digest,
 pub observed:bool,
 pub replayed:bool,
 pub adversarial:bool,
}

impl ProofRecord{pub fn level(&self)->VerificationLevel{if !self.observed{VerificationLevel::ImplementedUntested}else if !self.replayed{VerificationLevel::Tested}else if !self.adversarial{VerificationLevel::Verified}else{VerificationLevel::AdversariallyVerified}}}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Gate{pub name:String,pub level:VerificationLevel,pub proof:Option<ProofRecord>}

#[derive(Debug,Clone,Eq,PartialEq)]
pub enum GovernorError{MissingProof,InvalidPromotion,DependencyNotVerified}

pub fn promote(gate:&Gate, target:VerificationLevel)->Result<Gate,GovernorError>{
 let proof=gate.proof.clone().ok_or(GovernorError::MissingProof)?;
 let actual=proof.level();
 if actual<target{return Err(GovernorError::InvalidPromotion)}
 Ok(Gate{name:gate.name.clone(),level:actual,proof:Some(proof)})
}

#[cfg(test)]
mod tests{use super::*;fn proof()->ProofRecord{ProofRecord{artifact:Digest::of(b"a",b"1"),invariant:Digest::of(b"i",b"1"),test:Digest::of(b"t",b"1"),observed:true,replayed:false,adversarial:false}}
#[test]fn observed_tests_promote_to_tested(){let g=Gate{name:"state".into(),level:VerificationLevel::ImplementedUntested,proof:Some(proof())};assert_eq!(promote(&g,VerificationLevel::Tested).unwrap().level,VerificationLevel::Tested)}
#[test]fn verified_requires_replay(){let g=Gate{name:"state".into(),level:VerificationLevel::Tested,proof:Some(proof())};assert_eq!(promote(&g,VerificationLevel::Verified),Err(GovernorError::InvalidPromotion))}
#[test]fn adversarial_requires_all_proof_flags(){let mut p=proof();p.replayed=true;p.adversarial=true;let g=Gate{name:"state".into(),level:VerificationLevel::Tested,proof:Some(p)};assert_eq!(promote(&g,VerificationLevel::AdversariallyVerified).unwrap().level,VerificationLevel::AdversariallyVerified)} }
