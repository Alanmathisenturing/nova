# NOVA Ω Gate Status

This file reports repository evidence only. Percentages are not inferred from architecture prose.

| Gate | Status | Evidence |
|---|---|---|
| Concept | SPECIFIED | Existing architecture/specification |
| Architecture | SPECIFIED | Existing workspace structure |
| Formalization | SPECIFIED | Existing transition/root contracts |
| Primitive Types | IMPLEMENTED_UNTESTED | Existing typed primitives plus new adjudication primitives |
| Kernel | IMPLEMENTED_UNTESTED | Existing runtime spine; full institutional path not verified |
| Evidence | IMPLEMENTED_UNTESTED | Temporal/content/provenance-bound Evidence primitive implemented; CI result still unobserved |
| StateRoot | IMPLEMENTED_UNTESTED | Existing state/runtime root path; independent replay proof not verified |
| Agency | IMPLEMENTED_UNTESTED | nova-agency deterministic AgencyState + VerdictClass transition + StateRoot binding added; CI execution not yet observed |
| Authority | IMPLEMENTED_UNTESTED | Glasswing Authority + scoped, revocable, expiring, StateRoot-bound Permit checks added; CI execution not yet observed |
| Adjudication | IMPLEMENTED_UNTESTED | nova-adjudication Claim/Case/Verdict/Root primitives and tests |
| Execution | IMPLEMENTED_UNTESTED | nova-runtime submit_authorized integrates Glasswing authorization with state transition; CI execution not yet observed |
| Replay | IMPLEMENTED_UNTESTED | Existing replay crate; end-to-end institutional replay not verified |
| Integration | IMPLEMENTED_UNTESTED | nova-institution implements Evidence→Adjudication→Agency→Authority→Permit→Execution→Outcome→Replay |
| Institutional Proof | IMPLEMENTED_UNTESTED | nova-institution binds Evidence→Adjudication→Agency→Authority→Permit→Execution→Replay; CI result still unobserved |
| Decision | IMPLEMENTED_UNTESTED | nova-decision validates StateRoot, evidence presence, uncertainty and expiry |
| Economic State | IMPLEMENTED_UNTESTED | nova-economic provides root-bound capital reserve/release transitions and exposure limits |
| Workforce Compiler | IMPLEMENTED_UNTESTED | nova-workforce compiles bounded role specifications into deterministic AgentSpec |

## Current hard gate

Agency is intentionally marked IMPLEMENTED_UNTESTED until CI produces an observed result. The new transition layer is not considered verified until the repository CI executes its tests.

No status in this file is upgraded merely because code exists.
