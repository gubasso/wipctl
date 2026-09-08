# Transition Journal Specification

<!--TOC-->

- [Purpose](#purpose)
- [Format](#format)
- [Requirements](#requirements)
  - [`transition-journal:the-program-records-transitions` — The program records transitions](#transition-journalthe-program-records-transitions--the-program-records-transitions)
  - [`transition-journal:an-event-is-three-non-empty-fields` — An event is three non-empty fields](#transition-journalan-event-is-three-non-empty-fields--an-event-is-three-non-empty-fields)
  - [`transition-journal:an-instant-is-one-textual-form` — An instant is one textual form](#transition-journalan-instant-is-one-textual-form--an-instant-is-one-textual-form)
  - [`transition-journal:a-journal-is-one-unbroken-path` — A journal is one unbroken path](#transition-journala-journal-is-one-unbroken-path--a-journal-is-one-unbroken-path)
  - [`transition-journal:the-last-event-agrees-with-the-lane` — The last event agrees with the lane](#transition-journalthe-last-event-agrees-with-the-lane--the-last-event-agrees-with-the-lane)
  - [`transition-journal:landing-is-a-creation` — Landing is a creation, not a transition](#transition-journallanding-is-a-creation--landing-is-a-creation-not-a-transition)
  - [`transition-journal:journal-or-nothing` — A journal is judged whole or not at all](#transition-journaljournal-or-nothing--a-journal-is-judged-whole-or-not-at-all)
  - [`transition-journal:an-event-is-not-dated-ahead` — An event is not dated ahead](#transition-journalan-event-is-not-dated-ahead--an-event-is-not-dated-ahead)
  - [`transition-journal:reopening-strips-the-close` — Reopening strips the close](#transition-journalreopening-strips-the-close--reopening-strips-the-close)
  - [`transition-journal:a-tombstone-is-a-terminal-journal` — A tombstone is a terminal journal](#transition-journala-tombstone-is-a-terminal-journal--a-tombstone-is-a-terminal-journal)
  - [`transition-journal:the-journal-records-no-reason` — The journal records no reason](#transition-journalthe-journal-records-no-reason--the-journal-records-no-reason)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Lane files say where an entry is, and the journal says when it arrived. Every measure of flow is a question about arrival, so transitions are events in the record itself. The boundary runs at authority. The lane files are authoritative for current state and are the reviewable diff. The journal is authoritative for transitions. Neither is derived from the other.

## Format

One append-only stream per entry, one event per line, exactly three non-empty tab-separated fields.

```text
2026-08-14T09:52:31Z	todo	doing
2026-08-15T16:03:10Z	doing	review
2026-08-16T11:20:44Z	review	closed
```

Field one is the instant, field two is the lane left, and field three is the lane entered or a terminal token.

## Requirements

### `transition-journal:the-program-records-transitions` — The program records transitions

The program MUST record every transition as an event in the record, and MUST NOT read version control as the source of a transition.

#### Scenario: A clone is shallow

- GIVEN a checkout whose history was squashed or truncated
- WHEN flow is measured
- THEN the journal still answers. History can be rewritten, and requiring a version-control practice exceeds the single assumption about the host

Verify: `cargo nextest run --test validation`

### `transition-journal:an-event-is-three-non-empty-fields` — An event is three non-empty fields

Every journal line MUST carry exactly three non-empty tab-separated fields, and both lane names MUST be real lanes or a terminal token.

#### Scenario: A field is left blank

- GIVEN a line with an empty second field
- WHEN validation runs
- THEN it fails, because the three-column format is the whole grammar and a blank column states nothing

Verify: `cargo nextest run --test validation`

### `transition-journal:an-instant-is-one-textual-form` — An instant is one textual form

Every instant MUST be a valid coordinated universal time in the stated textual form, at second precision.

#### Scenario: A journal is sorted

- GIVEN a stream of events
- WHEN they are ordered
- THEN lexical order is chronological order, because one fixed textual form means sorting needs no date parsing

Verify: `cargo nextest run --test validation`

### `transition-journal:a-journal-is-one-unbroken-path` — A journal is one unbroken path

An event's source MUST NOT equal its destination, and MUST equal the destination of the event above it.

#### Scenario: A move is recorded twice

- GIVEN a second event repeating a destination as its own source and destination
- WHEN validation runs
- THEN it fails, because a journal is one path and the first event's source is bound only to be a real lane

Verify: `cargo nextest run --test validation`

### `transition-journal:the-last-event-agrees-with-the-lane` — The last event agrees with the lane

The last event's destination MUST equal the entry's lane, and a closed entry's last event MUST carry the entry's own close date.

#### Scenario: A lane file is edited by hand

- GIVEN an entry moved without the verb
- WHEN the gate runs
- THEN it fails naming both sides and offering both repairs, which is exactly what catches an edit that bypassed the program

Verify: `cargo nextest run --test validation`

### `transition-journal:landing-is-a-creation` — Landing is a creation, not a transition

The implementation MUST create an entry's journal at its first move and MUST NOT write an event when a fragment lands.

#### Scenario: A capture is drained

- GIVEN a fragment landing into a planning lane
- WHEN the drain completes
- THEN no journal exists yet, because an entry that never moved has no journal file rather than an empty one

Verify: `cargo nextest run --test validation`

### `transition-journal:journal-or-nothing` — A journal is judged whole or not at all

The implementation MUST hold an entry with a journal to every journal rule, and an entry with no journal to none of them.

#### Scenario: A record predates the journal

- GIVEN entries that moved before the journal existed
- WHEN validation runs
- THEN they keep passing. Holding a partial journal to the full rule set fails records nobody can repair

Verify: `cargo nextest run --test validation`

### `transition-journal:an-event-is-not-dated-ahead` — An event is not dated ahead

An event dated after the present MUST be a failure, and an event preceding the one above it MUST be a warning.

#### Scenario: A machine's clock is wrong

- GIVEN an event stamped in the future
- WHEN validation runs
- THEN it fails, while an out-of-order pair is only a warning, because clocks on two machines drift and the order is still readable

Verify: `cargo nextest run --test validation`

### `transition-journal:reopening-strips-the-close` — Reopening strips the close

When an entry leaves the closed lane, the implementation MUST strip its closing fields, MUST append the departure event, and MUST keep the closing event.

#### Scenario: Finished work turns out to be unfinished

- GIVEN a closed entry that reopens
- WHEN the move completes
- THEN the gap left in the close order is not a defect, and the reopened entry counts as open in epic arithmetic

Verify: `cargo nextest run --test validation`

### `transition-journal:a-tombstone-is-a-terminal-journal` — A tombstone is a terminal journal

A journal ending in a terminal token MUST be a tombstone, exempt from the lane-agreement rule and held to every other journal rule.

#### Scenario: An entry is renamed

- GIVEN an entry travelling whole to a new id
- WHEN the rename completes
- THEN a fresh one-line tombstone is left at the old id, except where the rename resolves a collision and frees nothing

Verify: `cargo nextest run --test validation`

### `transition-journal:the-journal-records-no-reason` — The journal records no reason

The journal MUST NOT carry a who, a why, or a reason column.

#### Scenario: Someone wants to know who moved an entry

- GIVEN a request for an author column
- WHEN the format is read
- THEN three columns is the whole of it. Nothing proves the verb was used rather than the file edited, which the lane-agreement gate catches

Verify: `cargo nextest run --test validation`

## Unenforced rules

| Rule                                                 | Why no command decides it                                                           |
| ---------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `transition-journal:the-program-records-transitions` | No command can tell an event the program wrote from one an author appended by hand. |

The journal carries no schema on purpose. Validating a three-column stream with a schema toolchain needs the very parsing the format avoids. Its rules live in the cross-file checker.
