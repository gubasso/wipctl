# Pending Fragment Specification

<!--TOC-->

- [Purpose](#purpose)
- [File shape](#file-shape)
- [Requirements](#requirements)
  - [`pending-fragment:a-fragment-claims-a-planning-lane` — A fragment claims a planning lane](#pending-fragmenta-fragment-claims-a-planning-lane--a-fragment-claims-a-planning-lane)
  - [`pending-fragment:a-position-names-a-landed-entry` — A position names a landed entry](#pending-fragmenta-position-names-a-landed-entry--a-position-names-a-landed-entry)
  - [`pending-fragment:the-capture-instant-orders-the-drain` — The capture instant orders the drain](#pending-fragmentthe-capture-instant-orders-the-drain--the-capture-instant-orders-the-drain)
  - [`pending-fragment:a-fragment-requires-only-id-and-type` — A fragment requires only an id and a type](#pending-fragmenta-fragment-requires-only-id-and-type--a-fragment-requires-only-an-id-and-a-type)
  - [`pending-fragment:a-closing-field-never-appears` — A closing field never appears](#pending-fragmenta-closing-field-never-appears--a-closing-field-never-appears)
  - [`pending-fragment:an-unknown-field-is-a-failure` — An unknown field is a failure](#pending-fragmentan-unknown-field-is-a-failure--an-unknown-field-is-a-failure)
  - [`pending-fragment:a-pending-entry-has-no-rank` — A pending entry has no rank](#pending-fragmenta-pending-entry-has-no-rank--a-pending-entry-has-no-rank)
  - [`pending-fragment:a-claimed-document-is-not-an-orphan` — A claimed document is not an orphan](#pending-fragmenta-claimed-document-is-not-an-orphan--a-claimed-document-is-not-an-orphan)
  - [`pending-fragment:a-fragment-id-is-not-a-duplicate` — A fragment id is not a duplicate](#pending-fragmenta-fragment-id-is-not-a-duplicate--a-fragment-id-is-not-a-duplicate)
  - [`pending-fragment:the-drain-reports-drift-and-repairs-none` — The drain reports drift and repairs none](#pending-fragmentthe-drain-reports-drift-and-repairs-none--the-drain-reports-drift-and-repairs-none)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

One file per captured entry, written by capture and consumed by the drain. A fragment states a delta, meaning where a new entry wants to land, and never a copy of anything a lane file holds. The boundary runs at landing. This domain owns the fragment and what the drain refuses. The lane file domain owns the entry once it lands, and it owns the two forms a dependency takes.

## File shape

```yaml
lane: backlog
after: null
captured: "2026-08-14T09:41:07Z"
entry:
  id: rate-limit-the-search-endpoint
  type: story
  points: 2
  summary: "Callers of the search endpoint are limited per token, and the limit is announced in the response headers rather than discovered by being cut off."
  needs: ["payments#secure-session-storage"]
  epic: session-hardening
```

A fragment's `needs` accepts the same two forms a lane entry's does, and the double-quoting rule applies here too.

## Requirements

### `pending-fragment:a-fragment-claims-a-planning-lane` — A fragment claims a planning lane

A fragment MUST claim a planning lane and MUST NOT claim a work lane.

#### Scenario: A capture wants to start work directly

- GIVEN a fragment claiming the in-flight lane
- WHEN the schema runs
- THEN it fails, because landing into a work lane is a transition, and a transition owes a journal event

Verify: `cargo nextest run --test schemas`

### `pending-fragment:a-position-names-a-landed-entry` — A position names a landed entry

A fragment's position claim MUST be empty or name a landed entry, and MUST NOT name an id that exists only as another fragment.

#### Scenario: Two captures reference each other

- GIVEN a fragment positioned beneath another fragment
- WHEN validation runs
- THEN it fails, because a position between two things that have not landed states no order

Verify: `cargo nextest run --test validation`

### `pending-fragment:the-capture-instant-orders-the-drain` — The capture instant orders the drain

A fragment MUST state the capture instant in the stated timestamp form, and the drain MUST order by it.

#### Scenario: Two machines capture while apart

- GIVEN two fragments captured on different machines
- WHEN they drain together
- THEN the stated instants order them, because file order on disk is an accident of the filesystem

Verify: `cargo nextest run --test schemas`

### `pending-fragment:a-fragment-requires-only-id-and-type` — A fragment requires only an id and a type

A fragment's entry MUST require an id and a type alone, and the drain MUST refuse an incomplete fragment naming the file and the field.

#### Scenario: A capture is committed before the points are chosen

- GIVEN a fragment written with no points and no summary
- WHEN it is committed
- THEN the schema accepts it and the drain refuses it. A schema requiring them rejects every capture at its own commit

Verify: `cargo nextest run --test schemas`

### `pending-fragment:a-closing-field-never-appears` — A closing field never appears

A fragment MUST NOT carry an outcome, a close date, a successor, or a delivered commit.

#### Scenario: A capture describes finished work

- GIVEN a fragment carrying a close date
- WHEN the schema runs
- THEN it fails, because a capture is never a close

Verify: `cargo nextest run --test schemas`

### `pending-fragment:an-unknown-field-is-a-failure` — An unknown field is a failure

An unknown field anywhere in a fragment MUST be a failure.

#### Scenario: A field name is misspelled

- GIVEN a fragment with a mistyped key
- WHEN the schema runs
- THEN it fails, because a silently ignored key makes a typo look like a setting that had no effect

Verify: `cargo nextest run --test schemas`

### `pending-fragment:a-pending-entry-has-no-rank` — A pending entry has no rank

A pending entry MUST be part of the record for validation alone: never eligible, without rank, invisible to the next-work verb and the epic verbs, and outside epic arithmetic.

#### Scenario: An epic is summarised while a capture is pending

- GIVEN a fragment naming an epic
- WHEN the epic's points are totalled
- THEN the fragment's points are not counted, because an unlanded entry has no agreed position in the plan

Verify: `cargo nextest run --test validation`

### `pending-fragment:a-claimed-document-is-not-an-orphan` — A claimed document is not an orphan

The implementation MUST treat a story document claimed by a fragment as claimed, and MUST fail a document that nothing claims.

#### Scenario: A capture writes its document first

- GIVEN a document whose only claim is a pending fragment
- WHEN validation runs
- THEN it passes, while a document no lane and no fragment claims fails

Verify: `cargo nextest run --test validation`

### `pending-fragment:a-fragment-id-is-not-a-duplicate` — A fragment id is not a duplicate

A fragment's id MUST NOT appear in a lane file, and its entry, epic, and dependencies MUST each resolve in the record each one names.

#### Scenario: A capture is drained twice

- GIVEN an id present in both a fragment and a lane
- WHEN validation runs
- THEN it fails as a duplicate, because two records of one entry disagree on the first edit. A prefixed dependency resolves in the peer its alias names, under the same rule

Verify: `cargo nextest run --test validation`

### `pending-fragment:the-drain-reports-drift-and-repairs-none` — The drain reports drift and repairs none

When the drain finds drift, it MUST report the drift and MUST NOT repair it.

#### Scenario: A position target closed while the capture waited

- GIVEN a fragment positioned beneath an entry that has since closed
- WHEN the drain runs
- THEN the fragment lands at the bottom of its claimed lane and the report says so. A landing whose order nobody chose is worse than one that reports

Verify: `cargo nextest run --test verb_contracts`

## Unenforced rules

| Rule                                                    | Why no command decides it                                                                                                   |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `pending-fragment:a-fragment-requires-only-id-and-type` | Keeping the fragment schema's restatement of the entry fields in agreement with the lane schema is a review responsibility. |

Drift is a fact a fragment states that the record then outgrew. It is a dependency that closed, or a position target that is now another fragment, in another lane, closed, or gone. Rank is a claim, and only a person makes one.
