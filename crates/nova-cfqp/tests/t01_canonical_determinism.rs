use nova_cfqp::{canonical::Canonical, types::*, InformationBoundary};
use nova_types::Digest;

#[test]
fn t01_canonical_determinism_evidence() {
    let e1 = Evidence::new(
        EvidenceId(1),
        1000,
        1000,
        vec![1, 2, 3],
    );

    let e2 = Evidence::new(
        EvidenceId(1),
        1000,
        1000,
        vec![1, 2, 3],
    );

    assert_eq!(e1.digest, e2.digest, "same evidence must have same digest");
}

#[test]
fn t01_canonical_determinism_information_set() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(2), 1000, 1500, vec![4, 5, 6]);

    let set1 = InformationSet::new(boundary, vec![e1.clone(), e2.clone()]).unwrap();
    let set2 = InformationSet::new(boundary, vec![e1.clone(), e2.clone()]).unwrap();

    assert_eq!(set1.set_digest, set2.set_digest, "identical sets must have same digest");
}

#[test]
fn t01_canonical_determinism_decision() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let set = InformationSet::new(boundary, vec![e1]).unwrap();

    let d1 = Decision::new(DecisionId(1), 100, set.clone(), vec![7, 8, 9]);
    let d2 = Decision::new(DecisionId(1), 100, set.clone(), vec![7, 8, 9]);

    assert_eq!(d1.digest, d2.digest, "identical decisions must have same digest");
}

#[test]
fn t01_canonical_determinism_outcome() {
    let o1 = Outcome::new(OutcomeId(1), DecisionId(1), 3000, vec![10, 11]);
    let o2 = Outcome::new(OutcomeId(1), DecisionId(1), 3000, vec![10, 11]);

    assert_eq!(o1.digest, o2.digest, "identical outcomes must have same digest");
}
