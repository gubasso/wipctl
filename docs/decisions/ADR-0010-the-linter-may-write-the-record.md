# ADR-0010: The linter may write the record

## Context and Problem Statement

Ranking legality (R1, R2, the closed-lane date order) is mechanical; restoring it by hand is
busywork. But a tool that writes the record risks becoming an author of priority.

## Considered Options

- A repair that writes on explicit request only, under strict guarantees
- Report-only, humans reorder — the machine knows the legal orders and withholds them; toil with no
  safety gain
- Canonical ordering — the tool becomes the author of priority, which the charter forbids

## Decision Outcome

Chosen option: an explicit-request repair — the machine restores legality without ever authoring
priority. The repair writes under four guarantees: deterministic; identity-preserving on legal
input, byte for byte, final newline included; idempotent; and non-canonical — every legal
permutation is a fixed point, so a legal human order is never disturbed. It relocates original line
blocks, never re-serialises, never moves an entry between lanes, and refuses entirely when content
checks fail or the graph is cyclic. No hook invokes it: a gate that rewrites the thing it gates
cannot fail, and a gate that cannot fail proves nothing.

## Consequences

- Good: `fix` exists, `fix --slots` reports legal candidates without writing, and validation stays
  read-only and falsifiable everywhere it is wired.
- Bad: restoring legality stays a human-invoked act; no gate ever repairs the record on its own.

## Status

Accepted
