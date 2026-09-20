# NOVA

NOVA is a Rust-first epistemic execution runtime built around deterministic state, evidence, policy enforcement, and replayable outcomes.

## Current status

This repository is being built as a real Rust workspace. The first milestone implements the deterministic runtime slice:

- observation -> event
- event bus dispatch
- state transition
- epistemic update
- proposal + policy + capability enforcement
- execution permit
- result + commit + state root
- replay verification

## Workspace structure

- `crates/nova-types` — strong domain identifiers and canonical hashing
- `crates/nova-event-bus` — event transport and acknowledgement semantics
- `crates/nova-state` — state transitions and roots
- `crates/nova-epistemic` — beliefs, hypotheses, predictions, outcomes
- `crates/nova-glasswing` — capability and policy enforcement
- `crates/nova-replay` — deterministic replay and divergence detection
- `crates/nova-runtime` — end-to-end vertical slice / runtime orchestration

## Validation

The repository includes an integration test covering the end-to-end decision cycle.
