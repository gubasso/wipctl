# A slot repair is its own verb form

## Context and Problem Statement

A drifted slot is healthy, but a person can want its path to show the current project name. Renaming it changes the registry, state, cache, and two lock names without changing the record.

## Considered Options

- `wipctl fix --registry [<name>]` — chosen because `fix` owns repairs and the registry owns slots.
- Extend `wipctl fix --slots` — rejected because `repair:the-slot-report-writes-nothing` makes that ranking report read-only.
- Add a fifth attachment form — rejected because the plan is already attached and the operation changes only its local slot.

## Decision Outcome

Chosen option: `wipctl fix --registry [<name>]`. The repair takes both name locks in lexical order, moves the slot and state, then removes the old cache. Each boundary can resume after interruption. No project file or plan commit changes because a slot is local to one machine.

Enforced by `repair:a-slot-repair-moves-local-state-in-order`, `repair:a-slot-repair-locks-both-names`, `repair:an-explicit-slot-name-is-free`, and `repair:a-slot-repair-is-machine-local`.

## Consequences

- Good: an explicit operation owns every local move and can resume safely.
- Bad: the repair needs two locks and can refuse an operator-chosen occupied name.

## Status

Accepted
