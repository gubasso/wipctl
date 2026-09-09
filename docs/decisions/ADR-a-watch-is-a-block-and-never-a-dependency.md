# A watch is a block and never a dependency

## Context and Problem Statement

A project can wait on an item it does not own. The record needs to show which local work stops without claiming it can order the outside item.

## Considered Options

- `a watch with a Blocks line` — chosen.
- `a watch id in needs` — rejected: the dependency list sequences work this record can order, while the outside item is beyond that order.
- `the wait in an entry note` — rejected: nothing derives blocking or gives the wait a clear exit from prose in a note.

## Decision Outcome

Chosen option: `a watch with a Blocks line` — the edge names the local entries affected while leaving the outside item outside the dependency graph.

Enforced by `watches:a-watch-is-never-a-dependency`.

## Consequences

- Good: ranking can distinguish an outside wait from work the record can sequence.
- Bad: a person must clear the watch because no check reads the outside system.

## Status

Accepted
