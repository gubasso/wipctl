# Open Questions Specification

## Purpose

The zone file holding the questions that block named work. A question earns its place by blocking. The boundary runs at the edge: this domain owns the question's shape and its blocking claim, while the ranking domain owns what the eligibility predicate does with that claim.

## Section shape

```markdown
## Q-may-a-position-name-a-fragment — May a position name a fragment?

Raised: 2026-08-14

Blocks: land-the-capture-queue, drain-reports-drift — the drain's ordering rule depends on the answer.

Exit: a decision record, measured against a worked two-session capture.
```

## Requirements

### `open-questions:a-question-blocks-something` — A question blocks something

Every question section MUST carry a blocking line naming at least one entry id.

#### Scenario: A question blocks nothing

- GIVEN a question raised out of curiosity
- WHEN validation runs
- THEN it fails, because a question that blocks nothing belongs in a drafts workspace and not in the record

Verify: `cargo nextest run --test validation`

### `open-questions:a-blocked-id-exists` — A blocked id exists

Every id named on a blocking line MUST exist in some lane.

#### Scenario: A blocked entry is deleted

- GIVEN a question naming an id that no longer exists
- WHEN validation runs
- THEN it fails, because a question blocking nothing that exists blocks nothing at all

Verify: `cargo nextest run --test validation`

### `open-questions:a-blocked-id-is-not-underway` — A blocked id is not underway

A blocked id MUST NOT already be in the in-flight or review lane.

#### Scenario: Work starts under an open question

- GIVEN a question naming an entry already in flight
- WHEN validation runs
- THEN it fails, because either the question is stale or the entry moved illegally, and both need a person

Verify: `cargo nextest run --test validation`

### `open-questions:a-stale-question-is-reported` — A stale question is reported

At least one blocked id MUST still be open, and the implementation MUST report a question whose blocked entries have all closed.

#### Scenario: The last blocked entry closes

- GIVEN a question whose every blocked entry is finished
- WHEN validation runs
- THEN the question is reported as stale, because nothing is waiting on the answer any more

Verify: `cargo nextest run --test validation`

### `open-questions:a-section-carries-its-four-parts` — A section carries its four parts

Every question section MUST carry the question id and text in its heading, the date it was raised, its blocking line, and what closes it.

#### Scenario: A question states no exit

- GIVEN a question with no closing condition
- WHEN a reader asks how it ends
- THEN nothing answers, so the exit names a decision, a story revision, or a measurement, in whatever form the project records

Verify: `cargo nextest run --test documents`

### `open-questions:blocking-is-derived-never-stored` — Blocking is derived, never stored

The implementation MUST derive blocking from the question edges and the dependency edges, and MUST NOT store it as a lane or a field.

#### Scenario: A blocked flag is proposed

- GIVEN a request to mark blocked entries in the lane files
- WHEN the two edges already answer it
- THEN the flag is refused, because a stored copy of a derived fact drifts on the first edit to either edge

Verify: `cargo nextest run --test validation`
