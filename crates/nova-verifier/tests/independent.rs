use nova_runtime::Runtime;
use nova_verifier::verify_history;

#[test]
fn root_matches_independent_reconstruction() {
    let mut r = Runtime::default();
    r.increment(1, 4).unwrap();
    r.increment(2, -1).unwrap();
    let root = r.state().root();
    let checked = verify_history(r.history(), Some(root)).unwrap();
    assert_eq!(checked.events, 2);
    assert_eq!(checked.counter, 3);
    assert_eq!(checked.root, root);
}

#[test]
fn modified_history_is_rejected() {
    let mut r = Runtime::default();
    r.increment(1, 4).unwrap();
    r.increment(2, -1).unwrap();
    let root = r.state().root();
    let mut history = r.history().to_vec();
    history[1].payload[0] ^= 1;
    assert!(verify_history(&history, Some(root)).is_err());
}
