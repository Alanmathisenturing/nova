# Repository Manifest

| Path | Purpose | Status | Evidence |
|---|---|---|---|
| `Cargo.toml` | Rust workspace | IMPLEMENTED | `cargo check --workspace` |
| `crates/nova-types` | Typed IDs and digests | IMPLEMENTED | crate source |
| `crates/nova-event-bus` | Typed events and ordered bus | TESTED | integrity unit tests |
| `crates/nova-state` | Minimal state transition and root | TESTED | transition tests |
| `crates/nova-replay` | Replay and root verification | TESTED | replay tests |
| `crates/nova-runtime` | End-to-end runtime binary | IMPLEMENTED | runtime tests |
| `crates/nova-provenance` | Transition witness | SCAFFOLDED | witness type only |
| `crates/nova-epistemic` | Evidence and belief primitives | SCAFFOLDED | primitive types only |
| `crates/nova-glasswing` | Capability and permit primitives | SCAFFOLDED | permit issuance unit test |
| `crates/nova-evidence` | Evidence re-export boundary | SCAFFOLDED | no runtime integration |
| Durable persistence | Append-only disk history | NOT_YET_NEEDED | next slice |

The manifest records repository reality and does not treat planned architecture as implemented behavior.
