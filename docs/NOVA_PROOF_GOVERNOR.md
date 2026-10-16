# NOVA Ω Proof Governor

## Purpose

Prevent schedule pressure, documentation, generated code, or declared percentages from being mistaken for implementation proof.

## Required proof record

Each claim must map to:
1. **Claim/property** — falsifiable statement.
2. **Invariant** — condition that must always hold.
3. **Artifact** — exact source path and commit.
4. **Executable test** — exact test name or command.
5. **Observed result** — pass/fail/blocked with run URL and commit SHA.
6. **Adversarial case** — mutation, invalid input, replay, boundary violation, or failure injection.
7. **Replay/independent check** — when relevant.
8. **Failure behavior** — fail closed, reject, hold, or explicitly return indeterminate.
9. **Dependencies** — upstream gates required for validity.

## Status vocabulary

- `PASS`: evidence exists for the exact commit and required test.
- `FAIL`: a test or invariant was violated.
- `BLOCKED`: prerequisite prevented execution.
- `UNKNOWN`: no trustworthy execution evidence.
- `PARTIAL`: some but not all required dimensions are verified.

UNKNOWN, BLOCKED and PARTIAL are never PASS. A successful compile is not proof of an invariant. A unit test is not proof of integration, durability, adversarial resistance, or production readiness.

## Merge gate

A change may be proposed for review only when:
- workspace formatting and compilation have been attempted;
- relevant tests are present;
- claims are scoped to what the tests actually prove;
- limitations and unverified properties are explicit.

A release gate additionally requires debug and release tests, integration/replay coverage, security review appropriate to the risk, and a reproducible artifact.

## Current known blocker (2026-10-16)

The recorded GitHub Actions run `37419961715` failed before compilation because Cargo could not find a target for `crates/nova-cfqp`. The library manifest existed but its `src/lib.rs` target did not exist on `main`. The correct first action is to restore the implementation and its tests, then rerun CI. The old failure is historical evidence only; it must not be described as the status of a later commit.

## Non-goals

This governor does not claim formal verification, zero-knowledge proofs, distributed consensus, economic safety, or production readiness unless those properties are separately implemented and independently verified.
