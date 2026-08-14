# ADR-0023: Closing is not terminal

## Context and Problem Statement

Work returns: a review reopens a closed story, a cut turns out premature. A verb surface that
refuses the transition drives operators back to editing lane files by hand, which is the one thing
the transition rule exists to prevent.

## Considered Options

- `move` performs the transition out of `closed` like any other
- A dedicated `reopen` verb — a second spelling of `move` with the same preflight; the transition
  is not special enough to earn a name
- Preserving the old outcome for a later re-close — the record would carry a prediction; a re-close
  records the outcome it actually has

## Decision Outcome

Chosen option: `closed` is a lane, not a terminus — `move` performs the transition out of `closed`
into any lane. The entry travels with `outcome`, `closed`, and `succeeded_by` stripped; a
`closed -> <lane>` event is appended; the original closing event stays in the journal. Work-lane
eligibility applies to reopening as to any entry. A reopened entry counts as open in epic
arithmetic. `closed.yml`'s append-only date order is thereby a convention about arrival, not a
property the record depends on: a gap left by a departure is not a defect.

## Consequences

- Good: no transition drives an operator back to hand-editing lane files.
- Bad: the velocity of a past window can change when history is reopened — accepted, because
  velocity is derived and the record is the truth.
- Bad: the close-date ordering check must tolerate departures.

## Status

Accepted
