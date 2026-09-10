# A watch carries the date it was last read

## Context and Problem Statement

A watch can outlive the outside condition that once blocked work. The record cannot read a remote system, but it can state when a person last checked the item and what they saw.

## Considered Options

- `a dated observation` — chosen.
- `a mirrored state field` — rejected: the outside system owns that state, and an undated copy can age without showing its age.
- `no reading` — rejected: a reader cannot distinguish a fresh block from a guess nobody has checked for months.

## Decision Outcome

Chosen option: `a dated observation` — each watch records the date a person looked and one sentence describing what they saw.

Enforced by `watches:a-reading-carries-its-date-and-its-observation`.

## Consequences

- Good: the record can announce an old reading without reaching a network.
- Good: the observation remains true as a statement about its date even when the outside item changes later.
- Bad: a person must read the item and update the watch.

## Status

Accepted
