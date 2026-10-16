use nova_cfqp::{types::*, InformationBoundary};

#[test]
fn t02_evidence_content_mutation() {
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 4]); // different content

    assert_ne!(e1.digest, e2.digest, "different content must have different digest");
}

#[test]
fn t02_evidence_availability_time_mutation() {
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(1), 1000, 1001, vec![1, 2, 3]); // different availability_at

    assert_ne!(e1.digest, e2.digest, "different availability time must change digest");
}

#[test]
fn t02_information_set_membership_mutation() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(2), 1000, 1500, vec![4, 5, 6]);

    let set1 = InformationSet::new(boundary, vec![e1.clone()]).unwrap();
    let set2 = InformationSet::new(boundary, vec![e1.clone(), e2]).unwrap();

    assert_ne!(set1.set_digest, set2.set_digest, "adding evidence must change set digest");
}

#[test]
fn t02_decision_action_mutation() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let set = InformationSet::new(boundary, vec![e1]).unwrap();

    let d1 = Decision::new(DecisionId(1), 100, set.clone(), vec![7, 8, 9]);
    let d2 = Decision::new(DecisionId(1), 100, set, vec![7, 8, 10]); // different action

    assert_ne!(d1.digest, d2.digest, "different action must change decision digest");
}

#[test]
fn t02_decision_actor_mutation() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let set = InformationSet::new(boundary, vec![e1]).unwrap();

    let d1 = Decision::new(DecisionId(1), 100, set.clone(), vec![7, 8, 9]);
    let d2 = Decision::new(DecisionId(1), 101, set, vec![7, 8, 9]); // different actor

    assert_ne!(d1.digest, d2.digest, "different actor must change decision digest");
}

#[test]
fn t02_outcome_result_mutation() {
    let o1 = Outcome::new(OutcomeId(1), DecisionId(1), 3000, vec![10, 11]);
    let o2 = Outcome::new(OutcomeId(1), DecisionId(1), 3000, vec![10, 12]); // different result

    assert_ne!(o1.digest, o2.digest, "different outcome result must change digest");
}
