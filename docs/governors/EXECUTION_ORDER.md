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

The current GitHub Actions query for the PR head has returned no workflow run. Therefore no layer is promoted to VERIFIED by this ledger.

The next mandatory transition is execution evidence: cargo fmt --all -- --check, cargo check --workspace, cargo test --workspace, and release tests must actually run and produce observable results.