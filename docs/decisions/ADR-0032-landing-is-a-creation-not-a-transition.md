# ADR-0032: Landing is a creation, not a transition

## Context and Problem Statement

A fragment landing in `todo` looks like an arrival. Does it owe a journal event? The journal's completeness claim must say what an entry the journal never saw arriving looks like.

## Considered Options

- Landing writes no journal event; a journal begins at the first `move`
- A `pending -> todo` event — makes `pending` a lane in the journal's vocabulary but nowhere else; every journal rule and view must then special-case a lane that does not exist
- An event only for `todo` landings — two capture paths with different journal shapes, and the distinction records nothing a measure reads

## Decision Outcome

Chosen option: landing is a creation — the drain writes no journal event, and a landed entry's journal begins at its first `move`. The journal's claim is per-entry from first transition — journal-or-nothing — so an entry with no journal is the normal state of never-moved work, whether it was landed by the drain, written by hand in a zone without the tool, or scaffolded. One rule covers all three.

## Consequences

- Good: the journal vocabulary stays the five lanes plus `deleted`, and flow measures are unaffected — age starts at first entry into `doing` regardless of how the entry reached a planning lane.
- Bad: the journal never says how an entry reached a planning lane.

## Status

Accepted
