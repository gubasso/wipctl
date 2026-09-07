# A lane transition is a recorded event

## Context and Problem Statement

Age, dwell, and rework are questions about when an entry arrived somewhere. Lane files hold only where it is now.

## Considered Options

- The program appends an event as it performs the move
- Deriving transitions from repository history — fails three independent ways: history is rewritten by a rebase, collapsed by a squash (a round trip folds to nothing), and absent from a shallow clone; and it would demand a git practice, exceeding the single host assumption
- A timestamp field on the lane entry — a scalar cannot hold a round trip: the second entry into `doing` overwrites the first, and rework becomes unrecordable

## Decision Outcome

Chosen option: a recorded event — the record states its own history. A lane change is an event the record states: the program performs the move and appends the event in one act. Version control witnesses the record; it is never the source of transitions.

## Consequences

- Good: the journal exists, `flow` reads record facts, and the gate holds lanes and journal in agreement.
- Bad: every transition the method permits must be a verb — reopening, deletion, and the lock sit on the surface for this reason.

## Status

Accepted
