use nova_types::Digest;

#[test]
fn t06_digest_domain_separation() {
    let data = b"test data";

    let evidence_digest = Digest::of(b"cfqp/evidence/v1", data);
    let decision_digest = Digest::of(b"cfqp/decision/v1", data);
    let outcome_digest = Digest::of(b"cfqp/outcome/v1", data);

    assert_ne!(
        evidence_digest, decision_digest,
        "same data in different domains must have different digests"
    );
    assert_ne!(
        decision_digest, outcome_digest,
        "same data in different domains must have different digests"
    );
    assert_ne!(
        evidence_digest, outcome_digest,
        "same data in different domains must have different digests"
    );
}

#[test]
fn t06_digest_version_separation() {
    let data = b"test data";

    let v1_digest = Digest::of(b"cfqp/evidence/v1", data);
    let v2_digest = Digest::of(b"cfqp/evidence/v2", data);

    assert_ne!(
        v1_digest, v2_digest,
        "version difference must change digest even with same data"
    );
}
