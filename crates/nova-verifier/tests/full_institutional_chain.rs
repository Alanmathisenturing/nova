use nova_adjudication::{AdjudicationCase, AgencyId, Claim, ClaimId, Dissent, Verdict, VerdictClass};
use nova_agency::{AgencyState, AgencyStatus};
use nova_evidence::Evidence;
use nova_glasswing::{issue, Action};
use nova_institution::{InstitutionalEvent, InstitutionalRuntime};
use nova_types::{Digest, EventId, LogicalTime, Provenance, StateRoot};
use nova_verifier::institutional::verify;

fn initial() -> AgencyState {
    AgencyState { id: AgencyId(7), epoch: 0, status: AgencyStatus::Probation,
        qualification_root: Digest::of(b"q", b"1"), adjudication_root: None, prior_state_root: StateRoot([0;32]) }
}

#[test]
fn full_institutional_chain_matches_runtime_root() {
    let mut runtime = InstitutionalRuntime::new(initial());
    let evidence = Evidence::new(1, LogicalTime(1), LogicalTime(2), b"bounded execution".to_vec(),
        Provenance { origin: "test".into(), trace_id: "e1".into() });
    let root = runtime.state().root;
    let claim = Claim { id: ClaimId(1), subject: "agency:7".into(), predicate: "qualified_for".into(),
        object: "counter.increment".into(), evidence: vec![evidence.digest], issuer: AgencyId(7),
        created_at: LogicalTime(1), prior_state_root: root };
    let case_ = AdjudicationCase { id: nova_adjudication::CaseId(1), claim: claim.id,
        evidence: claim.evidence.clone(), evaluator: AgencyId(7), adjudicator: AgencyId(7),
        opened_at: LogicalTime(2), prior_state_root: root };
    let verdict = Verdict { case_id: case_.id, classification: VerdictClass::Supported,
        adjudicator: AgencyId(7), evidence: case_.evidence.clone(), reasoning_digest: Digest::of(b"r", b"1"),
        dissents: vec![Dissent { author: AgencyId(9), digest: Digest::of(b"d", b"1") }], prior_state_root: root };
    runtime.adjudicate(evidence, claim, case_, verdict).unwrap();

    let event = nova_event_bus::Event::increment(EventId(1), LogicalTime(3), None, 7,
        Provenance { origin: "test".into(), trace_id: "x".into() });
    let action = Action { actor: "agency:7".into(), resource: "counter".into(),
        capability: "counter.increment".into(), event: event.id, state_root: runtime.state().root };
    let permit = issue(&runtime.authority(), &action, runtime.state().agency.epoch).unwrap();
    runtime.execute(event, action, permit, runtime.state().agency.epoch).unwrap();

    let verified = verify(initial(), runtime.history(), runtime.state().root).unwrap();
    assert_eq!(verified.events, runtime.history().len());
    assert_eq!(verified.state.root, runtime.state().root);
    assert_eq!(verified.state.execution.counter, 7);
    assert_eq!(verified.state.agency.status, AgencyStatus::Active);
}

#[test]
fn full_chain_rejects_tampered_permit() {
    let mut runtime = InstitutionalRuntime::new(initial());
    let evidence = Evidence::new(1, LogicalTime(1), LogicalTime(2), b"x".to_vec(),
        Provenance { origin: "test".into(), trace_id: "e".into() });
    let root = runtime.state().root;
    let claim = Claim { id: ClaimId(1), subject: "agency:7".into(), predicate: "qualified_for".into(),
        object: "counter.increment".into(), evidence: vec![evidence.digest], issuer: AgencyId(7),
        created_at: LogicalTime(1), prior_state_root: root };
    let case_ = AdjudicationCase { id: nova_adjudication::CaseId(1), claim: claim.id,
        evidence: claim.evidence.clone(), evaluator: AgencyId(7), adjudicator: AgencyId(7),
        opened_at: LogicalTime(2), prior_state_root: root };
    let verdict = Verdict { case_id: case_.id, classification: VerdictClass::Supported,
        adjudicator: AgencyId(7), evidence: case_.evidence.clone(), reasoning_digest: Digest::of(b"r", b"1"),
        dissents: vec![], prior_state_root: root };
    runtime.adjudicate(evidence, claim, case_, verdict).unwrap();
    let event = nova_event_bus::Event::increment(EventId(1), LogicalTime(3), None, 1,
        Provenance { origin: "test".into(), trace_id: "x".into() });
    let action = Action { actor: "agency:7".into(), resource: "counter".into(),
        capability: "counter.increment".into(), event: event.id, state_root: runtime.state().root };
    let permit = issue(&runtime.authority(), &action, runtime.state().agency.epoch).unwrap();
    runtime.execute(event, action, permit, runtime.state().agency.epoch).unwrap();
    let mut history = runtime.history().to_vec();
    if let InstitutionalEvent::Executed { permit, .. } = &mut history[1] {
        permit.action_digest = Digest::of(b"tampered", b"permit");
    }
    assert!(verify(initial(), &history, runtime.state().root).is_err());
}
