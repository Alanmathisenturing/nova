# NOVA Ω Implementation Ledger

This ledger records repository evidence, not architectural intent.

## Verification wave

| Task | Artifact | Invariant | Test | Observed | Replay | Independent verification | Adversarial | CI |
|---|---|---|---|---|---|---|---|---|
| Independent StateRoot reconstruction | `crates/nova-verifier` | Same ordered valid history produces the same canonical StateRoot | `crates/nova-verifier/src/lib.rs`, `tests/independent.rs` | Pending CI for current commits | Reconstructs state from history | Implemented as a separate transition path | Tamper, reorder, duplicate, root mismatch | Pending |

## Promotion rule

A layer cannot be promoted to VERIFIED merely because the implementation compiles.

Required:

```
artifact
→ invariant
→ executed test
→ observed result
→ independent reconstruction
→ adversarial test
```

Current verification implementation is intentionally conservative until GitHub Actions observes the current branch commits.
