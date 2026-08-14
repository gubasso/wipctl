# ADR-0024: A deletion is recorded and burns the id

## Context and Problem Statement

An entry that should never have existed — a mistype, a duplicate — has no honest lane. `cut` is
wrong: `cut` is a decision the record keeps, with points in the epic denominator.

## Considered Options

- A `delete` verb that removes the entry, leaves the journal as a tombstone, and burns the id
- Plain file removal — the id becomes reusable, and "one id names one thing" holds only for the
  record's present, not its life; any old reference silently rebinds
- A `deleted` lane — deletion is not a workflow state; a sixth lane would appear in every view and
  every schema for a case that should be rare
- A reason column on the event — the journal stays three columns; a reason is prose, and prose has
  other homes

## Decision Outcome

Chosen option: recorded deletion with a burned id — the record never loses the fact that something
was deleted. `delete` removes the lane entry and its story document, and appends a final
`<lane> -> deleted` event to the entry's journal. The journal remains as a tombstone: exempt from
the id-in-no-lane failure and the lane-agreement invariant, held to every other journal rule. The
id is burned — a new entry, epic, or fragment claiming it is a failure naming both the claimant and
the tombstone. Deletion refuses while anything still names the id (`needs`, `succeeded_by`, a
`Blocks:` line).

## Consequences

- Good: "one id names one thing" holds for the record's whole life, not only its present.
- Bad: two checker rules (tombstone exemption, burned-id refusal) and a tenth-verb-sized addition
  to the surface.

## Status

Accepted
