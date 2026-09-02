# Transition journal

Lane files say where an entry is; the journal says when it arrived. Every measure of flow — age, dwell, rework — is a question about arrival, so transitions are recorded by the program as events, in the record itself. Version control only witnesses them; it is never the source: history can be rewritten, squashed, or absent from a shallow clone, and requiring a version-control practice would exceed the single host assumption.

## Format

One append-only stream per entry: `journal/<id>.tsv`. One event per line, exactly three non-empty tab-separated fields:

```text
2026-08-14T09:52:31Z	todo	doing
2026-08-15T16:03:10Z	doing	review
2026-08-16T11:20:44Z	review	closed
```

- Field 1 — the instant, RFC 3339 UTC with `Z`, second precision. One fixed textual form, so lexical order is chronological order and sorting needs no date parsing.
- Field 2 — the lane left.
- Field 3 — the lane entered, or a terminal token: `deleted` or `renamed`.

The file is created on the entry's first transition. An entry that never moved has no journal file. Landing a fragment is a creation, not a transition: a landed entry's journal starts at its first move, not at its landing.

The journal has no schema on purpose: validating it with a schema toolchain would require the very parsing the three-column format avoids. Its rules live in the cross-file checker.

## Two halves of one record

Lane files are authoritative for current state and are the reviewable diff. The journal is authoritative for transitions. Neither is derived from the other; the gate holds them in agreement: the last event's destination MUST equal the entry's actual lane, and the failure names both sides and offers both repairs.

## Rules the checker holds

- A journal for an id in no lane is a failure — unless it is a tombstone.
- Every line MUST have exactly three non-empty tab-separated fields.
- Every instant MUST be valid RFC 3339 UTC with `Z` (real calendar date, hour 23 at most, minute 59 at most, second 60 at most for a leap second).
- Both lane names on an event MUST be real lanes; the destination MAY also be `deleted` or `renamed`.
- An event's source MUST NOT equal its destination, and MUST equal the destination of the event above it — a journal is one unbroken path; the first event's source is bound only to be a real lane.
- An event dated after now is a failure. An event preceding the one above it is a warning.
- The last event's lane MUST equal the entry's lane. For an entry in `closed`, the last event MUST enter `closed` and its day MUST equal the entry's `closed:` date.
- Journal-or-nothing: an entry with a journal is held to every rule above; an entry with no journal is held to none of them. This is what lets a record predating the journal keep passing.

## Reopening

`closed` is a lane, not a terminus. An entry moved out of `closed` MUST travel with `outcome`, `closed`, and `succeeded_by` stripped, and a `closed -> <lane>` event MUST be appended; the original closing event stays. A gap a departure leaves in `closed.yml`'s date order is not a defect. A reopened entry counts as open in epic arithmetic.

## Tombstones

A journal whose final event's destination is `deleted` or `renamed` is a tombstone. A deleted entry keeps nothing but its tombstone. A renamed entry travels whole to its new id — document, journal, fragment, history — and leaves a fresh one-line tombstone at the old id, whose event's destination is `renamed`, in every case but one: a rename resolving an id collision frees no id, because the surviving capture still holds it, and so leaves no tombstone (see [../cli/verbs/rename.md](../cli/verbs/rename.md)). A tombstone is exempt from the id-in-no-lane failure and from the lane-agreement invariant, and is held to every other journal rule. In either case the id the tombstone holds is burned; see [ids.md](./ids.md).

## What the journal does not record

No who, no why, no reason column — three columns is the whole format. Nothing proves the verb was used rather than the file edited; that is exactly what the lane-agreement gate exists to catch.
