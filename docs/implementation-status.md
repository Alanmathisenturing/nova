# M0 Implementation Status

| Area | Status | Evidence |
|---|---|---|
| Workspace | IMPLEMENTED | `cargo check --workspace` |
| Typed event model | IMPLEMENTED | `nova-event-bus` tests |
| State transition | IMPLEMENTED | `nova-state` tests |
| Deterministic StateRoot | IMPLEMENTED | `nova-state` root tests |
| Ordered event bus | IMPLEMENTED | `nova-event-bus` tests |
| In-memory commit history | IMPLEMENTED | `nova-runtime` tests |
| Replay verification | IMPLEMENTED | `nova-replay` tests |
| Durable persistence | NOT_IMPLEMENTED | Planned M1 |
| Provenance witness | IMPLEMENTED | `nova-provenance` |
| Capability enforcement | IMPLEMENTED | `nova-glasswing` tests |
| Epistemic runtime | SCAFFOLD/IMPLEMENTED | typed primitives only |
| Distributed execution | NOT_IMPLEMENTED | Planned later |

## Claims policy

`IMPLEMENTED` means executable behavior exists. `TESTED` means automated tests cover it. `VERIFIED` is reserved for behavior backed by deterministic and failure-path tests. This repository does not currently claim `PRODUCTION_READY`.
