use nova_adjudication::{AdjudicationRecord, AgencyId};
use nova_agency::{transition as agency_transition, AgencyState, AgencyStatus};
use nova_evidence::Evidence;
use nova_glasswing::{authorize, issue, Action, Authority, Capability};
use nova_institution::{InstitutionalEvent, InstitutionalState};
use nova_state::{transition as state_transition, State};
use nova_types::{Digest, StateRoot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error { Evidence, Binding, Agency, Policy, State }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultState { pub state: InstitutionalState, pub events: usize }

pub fn verify(initial: AgencyState, history: &[InstitutionalEvent], expected: StateRoot) -> std::result::Result<ResultState, Error> {
    let mut agency = initial;
    let mut execution = State::default();
    let mut root = root_of(&agency, &execution);
    for item in history {
        match item {
            InstitutionalEvent::Adjudicated { evidence, claim, case_, verdict } => {
                evidence.validate().map_err(|_| Error::Evidence)?;
                if claim.id != case_.claim || claim.issuer != agency.id || claim.prior_state_root != root
                    || case_.prior_state_root != root || verdict.prior_state_root != root
                    || verdict.case_id != case_.id || verdict.evidence != case_.evidence
                    || !case_.evidence.contains(&evidence.digest) { return Err(Error::Binding); }
                let record = AdjudicationRecord::seal(case_, verdict);
                let (next, _) = agency_transition(&agency, agency.root(), agency.id, record.root, verdict.classification)
                    .map_err(|_| Error::Agency)?;
                agency = next;
                root = root_of(&agency, &execution);
            }
            InstitutionalEvent::Executed { event, action, permit } => {
                event.validate().map_err(|_| Error::State)?;
                if action.state_root != root || action.event != event.id { return Err(Error::Binding); }
                let authority = authority_for(&agency);
                let expected_permit = issue(&authority, action, agency.epoch).map_err(|_| Error::Policy)?;
                if expected_permit != *permit { return Err(Error::Policy); }
                authorize(permit, action, agency.epoch).map_err(|_| Error::Policy)?;
                execution = state_transition(&execution, event).map_err(|_| Error::State)?.next;
                root = root_of(&agency, &execution);
            }
        }
    }
    if root != expected { return Err(Error::Binding); }
    Ok(ResultState { state: InstitutionalState { agency, execution, root }, events: history.len() })
}

fn authority_for(agency: &AgencyState) -> Authority {
    Authority {
        actor: format!("agency:{}", agency.id.0),
        capability: Capability { name: "counter.increment".into(), scope: "counter".into() },
        expires_at: Some(agency.epoch + 100),
        revoked: !matches!(agency.status, AgencyStatus::Active | AgencyStatus::Probation),
    }
}

fn root_of(agency: &AgencyState, execution: &State) -> StateRoot {
    let mut b = Vec::new();
    b.extend_from_slice(&agency.root().0);
    b.extend_from_slice(&execution.root().0);
    StateRoot(Digest::of(b"nova.institutional_state.v1", &b).0)
}

#[allow(dead_code)]
fn _evidence_type(_: &Evidence) {}
