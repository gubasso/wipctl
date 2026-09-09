# Lane File Specification

<!--TOC-->

- [Purpose](#purpose)
- [File shape](#file-shape)
- [Entry fields](#entry-fields)
- [A dependency that crosses plans](#a-dependency-that-crosses-plans)
- [Lane semantics](#lane-semantics)
- [Requirements](#requirements)
  - [`lane-file:the-file-is-the-lane` — The file is the lane and the position is the rank](#lane-filethe-file-is-the-lane--the-file-is-the-lane-and-the-position-is-the-rank)
  - [`lane-file:an-id-is-unique-across-the-record` — An id is unique across the record](#lane-filean-id-is-unique-across-the-record--an-id-is-unique-across-the-record)
  - [`lane-file:the-points-scale-counts-judgments` — The points scale counts judgments](#lane-filethe-points-scale-counts-judgments--the-points-scale-counts-judgments)
  - [`lane-file:a-summary-is-one-bounded-line` — A summary is one bounded line](#lane-filea-summary-is-one-bounded-line--a-summary-is-one-bounded-line)
  - [`lane-file:needs-is-the-only-sequencing-fact` — Dependencies are the only sequencing fact](#lane-fileneeds-is-the-only-sequencing-fact--dependencies-are-the-only-sequencing-fact)
  - [`lane-file:a-prefixed-dependency-is-quoted` — A prefixed dependency is quoted](#lane-filea-prefixed-dependency-is-quoted--a-prefixed-dependency-is-quoted)
  - [`lane-file:an-entry-names-its-epic-and-nothing-higher` — An entry names its epic and nothing higher](#lane-filean-entry-names-its-epic-and-nothing-higher--an-entry-names-its-epic-and-nothing-higher)
  - [`lane-file:a-code-reference-is-stated-never-verified` — A code reference is stated, never verified](#lane-filea-code-reference-is-stated-never-verified--a-code-reference-is-stated-never-verified)
  - [`lane-file:a-close-field-appears-only-when-closed` — A close field appears only when closed](#lane-filea-close-field-appears-only-when-closed--a-close-field-appears-only-when-closed)
  - [`lane-file:a-reshaped-close-names-its-successor` — A reshaped close names its successor](#lane-filea-reshaped-close-names-its-successor--a-reshaped-close-names-its-successor)
  - [`lane-file:the-yaml-subset-is-narrow` — The accepted YAML subset is narrow](#lane-filethe-yaml-subset-is-narrow--the-accepted-yaml-subset-is-narrow)
  - [`lane-file:the-closed-lane-is-not-a-terminus` — The closed lane is not a terminus](#lane-filethe-closed-lane-is-not-a-terminus--the-closed-lane-is-not-a-terminus)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

One file per lane, and the fields an entry carries. The file is the lane, so there is no status field, and the sequence position is the ranking, so there is no priority field. The boundary runs at the file: the shape schema owns each file's own structure, and the cross-file checker owns everything spanning two files.

## File shape

```yaml
lane: todo
stories:
  - id: rate-limit-the-search-endpoint
    type: story
    points: 2
    summary: "Callers of the search endpoint are limited per token, and the limit is announced in the response headers rather than discovered by being cut off."
    needs: [secure-session-storage]
    epic: session-hardening
```

## Entry fields

| Field          | Requirement            | Meaning                                                                                                 |
| -------------- | ---------------------- | ------------------------------------------------------------------------------------------------------- |
| `id`           | required               | the title's slug; the story document's filename stem                                                    |
| `type`         | required               | `story`, `spike`, or `chore`                                                                            |
| `points`       | required               | `1`, `2`, or `3`, counting judgments                                                                    |
| `summary`      | required               | one double-quoted line, 60 to 400 characters, no surrounding whitespace                                 |
| `needs`        | optional               | one-line flow sequence of unique ids this entry depends on, each naming this record or an attached peer |
| `epic`         | optional               | one id naming an existing epic document                                                                 |
| `tags`         | optional               | one-line flow sequence of unique slugs; nothing gates on a tag                                          |
| `branch`       | optional               | the code branch carrying the work                                                                       |
| `delivered`    | closed lane only       | the commit that delivered the work                                                                      |
| `outcome`      | required when closed   | `done`, `cut`, or `reshaped`                                                                            |
| `closed`       | required when closed   | the close date, and the velocity source                                                                 |
| `succeeded_by` | required when reshaped | one id                                                                                                  |
| `note`         | optional               | free prose about handling; nothing parses it                                                            |

Points read as follows. `1` is confirmation only: named tests settle acceptance, and at most one unchanged contract is involved. `2` is one judgment: an interface, name, message, or existing contract needs human reasoning. `3` is two judgments, or one that is hard to reverse, such as a schema or a security control.

## A dependency that crosses plans

A `needs` id MAY carry an alias prefix, `<alias>#<id>`, naming an entry in the peer that the peer table binds to that alias. Both forms mean one thing: closed before this entry is eligible. Eligibility, the graph, and the acyclicity proof treat them identically, because they are the same edge.

```yaml
- id: profile-composition
  type: story
  points: 2
  summary: "Compose a profile from the session identity the payments plan stores, so that a signed-in reader sees their own name."
  needs: [session-token-parsing, "payments#secure-session-storage"]
```

The example mixes both forms in one list on purpose. They sit side by side, and nothing distinguishes them past resolution.

```text
bare id       the five lane files of this record

alias#id      1. the alias resolves in the peers section to a uid
              2. the uid resolves in the attachment registry to a slot
              3. the id resolves in that slot's five lane files, read at
                 the tree of its current commit
```

A prefixed id names a lane entry and nothing else, exactly as a bare id does. A dependency on a peer's whole epic is written as a dependency on that epic's last entry, which is what it means.

The separator is `#`. A comment marker in this format must be separated from what precedes it, so `payments#secure-session-storage` is one plain scalar and is never a comment. The quoting rule below is a choice about how the record reads, not a rescue from a parse error.

## Lane semantics

| Lane      | Meaning                                                               |
| --------- | --------------------------------------------------------------------- |
| `backlog` | agreed, not scheduled                                                 |
| `todo`    | scheduled; the topmost entry is what to start                         |
| `doing`   | in flight; every dependency closed and no blocking question           |
| `review`  | waiting on a reader; the same entry gate as in flight                 |
| `closed`  | finished, append-ordered by close date ascending, stable within a day |

## Requirements

### `lane-file:the-file-is-the-lane` — The file is the lane and the position is the rank

A lane file MUST name the lane matching its basename, MUST order its entries highest priority first, and MUST declare an empty lane explicitly.

#### Scenario: A status field is proposed

- GIVEN a request for a status or priority field
- WHEN the file already answers both
- THEN the field is refused, because two places holding one fact disagree on the first edit

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
- THEN there is no such value, and the acceptance criteria already name the split

Verify: `cargo nextest run --test schemas`

### `lane-file:a-summary-is-one-bounded-line` — A summary is one bounded line

An entry's summary MUST be one double-quoted line of 60 to 400 characters with no leading or trailing whitespace.

#### Scenario: A summary grows into a specification

- GIVEN a summary restating the whole story
- WHEN the bound is applied
- THEN it is cut back to a restatement for a reader scanning the lane. The note field says how the entry is handled, and the summary says what the work is

Verify: `cargo nextest run --test schemas`

### `lane-file:needs-is-the-only-sequencing-fact` — Dependencies are the only sequencing fact

The record MUST store sequencing only as an entry's dependency list.

#### Scenario: A second ordering field is proposed

- GIVEN a request for a phase or a milestone order
- WHEN the dependency list already expresses it
- THEN the field is refused, because two orderings disagree the moment either moves

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

#### Scenario: The repair relocates an entry block

- GIVEN a lane file with comments and blank lines
- WHEN the repair moves an entry
- THEN the block moves byte for byte and the comments survive, because nothing is ever re-serialised

Verify: `cargo nextest run --test schemas`

### `lane-file:the-closed-lane-is-not-a-terminus` — The closed lane is not a terminus

An entry MAY leave the closed lane, and a gap left by a departure MUST NOT be treated as a defect.

#### Scenario: Work reopens after review

- GIVEN a closed entry that turns out to be unfinished
- WHEN it moves back
- THEN the move is legal and recorded, and a close date out of sequence is a warning the repair resolves

Verify: `cargo nextest run --test validation`

## Unenforced rules

| Rule                                          | Why no command decides it                                                                     |
| --------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `lane-file:the-points-scale-counts-judgments` | Whether a value names its judgments is a reading of the work, not a match against the number. |
| `lane-file:a-summary-is-one-bounded-line`     | The bound is gated; whether the line still describes its story is not.                        |
