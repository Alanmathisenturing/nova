use nova_adjudication::{AdjudicationCase, AdjudicationRecord, AgencyId, Claim, Verdict, VerdictClass};
use nova_agency::{transition as agency_transition, AgencyState, AgencyStatus};
use nova_evidence::{Evidence, EvidenceError};
use nova_event_bus::{Event, EventError};
use nova_glasswing::{issue, Action, Authority, Capability, Permit, PolicyError};
use nova_state::{transition as state_transition, State};
use nova_types::{Digest, StateRoot};
use nova_persistence::{Journal, JournalError};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstitutionalState { pub agency: AgencyState, pub execution: State, pub root: StateRoot }
impl InstitutionalState {
    pub fn new(agency:AgencyState)->Self { let execution=State::default(); let root=root_of(&agency,&execution); Self{agency,execution,root} }
    fn refresh(&mut self){self.root=root_of(&self.agency,&self.execution);}
}
fn root_of(a:&AgencyState,e:&State)->StateRoot{let mut b=Vec::new();b.extend_from_slice(&a.root().0);b.extend_from_slice(&e.root().0);StateRoot(Digest::of(b"nova.institutional_state.v1",&b).0)}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstitutionalEvent {
    Adjudicated { evidence:Evidence, claim:Claim, case_:AdjudicationCase, verdict:Verdict },
    Executed { event:Event, action:Action, permit:Permit, now:u64 },
}
impl InstitutionalEvent {
    fn digest(&self)->Digest{
        let mut b=Vec::new();
        match self {
            Self::Adjudicated{evidence,claim,case_,verdict}=>{b.push(1);b.extend_from_slice(&evidence.digest.0);b.extend_from_slice(&claim.digest().0);b.extend_from_slice(&case_.digest().0);b.extend_from_slice(&verdict.digest().0);}
            Self::Executed{event,action,permit,now}=>{b.push(2);b.extend_from_slice(&event.digest.0);b.extend_from_slice(&action.digest().0);b.extend_from_slice(&permit.action_digest.0);b.extend_from_slice(&now.to_le_bytes());}
        }
        Digest::of(b"nova.institutional_event.v1",&b)
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstitutionalWitness { pub event_digest:Digest, pub prior_root:StateRoot, pub next_root:StateRoot }
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstitutionError { Evidence(EvidenceError), Event(EventError), Agency, Policy(PolicyError), Persistence(String), InvalidBinding, StaleStateRoot, Replay }
impl From<EvidenceError> for InstitutionError {fn from(e:EvidenceError)->Self{Self::Evidence(e)}}
impl From<EventError> for InstitutionError {fn from(e:EventError)->Self{Self::Event(e)}}

pub struct InstitutionalRuntime { genesis:AgencyState, state:InstitutionalState, history:Vec<InstitutionalEvent>, witnesses:Vec<InstitutionalWitness> }
impl InstitutionalRuntime {
 pub fn new(agency:AgencyState)->Self{Self{genesis:agency.clone(),state:InstitutionalState::new(agency),history:Vec::new(),witnesses:Vec::new()}}
 pub fn state(&self)->&InstitutionalState{&self.state}
 pub fn history(&self)->&[InstitutionalEvent]{&self.history}
 pub fn witnesses(&self)->&[InstitutionalWitness]{&self.witnesses}
 pub fn authority(&self)->Authority{Authority{actor:format!("agency:{}",self.state.agency.id.0),capability:Capability{name:"counter.increment".into(),scope:"counter".into()},expires_at:Some(self.state.agency.epoch+100),revoked:!matches!(self.state.agency.status,AgencyStatus::Active|AgencyStatus::Probation)}}
 pub fn adjudicate(&mut self,evidence:Evidence,claim:Claim,case_:AdjudicationCase,verdict:Verdict)->Result<StateRoot,InstitutionError>{
  evidence.validate()?; if claim.id!=case_.claim||claim.issuer!=self.state.agency.id||claim.prior_state_root!=self.state.root{return Err(InstitutionError::InvalidBinding)} if case_.prior_state_root!=self.state.root||verdict.prior_state_root!=self.state.root||verdict.case_id!=case_.id||verdict.evidence!=case_.evidence||!case_.evidence.contains(&evidence.digest){return Err(InstitutionError::InvalidBinding)}
  let record=AdjudicationRecord::seal(&case_,&verdict); let prior=self.state.root;
  let (agency,_)=agency_transition(&self.state.agency,self.state.agency.root(),self.state.agency.id,record.root,verdict.classification).map_err(|_|InstitutionError::Agency)?;
  self.state.agency=agency;self.state.refresh();self.history.push(InstitutionalEvent::Adjudicated{evidence,claim,case_,verdict});
  self.witnesses.push(InstitutionalWitness{event_digest:self.history.last().unwrap().digest(),prior_root:prior,next_root:self.state.root});Ok(self.state.root)
 }
 pub fn execute(&mut self,event:Event,action:Action,permit:Permit,now:u64)->Result<StateRoot,InstitutionError>{
  event.validate()?;
  if action.state_root!=self.state.root||action.event!=event.id{return Err(InstitutionError::StaleStateRoot)}
  let expected=issue(&self.authority(),&action,now).map_err(InstitutionError::Policy)?; if expected!=permit{return Err(InstitutionError::Policy(PolicyError::Denied))}
  nova_glasswing::authorize(&permit,&action,now).map_err(InstitutionError::Policy)?;
  let proposal=state_transition(&self.state.execution,&event).map_err(|_|InstitutionError::Agency)?;let prior=self.state.root;
  self.state.execution=proposal.next;self.state.refresh();self.history.push(InstitutionalEvent::Executed{event,action,permit,now});
  self.witnesses.push(InstitutionalWitness{event_digest:self.history.last().unwrap().digest(),prior_root:prior,next_root:self.state.root});Ok(self.state.root)
 }
 pub fn execute_persisted(&mut self,event:Event,action:Action,permit:Permit,now:u64,journal:&mut Journal)->Result<StateRoot,InstitutionError>{
  event.validate()?;
  if action.state_root!=self.state.root||action.event!=event.id{return Err(InstitutionError::StaleStateRoot)}
  let expected=issue(&self.authority(),&action,now).map_err(InstitutionError::Policy)?;
  if expected!=permit{return Err(InstitutionError::Policy(PolicyError::Denied))}
  nova_glasswing::authorize(&permit,&action,now).map_err(InstitutionError::Policy)?;
  let proposal=state_transition(&self.state.execution,&event).map_err(|_|InstitutionError::Agency)?;
  // Persist before committing memory state; append failure leaves the state unchanged.
  journal.append(&event).map_err(|e:JournalError|InstitutionError::Persistence(format!("{e:?}")))?;
  let prior=self.state.root;
  self.state.execution=proposal.next;
  self.state.refresh();
  self.history.push(InstitutionalEvent::Executed{event,action,permit,now});
  self.witnesses.push(InstitutionalWitness{event_digest:self.history.last().unwrap().digest(),prior_root:prior,next_root:self.state.root});
  Ok(self.state.root)
 }
 pub fn open_execution_journal(path:impl AsRef<Path>)->Result<(Journal,Vec<Event>),InstitutionError>{
  Journal::open(path).map_err(|e|InstitutionError::Persistence(format!("{e:?}")))
 }
 pub fn replay(&self)->Result<StateRoot,InstitutionError>{
  if self.history.len()!=self.witnesses.len(){return Err(InstitutionError::Replay)}
  let mut agency=self.genesis.clone();
  let mut exec=State::default();
  let mut root=root_of(&agency,&exec);
  for (index,item) in self.history.iter().enumerate() {
   let witness=&self.witnesses[index];
   if witness.prior_root!=root||witness.event_digest!=item.digest(){return Err(InstitutionError::Replay)}
   match item {
    InstitutionalEvent::Adjudicated{evidence,claim,case_,verdict}=>{
     evidence.validate()?;
     if claim.id!=case_.claim||claim.issuer!=agency.id||claim.prior_state_root!=root{return Err(InstitutionError::Replay)}
     if case_.prior_state_root!=root||verdict.prior_state_root!=root||verdict.case_id!=case_.id||verdict.evidence!=case_.evidence||!case_.evidence.contains(&evidence.digest){return Err(InstitutionError::Replay)}
     let sealed=AdjudicationRecord::seal(case_,verdict);
     let (next,_)=agency_transition(&agency,agency.root(),agency.id,sealed.root,verdict.classification).map_err(|_|InstitutionError::Replay)?;
     agency=next;
     root=root_of(&agency,&exec);
    }
    InstitutionalEvent::Executed{event,action,permit,now}=>{
     event.validate().map_err(|_|InstitutionError::Replay)?;
     if action.state_root!=root||action.event!=event.id{return Err(InstitutionError::Replay)}
     let authority=Authority{actor:format!("agency:{}",agency.id.0),capability:Capability{name:"counter.increment".into(),scope:"counter".into()},expires_at:Some(agency.epoch+100),revoked:!matches!(agency.status,AgencyStatus::Active|AgencyStatus::Probation)};
     let expected=issue(&authority,action,*now).map_err(|_|InstitutionError::Replay)?;
     if expected!=*permit{return Err(InstitutionError::Replay)}
     nova_glasswing::authorize(permit,action,*now).map_err(|_|InstitutionError::Replay)?;
     exec=state_transition(&exec,event).map_err(|_|InstitutionError::Replay)?.next;
     root=root_of(&agency,&exec);
    }
   }
   if witness.next_root!=root{return Err(InstitutionError::Replay)}
  }
  if root!=self.state.root{return Err(InstitutionError::Replay)}
  Ok(root)
 }
}

#[cfg(test)]
mod tests{
 use super::*;use nova_adjudication::{CaseId,Claim,ClaimId,Dissent};use nova_types::{EventId,LogicalTime,Provenance};
 fn initial()->AgencyState{AgencyState{id:AgencyId(7),epoch:0,status:AgencyStatus::Probation,qualification_root:Digest::of(b"q",b"1"),adjudication_root:None,prior_state_root:StateRoot([0;32])}}
 fn evidence()->Evidence{Evidence::new(1,LogicalTime(1),LogicalTime(2),b"bounded execution".to_vec(),Provenance{origin:"test".into(),trace_id:"e1".into()})}
 fn verdict(root:StateRoot,e:&Evidence,c:VerdictClass)->(AdjudicationCase,Verdict){let claim=Claim{id:ClaimId(1),subject:"agency:7".into(),predicate:"qualified_for".into(),object:"counter.increment".into(),evidence:vec![e.digest],issuer:AgencyId(7),created_at:LogicalTime(1),prior_state_root:root};let case_=AdjudicationCase{id:CaseId(1),claim:claim.id,evidence:claim.evidence.clone(),evaluator:AgencyId(7),adjudicator:AgencyId(7),opened_at:LogicalTime(2),prior_state_root:root};let v=Verdict{case_id:case_.id,classification:c,adjudicator:AgencyId(7),evidence:case_.evidence.clone(),reasoning_digest:Digest::of(b"r",b"1"),dissents:vec![Dissent{author:AgencyId(9),digest:Digest::of(b"d",b"1")}],prior_state_root:root};(case_,v)}
 #[test]fn full_chain_and_replay(){let mut r=InstitutionalRuntime::new(initial());let e=evidence();let (c,v)=verdict(r.state().root,&e,VerdictClass::Supported);let claim=Claim{id:ClaimId(1),subject:"agency:7".into(),predicate:"qualified_for".into(),object:"counter.increment".into(),evidence:vec![e.digest],issuer:AgencyId(7),created_at:LogicalTime(1),prior_state_root:r.state().root};let root=r.adjudicate(e,claim,c,v).unwrap();assert_eq!(r.state().agency.status,AgencyStatus::Active);let event=Event::increment(EventId(1),LogicalTime(3),None,7,Provenance{origin:"test".into(),trace_id:"x".into()});let action=Action{actor:"agency:7".into(),resource:"counter".into(),capability:"counter.increment".into(),event:event.id,state_root:root};let permit=issue(&r.authority(),&action,r.state().agency.epoch).unwrap();let final_root=r.execute(event,action,permit,r.state().agency.epoch).unwrap();assert_eq!(r.state().execution.counter,7);assert_eq!(r.replay().unwrap(),final_root);}
 #[test]fn contradiction_revokes_authority(){let mut r=InstitutionalRuntime::new(initial());let e=evidence();let (c,v)=verdict(r.state().root,&e,VerdictClass::Contradicted);let claim=Claim{id:ClaimId(1),subject:"agency:7".into(),predicate:"qualified_for".into(),object:"counter.increment".into(),evidence:vec![e.digest],issuer:AgencyId(7),created_at:LogicalTime(1),prior_state_root:r.state().root};r.adjudicate(e,claim,c,v).unwrap();assert_eq!(r.state().agency.status,AgencyStatus::Suspended);let event=Event::increment(EventId(1),LogicalTime(3),None,1,Provenance{origin:"test".into(),trace_id:"x".into()});let a=Action{actor:"agency:7".into(),resource:"counter".into(),capability:"counter.increment".into(),event:event.id,state_root:r.state().root};assert!(issue(&r.authority(),&a,1).is_err());}
 #[test]fn stale_state_is_rejected(){let mut r=InstitutionalRuntime::new(initial());let e=evidence();let (c,v)=verdict(r.state().root,&e,VerdictClass::Supported);let claim=Claim{id:ClaimId(1),subject:"agency:7".into(),predicate:"qualified_for".into(),object:"counter.increment".into(),evidence:vec![e.digest],issuer:AgencyId(7),created_at:LogicalTime(1),prior_state_root:r.state().root};r.adjudicate(e,claim,c,v).unwrap();let event=Event::increment(EventId(1),LogicalTime(3),None,1,Provenance{origin:"test".into(),trace_id:"x".into()});let a=Action{actor:"agency:7".into(),resource:"counter".into(),capability:"counter.increment".into(),event:event.id,state_root:StateRoot([0;32])};let p=issue(&r.authority(),&a,1).unwrap();assert!(r.execute(event,a,p,1).is_err());}
 #[test]fn persisted_execution_recovers_and_replays(){
  use std::time::{SystemTime,UNIX_EPOCH};
  let path=std::env::temp_dir().join(format!("nova-institution-{}.journal",SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
  let mut r=InstitutionalRuntime::new(initial());
  let e=evidence();let (case_,v)=verdict(r.state().root,&e,VerdictClass::Supported);
  let claim=Claim{id:ClaimId(1),subject:"agency:7".into(),predicate:"qualified_for".into(),object:"counter.increment".into(),evidence:vec![e.digest],issuer:AgencyId(7),created_at:LogicalTime(1),prior_state_root:r.state().root};
  let root=r.adjudicate(e,claim,case_,v).unwrap();
  let event=Event::increment(EventId(1),LogicalTime(3),None,7,Provenance{origin:"test".into(),trace_id:"persisted".into()});
  let action=Action{actor:"agency:7".into(),resource:"counter".into(),capability:"counter.increment".into(),event:event.id,state_root:root};
  let permit=issue(&r.authority(),&action,r.state().agency.epoch).unwrap();
  let (mut journal,old)=Journal::open(&path).unwrap();assert!(old.is_empty());
  let final_root=r.execute_persisted(event,action,permit,r.state().agency.epoch,&mut journal).unwrap();
  drop(journal);
  let (_reopened,recovered)=Journal::open(&path).unwrap();
  assert_eq!(recovered.len(),1);
  let replayed=nova_replay::replay(&recovered,None).unwrap();
  assert_eq!(replayed.root,r.state().execution.root());
  assert_eq!(r.replay().unwrap(),final_root);
  std::fs::remove_file(path).unwrap();
 }

}
