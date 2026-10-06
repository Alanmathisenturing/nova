# NOVA Ω Institutional Runtime

The repository now contains an executable integration boundary for:

Evidence → Adjudication → Agency → Authority → Permit → Execution → Outcome → Replay

The integration crate is `crates/nova-institution`.

## State contract

InstitutionalState binds AgencyState and execution State into one deterministic StateRoot.

Every adjudication and execution checks the current root before mutation.

## Evidence contract

Evidence digest binds identity, observation time, availability time, content and provenance. Tampering or impossible temporal ordering is rejected.

## Adjudication contract

A verdict is bound to its case, evidence and prior institutional StateRoot. The resulting AdjudicationRoot becomes input to Agency transition.

## Agency contract

Supported → Active  
WeaklySupported → Probation  
Contradicted → Suspended  
Invalid → Revoked  
Rejected → Revoked  
Underdetermined → preserve status, advance epoch

## Authority contract

Only Active and Probation agencies can issue the bounded counter capability. Suspended and Revoked states cannot issue execution permits.

## Execution contract

A permit is bound to actor, capability, resource, event and StateRoot. The runtime checks the permit and current root before applying the state transition.

## Replay contract

The institutional history is replayed from the deterministic initial state. The resulting StateRoot must equal the committed root.

## Verification rule

The code is not marked VERIFIED until GitHub Actions produces an observed result for the current commit.