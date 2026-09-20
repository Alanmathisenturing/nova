# NOVA

NOVA is a Rust-first epistemic execution runtime built around deterministic state, evidence, policy enforcement, and replayable outcomes.

## Status

**M0: IMPLEMENTED / TESTED** — deterministic event processing, state transition, commit roots, append-only in-memory history, and replay verification are available in the workspace. File persistence, distributed transport, and production deployment are not yet implemented.

## Quick start

```bash
cargo test --workspace
cargo run -p nova-runtime
```

## Runtime spine

```text
Event → Bus → Runtime → Transition → Commit → StateRoot → History → Replay → Verify
```

The authoritative state can only be changed by `nova-runtime` through the typed transition path. Events are immutable values; replay reconstructs state from event history rather than trusting a cached state.

## Crates

- `nova-types` — identifiers and deterministic SHA-256 digests
- `nova-event-bus` — ordered in-process event transport
- `nova-state` — minimal authoritative state and transition rules
- `nova-epistemic` — typed evidence and belief primitives
- `nova-glasswing` — capability, policy, and execution permits
- `nova-replay` — history replay and root verification
- `nova-evidence` — source-aware evidence contract
- `nova-provenance` — transition lineage records
- `nova-runtime` — executable end-to-end vertical slice

## Limitations

The current implementation is intentionally single-process and in-memory. It does not claim durable persistence, distributed consensus, cryptographic signatures, formal verification, or production readiness. Those are later milestones.
