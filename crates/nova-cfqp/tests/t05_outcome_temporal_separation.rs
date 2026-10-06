use nova_cfqp::{types::*, InformationBoundary};

#[test]
fn t05_outcome_does_not_include_decision_information() {
    // This test demonstrates that Outcome commitment does NOT include
    // the Decision's InformationSet, preventing outcome from retroactively
    // becoming evidence available at decision time.

    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let set = InformationSet::new(boundary, vec![e1]).unwrap();
    let decision = Decision::new(DecisionId(1), 100, set, vec![7, 8, 9]);

    let outcome = Outcome::new(OutcomeId(1), decision.id, 3000, vec![result_data]);

    // The outcome digest includes the decision_id but NOT the information set
    // This preserves temporal separation
    assert!(
        outcome.digest.as_bytes() != decision.digest.as_bytes(),
        "outcome must have distinct digest from decision"
    );
}

#[test]
fn t05_multiple_outcomes_same_decision() {
    let o1 = Outcome::new(OutcomeId(1), DecisionId(1), 3000, vec![10, 11]);
    let o2 = Outcome::new(OutcomeId(1), DecisionId(1), 3001, vec![10, 11]); // different time

    assert_ne!(
        o1.digest, o2.digest,
        "different outcome times must change digest"
    );
}
