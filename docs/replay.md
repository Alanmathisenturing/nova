# Replay Contract

Replay reconstructs state from event history. It does not trust a cached state.

For each event it verifies:

1. schema and payload shape;
2. event digest integrity;
3. monotonic event ordering;
4. parent linkage;
5. deterministic transition behavior;
6. final StateRoot, when an expected root is supplied.

An altered payload or metadata returns `IntegrityViolation` before transition application.
