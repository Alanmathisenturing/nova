# NOVA Implementation Status

## Current verified scope

The current M0 slice provides typed events, canonical event digests, deterministic state transitions, StateRoot generation, in-process ordered dispatch, in-memory history, and replay verification.

## Explicit limitations

- Persistence is in-memory only.
- Replay is not yet backed by a restartable append-only file.
- The event bus is not durable or distributed.
- No cryptographic signatures or formal proofs are implemented.
- Capability enforcement is a primitive boundary and is not integrated into the kernel.

## Next slice

Implement a versioned append-only persistence boundary with corruption detection and restart/replay tests.
