# NOVA Ω Verification Matrix

Baseline: main @ 99c3e9d44e47f5c1d4caa2d4be919639f957ef0f
Date: 2026-10-06

| Layer | Artifact | Implementation | Invariant | Unit | Integration | Executed | Replay | Independent | Adversarial | CI | Gate |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Constitution | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | UNKNOWN | UNKNOWN | NO | NO | NO | UNKNOWN | BLOCKED |
| Primitive Types | YES | YES | PARTIAL | YES | PARTIAL | UNKNOWN | PARTIAL | NO | NO | UNKNOWN | BLOCKED |
| Event Model | YES | YES | PARTIAL | YES | YES | UNKNOWN | YES | NO | PARTIAL | UNKNOWN | BLOCKED |
| Evidence Kernel | YES | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | PARTIAL | NO | NO | UNKNOWN | BLOCKED |
| State / StateRoot | YES | YES | YES | YES | YES | UNKNOWN | YES | NO | PARTIAL | UNKNOWN | BLOCKED |
| Epistemic Runtime | YES | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | PARTIAL | NO | NO | UNKNOWN | BLOCKED |
| Decision Runtime | PARTIAL | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | NO | NO | NO | UNKNOWN | BLOCKED |
| Capability | YES | PARTIAL | PARTIAL | PARTIAL | NO | UNKNOWN | NO | NO | PARTIAL | UNKNOWN | BLOCKED |
| Authority | PARTIAL | PARTIAL | PARTIAL | PARTIAL | NO | UNKNOWN | NO | NO | NO | UNKNOWN | BLOCKED |
| Permit / Policy | YES | PARTIAL | PARTIAL | PARTIAL | NO | UNKNOWN | NO | NO | PARTIAL | UNKNOWN | BLOCKED |
| Adjudication | PARTIAL | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | PARTIAL | NO | PARTIAL | UNKNOWN | BLOCKED |
| Agency | PARTIAL | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | PARTIAL | NO | PARTIAL | UNKNOWN | BLOCKED |
| Institutional Runtime | YES | PARTIAL | PARTIAL | PARTIAL | YES | UNKNOWN | YES | NO | PARTIAL | UNKNOWN | BLOCKED |
| Replay | YES | YES | YES | YES | YES | UNKNOWN | YES | NO | PARTIAL | UNKNOWN | BLOCKED |
| Independent Verification | PARTIAL | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | PARTIAL | NO | NO | UNKNOWN | BLOCKED |
| Adversarial Verification | PARTIAL | PARTIAL | PARTIAL | PARTIAL | PARTIAL | UNKNOWN | PARTIAL | NO | NO | UNKNOWN | BLOCKED |
| Economic | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |
| Security Reality | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |
| Market Reality | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |
| Workforce Compiler | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |
| Self-Implementation Governor | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |
| Self-Governance | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |
| Production Hardening | NO | NO | NO | NO | NO | NO | NO | NO | NO | NO | NOT STARTED |

## Rules

UNKNOWN is not PASS.
PARTIAL is not PASS.
A phase cannot reach >=90% unless the required verification dimensions are satisfied.
