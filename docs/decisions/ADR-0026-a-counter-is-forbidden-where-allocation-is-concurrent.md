# ADR-0026: A counter is forbidden where allocation is concurrent

## Context and Problem Statement

Two sessions capture at the same moment in two clones with no shared state. Any scheme where a session asks "what is the next id" makes them collide by design.

## Considered Options

- Unobservable allocation, with collisions reported by the gate
- A `next-id` file or lock — a second store, and no lock spans two clones
- Timestamp ids — collide within the clock's resolution and encode an order the id must not carry
- Content-hash ids — stable only while content is; an edited story would change its own name

## Decision Outcome

Chosen option: no counter, unobservable allocation — identity needs no coordination to mint. A mint reads nothing, locks nothing, waits for nothing. A collision is reported by the gate — the uniqueness check names both claimants — not prevented by the mint, and a colliding uid is re-minted, never incremented, because an incremented uid is a counter. The one excluded case is the decision record sequence (`ADR-NNNN`), allocated serially by one person merging one decision at a time; this record names that exception so nobody generalises from it.

## Consequences

- Good: identity works offline, in parallel, with no coordination.
- Bad: the gate carries the whole burden of uniqueness, which it already carries for every other cross-file fact.

## Status

Accepted

Amended by ADR-0038 — the gate names claimants that coexist in one record; a same-path collision between clones surfaces as a version-control conflict, resolved by re-minting one capture whole.
