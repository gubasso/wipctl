# Epics

An epic is an end state no single story delivers: one document under `epics/`, plus an `epic` field on the entries that serve it.

## Four properties

1. One-way membership. Entries point at the epic; the document lists no members. A member list would be a second store of the same fact, drifting the moment an entry changes.
2. Shared id space. Epic ids and entry ids draw from one namespace; one id names one thing.
3. Gated reference. An entry's `epic` field must name an existing document — unlike a `tag`, which nothing gates.
4. Never a dependency, and one optional parent. An epic never appears in `needs` and never sequences members. It may name the one initiative it serves — in its own document, nothing deeper ([initiatives.md](./initiatives.md)) — and the ladder stops there, because unbounded nesting is how a plan becomes a filing system.

## Completion is derived

An epic is closed out when every entry carrying its id is in `closed.yml`. Closed out is not done: a cut member closes without delivering, which is why the document carries `Done when` — an observable end state, never "every member closed". Where the host keeps rule ids, the sharpest end state is a set of rule ids whose verification commands all pass: the epic then never restates a specification's rules, it cites them, and doneness is decided by running the commands rather than by reading prose. The rollup shows delivered against promised (`cut` stays in the denominator); nothing derives a retirement decision, which is a review question asked alongside "is the charter still true". An empty epic — a document no entry has joined — is a legal first draft.

## When to open one

Open an epic when a split produced pieces whose shared end state is invisible from any one of them. Two stories rarely qualify. Anti-patterns: a renamed story with one member; a container opened before the split; an epic carrying acceptance criteria; an epic per release or per quarter.

## Where the approach goes

An epic states an end state, not a design. The approach that reaches it — the seams chosen, the reason for the order, the alternatives rejected — belongs in `Core` and `Example` when it is short, and in a host document the members name under `Reads` when it is long ([host-integration.md](./host-integration.md)). An epic has no section of its own for it, because a design an epic holds is a state nobody transfers forward when the strategy changes.

## Executing one

`wipctl epic <id>` resolves the epic into eligible and blocked groups in graph order, outside prerequisites included and marked. The loop: take one entry from `eligible now`, work it, close it, ask again. Asking again is not optional — a close changes eligibility and may change the ranking. One entry serves one end state; a story that seems to serve two was split along the wrong seam.
