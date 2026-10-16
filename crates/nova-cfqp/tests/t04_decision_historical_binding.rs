use nova_cfqp::{types::*, InformationBoundary};

#[test]
fn t04_decision_bound_to_information_set() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let set1 = InformationSet::new(boundary, vec![e1.clone()]).unwrap();
    let set2_e = Evidence::new(EvidenceId(2), 1000, 1500, vec![4, 5, 6]);
    let set2 = InformationSet::new(boundary, vec![e1, set2_e]).unwrap();

    let d1 = Decision::new(DecisionId(1), 100, set1, vec![7, 8, 9]);
    let d2 = Decision::new(DecisionId(1), 100, set2, vec![7, 8, 9]);

    assert_ne!(
        d1.digest, d2.digest,
        "decision bound to different information set must have different digest"
    );
}

#[test]
fn t04_decision_bound_to_boundary() {
    let boundary1 = InformationBoundary::new(2000);
    let boundary2 = InformationBoundary::new(2001);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    
    let set1 = InformationSet::new(boundary1, vec![e1.clone()]).unwrap();
    let set2 = InformationSet::new(boundary2, vec![e1]).unwrap();

    let d1 = Decision::new(DecisionId(1), 100, set1, vec![7, 8, 9]);
    let d2 = Decision::new(DecisionId(1), 100, set2, vec![7, 8, 9]);

    assert_ne!(
        d1.digest, d2.digest,
        "decision under different boundary must have different digest"
    );
}
