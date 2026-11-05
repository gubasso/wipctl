# Lane File Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`lane-file:the-file-is-the-lane` — The file is the lane](#lane-filethe-file-is-the-lane--the-file-is-the-lane)
  - [`lane-file:an-id-is-unique-across-the-record` — An id is unique across the record](#lane-filean-id-is-unique-across-the-record--an-id-is-unique-across-the-record)
  - [`lane-file:the-points-scale-counts-judgments` — The points scale counts judgments](#lane-filethe-points-scale-counts-judgments--the-points-scale-counts-judgments)
  - [`lane-file:delay-cost-records-an-exception` — Delay cost records an exception](#lane-filedelay-cost-records-an-exception--delay-cost-records-an-exception)
  - [`lane-file:absent-delay-cost-is-ordinary` — Absent delay cost is ordinary](#lane-fileabsent-delay-cost-is-ordinary--absent-delay-cost-is-ordinary)
  - [`lane-file:a-summary-is-one-bounded-line` — A summary is one bounded line](#lane-filea-summary-is-one-bounded-line--a-summary-is-one-bounded-line)
  - [`lane-file:needs-is-the-only-sequencing-fact` — Dependencies are the only stored sequencing constraint](#lane-fileneeds-is-the-only-sequencing-fact--dependencies-are-the-only-stored-sequencing-constraint)
  - [`lane-file:a-dependency-may-name-a-peer-entry` — A dependency may name a peer entry](#lane-filea-dependency-may-name-a-peer-entry--a-dependency-may-name-a-peer-entry)
  - [`lane-file:a-prefixed-dependency-is-quoted` — A prefixed dependency is quoted](#lane-filea-prefixed-dependency-is-quoted--a-prefixed-dependency-is-quoted)
  - [`lane-file:an-entry-names-its-epic-and-nothing-higher` — An entry names its epic and nothing higher](#lane-filean-entry-names-its-epic-and-nothing-higher--an-entry-names-its-epic-and-nothing-higher)
  - [`lane-file:a-code-reference-is-stated-never-verified` — A code reference is stated, never verified](#lane-filea-code-reference-is-stated-never-verified--a-code-reference-is-stated-never-verified)
  - [`lane-file:a-close-field-appears-only-when-closed` — A close field appears only when closed](#lane-filea-close-field-appears-only-when-closed--a-close-field-appears-only-when-closed)
  - [`lane-file:a-reshaped-close-names-its-successor` — A reshaped close names its successor](#lane-filea-reshaped-close-names-its-successor--a-reshaped-close-names-its-successor)
  - [`lane-file:the-yaml-subset-is-narrow` — The accepted YAML subset is narrow](#lane-filethe-yaml-subset-is-narrow--the-accepted-yaml-subset-is-narrow)
  - [`lane-file:the-closed-lane-is-not-a-terminus` — The closed lane is not a terminus](#lane-filethe-closed-lane-is-not-a-terminus--the-closed-lane-is-not-a-terminus)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

One file per lane, and the fields an entry carries. The file is the lane, so there is no status field. The ranking domain computes order from the record. The boundary runs at the file: the shape schema owns each file's own structure, and the cross-file checker owns everything spanning two files.

## Requirements

### `lane-file:the-file-is-the-lane` — The file is the lane

A lane file MUST name the lane matching its basename and MUST declare an empty lane explicitly.

#### Scenario: A status field is proposed

- GIVEN a request for a status or priority field
- WHEN the file answers lane membership and the ranking procedure answers rank
- THEN each field is refused, because it would store a fact that already has one source

Verify: `cargo nextest run --test schemas`

### `lane-file:an-id-is-unique-across-the-record` — An id is unique across the record

An entry id MUST be unique across every lane, every epic, every initiative, and every tombstone.

#### Scenario: A burned id is reused

- GIVEN an id freed by a rename and burned by a tombstone
- WHEN a new entry claims it
- THEN the claim fails, because an old reference then resolves to new work

Verify: `cargo nextest run --test validation`

### `lane-file:the-points-scale-counts-judgments` — The points scale counts judgments

An entry MUST carry a point value of `1`, `2`, or `3`, and larger work MUST split along its named judgments.

#### Scenario: An entry needs four points

- GIVEN work carrying three judgments and a hard-to-reverse choice
- WHEN the author reaches for a fourth point
- THEN there is no such value, and the acceptance criteria already name the split that bounds its wait

Verify: `cargo nextest run --test schemas`, and a reviewer confirms that each value names the judgments of its work

### `lane-file:delay-cost-records-an-exception` — Delay cost records an exception

An entry MAY carry `delay_cost` as `immediate` or `deferred`, ordered before and after absence respectively.

#### Scenario: An entry carries an ordinary value

- GIVEN an entry whose `delay_cost` is `ordinary`
- WHEN the lane schema reads it
- THEN validation fails because ordinary work records no exceptional value

Verify: `cargo nextest run --test schemas`

### `lane-file:absent-delay-cost-is-ordinary` — Absent delay cost is ordinary

When an entry carries no `delay_cost`, the ranking procedure MUST compare it after `immediate` and before `deferred` without adding a value.

#### Scenario: Ordinary work is compared

- GIVEN an entry without `delay_cost` and another whose delay cost is deferred
- WHEN the delay-cost preference compares them
- THEN the entry without the field sorts first and both entries remain unchanged

Verify: `cargo nextest run --test ranking`

### `lane-file:a-summary-is-one-bounded-line` — A summary is one bounded line

An entry's summary MUST be one double-quoted line of 60 to 400 characters with no leading or trailing whitespace.

#### Scenario: A summary grows into a specification

- GIVEN a summary restating the whole story
- WHEN the bound is applied
- THEN it is cut back to a restatement for a reader scanning the lane. The note field says how the entry is handled, and the summary says what the work is

Verify: `cargo nextest run --test schemas`, and a reviewer confirms that the line still describes its story

### `lane-file:needs-is-the-only-sequencing-fact` — Dependencies are the only stored sequencing constraint

The record MUST store hard sequencing constraints only as an entry's dependency list.

#### Scenario: A second ordering field is proposed

- GIVEN a request for a phase or milestone constraint
- WHEN the dependency list already expresses it
- THEN the field is refused, because two constraint stores disagree on the first edit

Verify: `cargo nextest run --test validation`

### `lane-file:a-dependency-may-name-a-peer-entry` — A dependency may name a peer entry

A `needs` id MAY carry an alias prefix, `<alias>#<id>`, naming an entry in the peer that the peer table binds to that alias, and the implementation MUST treat both forms as one edge: closed before this entry is eligible.

#### Scenario: A list mixes both forms

- GIVEN `needs: [session-token-parsing, "payments#secure-session-storage"]`
- WHEN eligibility, the graph, and the acyclicity proof read the entry
- THEN each treats the two ids identically, because they are the same edge

Verify: `cargo nextest run --test validation`

### `lane-file:a-prefixed-dependency-is-quoted` — A prefixed dependency is quoted

A prefixed dependency MUST be written double-quoted, in a lane file and in a fragment, and the canonical-subset parser MUST hold that rule.

#### Scenario: A schema is proposed as the place to check it

- GIVEN a source line whose prefixed id carries no quotes
- WHEN the schema runs
- THEN it passes, because a schema reads a value the parser already unquoted. One form on the page, one form from every writer, and one form for a reviewer is what the rule buys

Verify: `cargo nextest run --test validation`

### `lane-file:an-entry-names-its-epic-and-nothing-higher` — An entry names its epic and nothing higher

Where an entry declares membership, it MUST name one existing epic document and MUST NOT name anything above the epic.

#### Scenario: An entry claims an initiative directly

- GIVEN an entry naming an initiative
- WHEN validation runs
- THEN it fails, because membership above the story flows through the epic and the epic document makes that claim

Verify: `cargo nextest run --test validation`

### `lane-file:a-code-reference-is-stated-never-verified` — A code reference is stated, never verified

The implementation MUST accept the branch and delivered fields as opaque beyond their shape.

#### Scenario: A branch is deleted after merge

- GIVEN a closed entry naming a branch that no longer exists
- WHEN validation runs
- THEN the field passes, because the record cannot verify another repository and a check that guesses is worse than none

Verify: `cargo nextest run --test validation`

### `lane-file:a-close-field-appears-only-when-closed` — A close field appears only when closed

An entry MUST carry an outcome and a close date in the closed lane, and MUST NOT carry either anywhere else.

#### Scenario: An entry is moved back out of the closed lane

- GIVEN a closed entry returning to an open lane
- WHEN the move completes
- THEN the close fields are gone, because a lane with a close date in it is a record two readers will read differently

Verify: `cargo nextest run --test validation`

### `lane-file:a-reshaped-close-names-its-successor` — A reshaped close names its successor

An entry closed as reshaped MUST name one successor id, and an entry closed otherwise MUST NOT name one.

#### Scenario: Work is re-cut into a different story

- GIVEN an entry closed because its shape changed
- WHEN the successor is written
- THEN the closed entry points at it, so the trail from the old id does not end

Verify: `cargo nextest run --test validation`

### `lane-file:the-yaml-subset-is-narrow` — The accepted YAML subset is narrow

The implementation MUST accept only flat mappings, one level of entry nesting, and one-line flow sequences, and MUST reject every other YAML construct.

#### Scenario: The move verb relocates an entry block

- GIVEN a lane file with comments and blank lines
- WHEN the move verb moves an entry
- THEN the block moves byte for byte and the comments survive, because nothing is ever re-serialised

Verify: `cargo nextest run --test schemas`

### `lane-file:the-closed-lane-is-not-a-terminus` — The closed lane is not a terminus

An entry MAY leave the closed lane, and a gap left by a departure MUST NOT be treated as a defect.

#### Scenario: Work reopens after review

- GIVEN a closed entry that turns out to be unfinished
- WHEN it moves back
- THEN the move is legal and recorded, and a later close places the entry by its new close date

Verify: `cargo nextest run --test validation`

## Unenforced

| Rule                                          | Reviewer confirms                                                                                                                                |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `lane-file:the-points-scale-counts-judgments` | each value names the judgments of its work, because whether a value names its judgments is a reading of the work, not a match against the number |
| `lane-file:a-summary-is-one-bounded-line`     | the line still describes its story, because the bound is gated and that description is not                                                       |
