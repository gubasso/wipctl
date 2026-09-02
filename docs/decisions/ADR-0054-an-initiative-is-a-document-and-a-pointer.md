# ADR-0054: An initiative is a document and a pointer from the epic

## Context and Problem Statement

Some end states are visible from no single epic, and the record has no way to say so. ADR-0004 rejected nested epics because a plan becomes a filing system and every consumer pays traversal for the rare deep case; a tier above the epic must answer that objection rather than reopen it.

## Considered Options

- One document under `initiatives/` plus an `Initiative` section on the epic documents that serve it, with the ladder bounded at three tiers by rule
- A member list in the initiative document — a second store of membership, stale on the first epic change, the fault ADR-0004 exists to refuse
- An `initiative` field on every lane entry — writes the same fact on every member and lets an entry claim an initiative its own epic does not serve
- A recursive parent link — the unbounded container chain, which is the filing system

## Decision Outcome

Chosen option: a document and a pointer, three tiers and no more. An initiative is an end state no single epic delivers: one document, no member list, membership pointing upward from the epic and stopping there. It draws from the shared id namespace, is a gated reference, never appears in `needs`, never sequences anything, and never enters ranking. Completion is derived by the same rule one tier down, and closed out is not done, which is why the document carries `Done when`. The depth bound answers ADR-0004: three tiers is a fixed traversal every consumer can write straight-line. A fourth tier is a new decision record, never a configuration value. Membership above the story flows only through the epic — a story with no epic is outside every initiative, and there is one path and no conflict rule to write.

## Consequences

- Good: the lane files, fragments, journal, ranking, and id grammar are untouched; the tier costs one document class and one section.
- Bad: a story serving a strategic end state alone must earn an epic to say so.

## Status

Accepted
