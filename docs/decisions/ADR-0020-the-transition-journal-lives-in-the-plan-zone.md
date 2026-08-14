# ADR-0020: The transition journal lives in the plan zone

## Context and Problem Statement

Transition events could live beside the record, in a per-user store, or in a service. A transition
and its event must not travel separately.

## Considered Options

- The journal in the zone, committed with the lane files
- A per-user store — fails the clean-clone test: the clone would carry the record but not its
  history, and two users would hold two histories
- A service — an availability dependency on a method whose promise is plain files
- A user path as a permitted cache — retained for caches only: a user path may never hold a fact a
  gate consults

## Decision Outcome

Chosen option: the journal lives in the zone, under `journal/`, committed with the lane files it
describes — a transition and its event travel in one change, merge in one merge, and arrive in one
clone.

## Consequences

- Good: the record's proposal mechanism (a branch) proposes transitions too.
- Bad: journal merge behaviour needs its own design, which ADR-0021 settles.

## Status

Accepted
