use nova_cfqp::{types::*, InformationBoundary};

#[test]
fn t07_information_set_membership_semantics() {
    // CFQP Information Set uses membership semantics (unordered)
    // but canonical commitment is deterministic via sorting by ID

    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(2), 1000, 1500, vec![4, 5, 6]);

    // Same evidence, different input order
    let set_ab = InformationSet::new(boundary, vec![e1.clone(), e2.clone()]).unwrap();
    let set_ba = InformationSet::new(boundary, vec![e2.clone(), e1.clone()]).unwrap();

    assert_eq!(
        set_ab.set_digest, set_ba.set_digest,
        "membership order should not affect canonical commitment"
    );
    assert_eq!(
        set_ab.contains(EvidenceId(1)),
        set_ba.contains(EvidenceId(1)),
        "both sets contain same members"
    );
}

#[test]
fn t07_information_set_deterministic_canonical() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(3), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(1), 1000, 1500, vec![4, 5, 6]);
    let e3 = Evidence::new(EvidenceId(2), 1000, 1600, vec![7, 8, 9]);

    // Multiple construction orderings
    let set1 = InformationSet::new(boundary, vec![e1.clone(), e2.clone(), e3.clone()]).unwrap();
    let set2 = InformationSet::new(boundary, vec![e3.clone(), e1.clone(), e2.clone()]).unwrap();
    let set3 = InformationSet::new(boundary, vec![e2.clone(), e3.clone(), e1.clone()]).unwrap();

    assert_eq!(
        set1.set_digest, set2.set_digest,
        "canonical commitment must be independent of input order"
    );
    assert_eq!(
        set2.set_digest, set3.set_digest,
        "canonical commitment must be independent of input order"
    );
}
