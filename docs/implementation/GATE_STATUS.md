# NOVA Ω Gate Status

This file reports repository evidence only. Percentages are not inferred from architecture prose.

| Gate | Status | Evidence |
|---|---|---|
| Concept | SPECIFIED | Existing architecture/specification |
| Architecture | SPECIFIED | Existing workspace structure |
| Formalization | SPECIFIED | Existing transition/root contracts |
| Primitive Types | TESTED | Existing typed primitives plus new adjudication primitives |
| Kernel | TESTED | Existing runtime spine; full institutional path not verified |
| Evidence | TESTED | Temporal/content/provenance-bound Evidence primitive implemented; CI result still unobserved |
| StateRoot | TESTED | Existing state/runtime root path; independent replay proof not verified |
| Agency | TESTED | nova-agency deterministic AgencyState + VerdictClass transition + StateRoot binding added; CI execution not yet observed |
| Authority | TESTED | Glasswing Authority + scoped, revocable, expiring, StateRoot-bound Permit checks added; CI execution not yet observed |
| Adjudication | TESTED | nova-adjudication Claim/Case/Verdict/Root primitives and tests |
| Execution | TESTED | nova-runtime submit_authorized integrates Glasswing authorization with state transition; CI execution not yet observed |
| Replay | TESTED | Existing replay crate; end-to-end institutional replay not verified |
| Integration | TESTED | nova-institution implements Evidence→Adjudication→Agency→Authority→Permit→Execution→Outcome→Replay |
| Institutional Proof | TESTED | nova-institution binds Evidence→Adjudication→Agency→Authority→Permit→Execution→Replay; CI result still unobserved |
| Decision | TESTED | nova-decision validates StateRoot, evidence presence, uncertainty and expiry |
| Economic State | TESTED | nova-economic provides root-bound capital reserve/release transitions and exposure limits |
| Workforce Compiler | TESTED | nova-workforce compiles bounded role specifications into deterministic AgentSpec |

## Current verification boundary

GitHub Actions run `37417532262` for commit `57fb4c0175da9244f629a9608c9ac4b588c57186` observed all of the following as successful: `cargo fmt --all`, `cargo check --workspace`, `cargo test --workspace`, and `cargo test --workspace --release`.

This promotes implemented layers to TESTED, not VERIFIED. VERIFIED still requires independent reconstruction/replay evidence that does not merely trust the runtime's own assertions.

No status is upgraded merely because code exists.
