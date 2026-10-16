# NOVA Ω Delivery Plan and Deadlines

**Plan date:** 2026-10-16  
**Status:** Proposed baseline, not a promise of completion.  
**Owner model:** Single founder with AI-assisted implementation.  
**Release rule:** No milestone is complete until its artifact, invariant, executable test, observed result, failure behavior, and replay evidence are linked.

## Schedule

| Gate | Target date | Deliverable | Exit criteria | Dependency |
|---|---|---|---|---|
| G0: Baseline | 2026-10-18 | Reproducible workspace and CI baseline | `cargo fmt --all -- --check`, `cargo check --workspace`, debug tests, and release tests complete on a pinned commit; results recorded | None |
| G1: Deterministic kernel | 2026-10-25 | Event → transition → commit → StateRoot → history → replay → verify | Replay reconstructs the same root; malformed or mutated history is rejected | G0 |
| G2: Epistemic and adjudication | 2026-11-08 | Evidence, information boundaries, decisions, outcomes, adjudication | Future evidence rejected at historical decision boundary; contradictions and insufficient evidence produce explicit non-assertive outcomes | G1 |
| G3: Agency and authorization | 2026-11-22 | Identity, qualification, delegation, policy, permit and execution boundary | No self-escalation; permit bound to actor, action, context and state root; expired/replayed permit rejected | G2 |
| G4: Integrated institutional runtime | 2026-12-06 | Evidence-to-outcome end-to-end vertical slice | Positive, negative, adversarial and replay tests pass; all links are attributable | G3 |
| G5: Technical alpha | 2027-01-15 | Durable persistence, recovery, restricted API, audit export | Restart/recovery tests pass; persistence corruption fails closed; documented threat model | G4 |
| G6: Product beta | 2027-03-15 | Usable API/UI for a narrow set of workflows | End-to-end user workflows, authorization, errors and audit records verified | G5 |
| G7: Zero-Employee operations pilot | 2027-05-01 | Workforce compiler and governed agent operations | Repeated synthetic/low-risk operating cycles; budgets, approvals, stop controls and accountability verified | G5, G6 |
| G8: Reality adapters | 2027-07-01 | Security and market adapters in isolated/sandbox mode | Source provenance, stale-data handling, limits, kill switch, and reproducible simulations; real capital remains a separate approval gate | G5, G6 |

## Deadline semantics

- Dates are target dates, not evidence of current completion.
- The next date is not permission to skip a blocked dependency.
- Any failed gate triggers a re-estimate based on observed work and failure cause.
- External integrations and real-capital actions cannot be promoted from simulation without a separate risk review.
- Production readiness is not implied by an alpha or beta milestone.

## Critical path

CI baseline → deterministic kernel and persistence → epistemic boundaries and adjudication → agency/authority/permit → integrated replay verification → alpha → product workflows → governed workforce → isolated reality adapters.

## First 48-hour objective

1. Repair the missing `nova-cfqp` library target on the mainline workspace by restoring the existing implementation and tests.
2. Run the full CI sequence and fix the next concrete failure, not hypothetical future issues.
3. Record the exact commit SHA and CI run URL.
4. Update the proof matrix only from observed results.
