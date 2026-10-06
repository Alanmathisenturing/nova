# NOVA Ω Governor Execution Order

This document is an execution ledger, not a completion claim.

## Order

1. Reality / Evidence Governor
2. Event / State Governor
3. Epistemic Governor
4. Decision Governor
5. Authority / Capability Governor
6. Execution Governor
7. Adjudication Governor
8. Agency Governor
9. Institutional Runtime Governor
10. Independent Verification / Replay Governor
11. Economic / Capital Governor
12. Workforce Compiler Governor
13. Autonomous Implementation Governor
14. Meta-Governor

## Rule

Each layer may advance only after the previous layer has a concrete artifact, invariant, executable test, and observed execution result. Until CI or another authoritative execution surface produces the result, the implementation state remains IMPLEMENTED_UNTESTED.

## Current execution evidence

- Evidence primitive: implemented with temporal, provenance and integrity validation.
- State primitive: event validation and monotonic event-id enforcement added.
- Runtime: logical-time ordering rejection added.
- Epistemic primitive: evidence digest binds source/content; belief evidence requirements added.
- Decision primitive: nova-decision added with StateRoot, evidence, expiry and uncertainty validation.
- Authority/Execution: existing Glasswing and runtime integration present.
- Adjudication/Agency: existing deterministic transition primitives present.
- Institutional Runtime: claim identity is now explicitly bound into adjudication history and replay.
- Economic: nova-economic capital state and root-bound reserve/release transitions added.
- Workforce: nova-workforce deterministic role-to-agent compilation added.

## Verification boundary

GitHub Actions run 37417642598 for commit 733665c03ed93d5ddb3ff0ad1315a97713b09c2a completed successfully. Its test job observed success for cargo fmt --all, cargo check --workspace, cargo test --workspace, and cargo test --workspace --release.

This is TESTED evidence. No layer is promoted to VERIFIED merely from this CI result because independent reconstruction is still a separate requirement.

The next mandatory transition is execution evidence: cargo fmt --all -- --check, cargo check --workspace, cargo test --workspace, and release tests must actually run and produce observable results.