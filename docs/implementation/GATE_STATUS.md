# NOVA Ω Gate Status

This file reports repository evidence only. Percentages are not inferred from architecture prose.

| Gate | Status | Evidence |
|---|---|---|
| Concept | SPECIFIED | Existing architecture/specification |
| Architecture | SPECIFIED | Existing workspace structure |
| Formalization | SPECIFIED | Existing transition/root contracts |
| Primitive Types | IMPLEMENTED_UNTESTED | Existing typed primitives plus new adjudication primitives |
| Kernel | IMPLEMENTED_UNTESTED | Existing runtime spine; full institutional path not verified |
| Evidence | IMPLEMENTED_UNTESTED | Existing crate is present; full evidence runtime not verified |
| StateRoot | IMPLEMENTED_UNTESTED | Existing state/runtime root path; independent replay proof not verified |
| Agency | NOT_IMPLEMENTED | Institutional Agency state machine not yet integrated |
| Authority | IMPLEMENTED_UNTESTED | Existing Glasswing permits; institutional authority derivation not verified |
| Adjudication | IMPLEMENTED_UNTESTED | nova-adjudication Claim/Case/Verdict/Root primitives and tests |
| Execution | IMPLEMENTED_UNTESTED | Existing runtime execution path; institutional authorization integration not verified |
| Replay | IMPLEMENTED_UNTESTED | Existing replay crate; end-to-end institutional replay not verified |
| Integration | NOT_IMPLEMENTED | Full Evidence→Adjudication→Agency→Authority→Execution→Replay path not verified |
| Institutional Proof | NOT_IMPLEMENTED | Requires independent replay plus adversarial end-to-end evidence |

## Current hard gate

Adjudication is intentionally marked IMPLEMENTED_UNTESTED until CI produces an observed result.

No status in this file is upgraded merely because code exists.
