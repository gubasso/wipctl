# ADR-0038: Capture disjointness holds outside collision

## Context and Problem Statement

ADR-0028 promises that parallel captures are disjoint files merging with no conflict, while
ADR-0026 accepts that two random uid draws can collide. Colliding captures produce the same story
and fragment paths, so the merge can conflict before any gate sees two claimants.

## Considered Options

- State the guarantee as conditional on no collision, with re-minting one side as the recovery
- Prevent collisions at mint time — requires shared state between clones, which ADR-0026 already
  rejected with counters
- Make the drain resolve colliding captures — a machine merging two different stories under one
  id is a machine choosing content

## Decision Outcome

Chosen option: the conditional guarantee with re-mint recovery — the disjointness ADR-0028
promises holds in every capture where no uid collision occurred, and a collision has one stated
recovery. The version-control add/add conflict on the shared paths is the one reliable collision
signal: a merge resolved by taking each path from a different side splices two captures into one
id, the ids at every path still agree, and no gate can see the splice. The recovery therefore
keeps one capture's document and fragment together as a pair and re-mints the other capture
entirely — re-captures it under a fresh uid — before merging again. Nothing repairs a collision
silently.

## Consequences

- Good: the ordinary path keeps its no-conflict merge, and the rare collision has a mechanical,
  documented recovery instead of an undefined state.
- Bad: a collision costs one session a re-capture, and the guarantee's statement is longer than
  the absolute it replaces.

## Status

Accepted

Amends ADR-0028 — the no-conflict merge consequence is conditional on no uid collision, with
re-minting as the recovery.
