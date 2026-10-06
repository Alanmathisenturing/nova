use nova_cfqp::{types::*, InformationBoundary, CfqpError};

#[test]
fn t03_boundary_admits_evidence_before_boundary() {
    let boundary = InformationBoundary::new(2000);
    assert!(boundary.admits(1000), "evidence available before boundary must be admitted");
}

#[test]
fn t03_boundary_admits_evidence_at_boundary() {
    let boundary = InformationBoundary::new(2000);
    assert!(boundary.admits(2000), "evidence available at boundary must be admitted");
}

#[test]
fn t03_boundary_rejects_future_evidence() {
    let boundary = InformationBoundary::new(2000);
    assert!(!boundary.admits(2001), "evidence available after boundary must be rejected");
}

#[test]
fn t03_information_set_rejects_future_evidence() {
    let boundary = InformationBoundary::new(2000);
    let e_future = Evidence::new(EvidenceId(1), 1000, 2001, vec![1, 2, 3]); // available_at > boundary

    let result = InformationSet::new(boundary, vec![e_future]);
    assert!(
        result.is_err(),
        "information set must reject evidence beyond boundary"
    );
}

#[test]
fn t03_information_set_accepts_boundary_evidence() {
    let boundary = InformationBoundary::new(2000);
    let e1 = Evidence::new(EvidenceId(1), 1000, 1000, vec![1, 2, 3]);
    let e2 = Evidence::new(EvidenceId(2), 1000, 2000, vec![4, 5, 6]); // available_at == boundary

    let result = InformationSet::new(boundary, vec![e1, e2]);
    assert!(
        result.is_ok(),
        "information set must accept evidence at boundary"
    );
}
