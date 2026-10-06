NOVA Ω Implementation Constitution

This is an execution contract, not a claim of completion.

Completion requires: concrete artifact, executable implementation, explicit invariant, executable test, observed test result, failure behavior, integration evidence where applicable, replay evidence where applicable, and regression protection.

Never claim implementation from prose, diagrams, TODOs, pseudocode, declarations alone, mocked execution presented as production execution, unexecuted tests, assumed repository state, or fabricated results.

Global invariants:
- Historical records are immutable.
- State changes occur only through validated transitions.
- Evidence, interpretation, belief, decision, authorization, action, outcome, and verdict remain distinct.
- No authority exists merely because an agent requests it.
- No execution occurs without a valid permit.
- Agency state changes require an explicit institutional transition.
- Corrections create new history; they do not rewrite prior history.
- Replay must reproduce the committed state root.

Verification levels:
NOT_IMPLEMENTED / IMPLEMENTED_UNTESTED / TESTED / VERIFIED / ADVERSARIALLY_VERIFIED.

Gate order:
1 Concept
2 Architecture
3 Formalization
4 Primitive Types
5 Kernel
6 Evidence
7 StateRoot
8 Agency
9 Authority
10 Adjudication
11 Execution
12 Replay
13 Integration
14 Institutional Proof

Final proof target:
LiveStateRoot == IndependentReplayStateRoot
plus adversarial rejection of unauthorized execution and historical mutation.

Current branch claim: an initial Adjudication primitive implementation has been added. Compilation and test execution are NOT VERIFIED until observed.
