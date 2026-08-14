# ADR-0025: Two writers cannot lose a transition

## Context and Problem Statement

Two writing verbs in one working tree can interleave: the second reads a lane the first has not yet replaced, and the first writer's transition is silently lost. The lost update is decided at that read, not at the write.

## Considered Options

- An exclusive zone lock held by every writing verb
- Optimistic re-read and retry — the retry re-decides preflight against a record that changed, and two writers can livelock politely forever
- Locking readers too — readers are pure functions of the files; a torn read is re-run for free, and blocking `next` on a `move` inverts the tool's responsiveness contract
- Cross-machine coordination — out of scope by construction; between machines, version control is the coordinator

## Decision Outcome

Chosen option: an exclusive lock on the plan zone, writers only — the lost update is prevented at the read that decides it. Every verb that writes a maintained record holds the lock from its first read to its last rename, any rank repair included. A waiting writer says it is waiting and for whom; the wait is bounded and expires into exit 3 naming the holder, never an unbounded block. A stale holding (a dead holder) is released, not inherited. The lock is a lock and not a record: outside the zone's tracked content, absent from a clone, answering no question about the plan. Read-only verbs never take it.

## Consequences

- Good: `move`, `fix`, `land`, and `delete` serialise within a working tree, and a new writing verb inherits the obligation by the writer contract.
- Bad: within one working tree, writers wait on each other, bounded by the lock timeout.

## Status

Accepted
