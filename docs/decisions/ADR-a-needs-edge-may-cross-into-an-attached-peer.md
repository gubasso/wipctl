# A dependency may cross into an attached peer

## Context and Problem Statement

An entry here can depend on an entry in another project's plan, and the record has nowhere to say so. The dependency list requires every id to exist in this record, so a foreign id fails the check. The only recourse is prose nothing reads.

## Considered Options

- An alias prefix on the existing dependency list, `<alias>#<id>`, resolved through the peer table
- A second field naming cross-plan dependencies — rejected: two fields holding one relation give two orderings
- One plan covering several projects — rejected here: that is a programme. It gives two teams one board, one cadence, one lock, and one reader, which answers a different question than one arrow
- A free-text reference nothing resolves — rejected: the record already has that, and no check and no proof can use it

## Decision Outcome

Chosen option: the alias prefix. No new field, no new edge kind, and no new meaning. Both forms mean closed before this entry is eligible, and eligibility, the graph, and the proof treat them identically, because they are the same edge.

A prefixed value is split once, on the first separator, by one function. The alias half resolves in the peer table beside the file that used it. The id half is carried whole, so the rule that no consumer parses an id survives: it is the id that is opaque, and a prefixed value was never one id.

A prefixed value is written double-quoted. It parses correctly without quotes, so the rule buys legibility rather than correctness: one form on the page, from every writer, and for a reviewer. That check is lexical, so the canonical-subset parser holds it and no schema can.

Enforced by `lane-file:a-prefixed-dependency-is-quoted` and `ids:a-prefixed-id-is-split-once`.

## Consequences

- Good: a real dependency on another team's work becomes a fact the tool orders, blocks, and proves acyclic.
- Bad: an entry can now be blocked by a plan this project does not control, and reading it requires that plan attached.

## Status

Accepted
