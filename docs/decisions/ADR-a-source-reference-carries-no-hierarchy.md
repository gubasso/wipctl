# A source reference carries no hierarchy

## Context and Problem Statement

Every tier can carry a source reference: a story names the items its diff answers, an epic names the item that states its end state. The remote system has a tree of its own, and the tempting check is to test one against the other. That check would move membership out of this record.

## Considered Options

- `no relation between one artifact's references and another's` — chosen.
- `a parentage check against the remote tree` — rejected: it makes the outside tree the authority on membership, which is the second store the epic and initiative tiers exist to refuse.
- `deriving a tier from the remote system's own kinds` — deferred: revisit if a project shows that its outside container reliably means an end state no single story delivers.

## Decision Outcome

Chosen option: `no relation between one artifact's references and another's` — membership runs one way and only one way, from entry to epic to initiative, and nothing else may state it. A story's reference is never checked against its epic's. No tier is opened because a remote system has a container of that name, and no rollup counts a referenced item. The two trees stay unrelated by rule, so a team can reference whatever their other system holds without reshaping the plan around it. The tiers keep their own test: an end state no single member delivers.

Enforced by `external-sources:a-source-ref-carries-no-hierarchy` and `external-sources:a-source-ref-never-sequences-and-never-counts`.

## Consequences

- Good: the ladder, the ranking, and every derived number are untouched by the new fact, and no traversal grows a second path.
- Bad: nothing reports that a story and its epic reference unrelated outside trees. That reads as a gap until a reader remembers that the two trees were never claimed to agree.

## Status

Accepted
