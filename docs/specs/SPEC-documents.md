# Documents Specification

<!--TOC-->

- [Purpose](#purpose)
- [The three heading sequences](#the-three-heading-sequences)
- [Requirements](#requirements)
  - [`documents:a-title-and-its-id-agree` — A title and its id agree](#documentsa-title-and-its-id-agree--a-title-and-its-id-agree)
  - [`documents:every-heading-is-present-in-order` — Every heading is present, in order](#documentsevery-heading-is-present-in-order--every-heading-is-present-in-order)
  - [`documents:an-empty-heading-carries-none` — An empty heading carries the literal none](#documentsan-empty-heading-carries-none--an-empty-heading-carries-the-literal-none)
  - [`documents:an-example-shows-the-readers-side` — An example shows the reader's side](#documentsan-example-shows-the-readers-side--an-example-shows-the-readers-side)
  - [`documents:never-cut-work-is-not-negotiable-scope` — Never-cut work is not negotiable scope](#documentsnever-cut-work-is-not-negotiable-scope--never-cut-work-is-not-negotiable-scope)
  - [`documents:an-assertion-names-its-test` — An assertion names its test](#documentsan-assertion-names-its-test--an-assertion-names-its-test)
  - [`documents:an-escape-precedes-the-trap` — An escape precedes the trap](#documentsan-escape-precedes-the-trap--an-escape-precedes-the-trap)
  - [`documents:a-changed-agreement-is-a-revision` — A changed agreement is a revision](#documentsa-changed-agreement-is-a-revision--a-changed-agreement-is-a-revision)
  - [`documents:done-when-is-not-every-member-closed` — Done when is not every member closed](#documentsdone-when-is-not-every-member-closed--done-when-is-not-every-member-closed)
  - [`documents:the-initiative-section-carries-one-token` — The initiative section carries one token](#documentsthe-initiative-section-carries-one-token--the-initiative-section-carries-one-token)
  - [`documents:an-initiative-has-no-parent-section` — An initiative has no parent section](#documentsan-initiative-has-no-parent-section--an-initiative-has-no-parent-section)
  - [`documents:a-reference-opens-with-its-path` — A reference opens with its path](#documentsa-reference-opens-with-its-path--a-reference-opens-with-its-path)
  - [`documents:a-source-item-opens-with-its-reference` — A source item opens with its reference](#documentsa-source-item-opens-with-its-reference--a-source-item-opens-with-its-reference)
  - [`documents:a-promised-path-is-exempt-and-never-stubbed` — A promised path is exempt and never stubbed](#documentsa-promised-path-is-exempt-and-never-stubbed--a-promised-path-is-exempt-and-never-stubbed)
  - [`documents:a-rule-delta-is-typed-and-single` — A rule delta is typed and single](#documentsa-rule-delta-is-typed-and-single--a-rule-delta-is-typed-and-single)
  - [`documents:an-artifact-lives-beside-the-document` — An artifact lives beside the document](#documentsan-artifact-lives-beside-the-document--an-artifact-lives-beside-the-document)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

One document per story, epic, and initiative, and the shape each one carries. The filename stem is the id and the title line repeats it. The boundary runs at the document. This domain owns the headings, the references, and the artifacts beside them. The lane file domain owns the fields an entry carries in a lane.

## The three heading sequences

| Story          | Epic           | Initiative     |
| -------------- | -------------- | -------------- |
| —              | `Initiative`   | —              |
| `Goal`         | `Goal`         | `Goal`         |
| `Example`      | `Example`      | `Example`      |
| `Core`         | `Core`         | `Core`         |
| `In scope`     | —              | —              |
| `Out of scope` | `Out of scope` | `Out of scope` |
| `Reads`        | `Reads`        | `Reads`        |
| `Amends`       | `Amends`       | `Amends`       |
| `Sources`      | `Sources`      | `Sources`      |
| `Acceptance`   | —              | —              |
| —              | `Done when`    | `Done when`    |
| `Tasks`        | —              | —              |
| `Rabbit holes` | —              | —              |
| `Revisions`    | `Revisions`    | `Revisions`    |

Four story headings are deliberately absent above the story. Negotiable scope needs a point budget no epic has. Acceptance, tasks, and traps are obligations on a work session, and no session implements an epic or an initiative. The closing condition runs the other way: a story closes on its acceptance, and the tiers above it state an observable end state instead.

## Requirements

### `documents:a-title-and-its-id-agree` — A title and its id agree

A document's title line MUST slugify to the id, or to the id minus its qualifying postfix.

#### Scenario: A title is reworded without a rename

- GIVEN a document whose heading and filename drifted apart
- WHEN validation runs
- THEN it fails, because a document a reader cannot resolve to an entry is a curiosity and not a record

Verify: `cargo nextest run --test validation`

### `documents:every-heading-is-present-in-order` — Every heading is present, in order

Every document MUST carry its shape's headings, in the stated order, and MUST NOT add another.

#### Scenario: A story gains a closing section

- GIVEN a story with an extra heading after acceptance
- WHEN the shape gate runs
- THEN it is refused, because the story closes on its acceptance and a second closing heading gives two answers

Verify: `cargo nextest run --test documents`

### `documents:an-empty-heading-carries-none` — An empty heading carries the literal none

Where a heading has nothing to say, it MUST carry the literal `None`, except the task and revision lists, which MAY be empty.

#### Scenario: A story amends nothing

- GIVEN a story that changes no document
- WHEN the section is written
- THEN it carries the literal word, because an empty section reads as unfinished and the word reads as decided

Verify: `cargo nextest run --test documents`

### `documents:an-example-shows-the-readers-side` — An example shows the reader's side

A story or spike MUST carry a fenced block under its example heading, and the fence MUST declare a language.

#### Scenario: An example describes a transcript

- GIVEN a section that describes the output instead of showing it
- WHEN the gate runs
- THEN it fails, because a description of a transcript is not a transcript, and `text` is the declaration when no language applies

Verify: `cargo nextest run --test documents`

### `documents:never-cut-work-is-not-negotiable-scope` — Never-cut work is not negotiable scope

A story MUST NOT list correctness, tests, review, or security as negotiable scope.

#### Scenario: A story runs out of budget

- GIVEN negotiable scope cut from the bottom
- WHEN tests sit in that list
- THEN they are cut, so they belong to what is never cut

Verify: reviewer confirms the negotiable list holds no never-cut work

### `documents:an-assertion-names-its-test` — An assertion names its test

Every acceptance assertion MUST be falsifiable and MUST name the test that proves it.

#### Scenario: A story closes

- GIVEN a story whose assertions each name a test
- WHEN every named test passes and every counted judgment is made
- THEN the story closes, because the point value counts exactly the judgments no test can settle

Verify: `cargo nextest run --test documents`

### `documents:an-escape-precedes-the-trap` — An escape precedes the trap

Each known trap MUST carry a pre-authorised escape.

#### Scenario: A trap is hit mid-session

- GIVEN a trap whose escape is written afterwards
- WHEN the session records it
- THEN it is a revision, because an escape decided under pressure is a new agreement and not the old one

Verify: `cargo nextest run --test documents`

### `documents:a-changed-agreement-is-a-revision` — A changed agreement is a revision

When the agreement changes after a story's work began, the author MUST record a dated one-line revision.

#### Scenario: Planned negotiable scope is cut

- GIVEN a cut the agreement already allowed
- WHEN the session records it
- THEN no revision is owed, while a rename or any other change to the agreement does owe one

Verify: reviewer confirms each agreement change carries a dated revision line

### `documents:done-when-is-not-every-member-closed` — Done when is not every member closed

An epic or initiative MUST state an observable end state and MUST NOT state that every member is closed.

#### Scenario: A member is cut rather than delivered

- GIVEN an epic whose end state reads as every member closed
- WHEN one member is cut
- THEN the epic reads as done while delivering nothing, so it states what is observably true instead

Verify: `cargo nextest run --test documents`

### `documents:the-initiative-section-carries-one-token` — The initiative section carries one token

An epic's initiative section MUST carry exactly one inline-code id naming an existing initiative document, or the literal `None`.

#### Scenario: An initiative has no epics yet

- GIVEN an initiative no epic joined
- WHEN validation runs
- THEN it passes, because the reverse direction is a legal first draft while a dangling token is not

Verify: `cargo nextest run --test validation`

### `documents:an-initiative-has-no-parent-section` — An initiative has no parent section

An initiative document MUST NOT carry an initiative section.

#### Scenario: A fourth tier is attempted

- GIVEN an initiative naming a parent
- WHEN the shape gate runs
- THEN the extra heading is refused, so the ladder stops at three tiers with no separate check

Verify: `cargo nextest run --test documents`

### `documents:a-reference-opens-with-its-path` — A reference opens with its path

Each reference item MUST open with its path as an inline-code token, and an outbound path MUST resolve, relative, against the project root.

#### Scenario: An assertion mentions a second path

- GIVEN an item whose sentence names another file
- WHEN the references are read
- THEN only the leading token is a path, because everything after it is prose, and inbound paths are deliberately unchecked as a review responsibility

Verify: `cargo nextest run --test documents`

### `documents:a-source-item-opens-with-its-reference` — A source item opens with its reference

Each source item MUST open with one source reference as an inline-code token, in the alias-and-key shape the external sources domain states.

#### Scenario: A source item names the outside item and explains it

- GIVEN an item whose sentence quotes the outside item's own title
- WHEN the sources section is read
- THEN only the leading token is a reference, because everything after it is prose the record never parses

Verify: `cargo nextest run --test documents`

### `documents:a-promised-path-is-exempt-and-never-stubbed` — A promised path is exempt and never stubbed

Where an assertion promises a document the work will create, the path MUST be exempt from the existence check for the entry's whole life.

#### Scenario: A story promises a new specification

- GIVEN an outbound reference marked as promised
- WHEN validation runs
- THEN the existence check is skipped while every shape rule still applies, because an empty file passing a check teaches the wrong lesson

Verify: `cargo nextest run --test documents`

### `documents:a-rule-delta-is-typed-and-single` — A rule delta is typed and single

Where an outbound assertion carries a rule delta, it MUST open with the change type followed by exactly one rule id as an inline-code token.

#### Scenario: A story amends three rules of one document

- GIVEN one document with three changed rules
- WHEN the assertions are written
- THEN there are three list items, because one item carrying three ids states no per-rule type

Verify: `cargo nextest run --test documents`

### `documents:an-artifact-lives-beside-the-document` — An artifact lives beside the document

Where a story carries non-narrative artifacts, they MUST live in a sibling directory named for the id.

#### Scenario: A story carries a dataset

- GIVEN a measurement input the story depends on
- WHEN it is filed
- THEN the document narrates and the sibling directory holds, because prose is the wrong container for a dataset

Verify: `cargo nextest run --test validation`

## Unenforced rules

| Rule                                               | Why no command decides it                                                       |
| -------------------------------------------------- | ------------------------------------------------------------------------------- |
| `documents:never-cut-work-is-not-negotiable-scope` | Whether a listed item is never-cut work is a reading of the item.               |
| `documents:a-changed-agreement-is-a-revision`      | No command can tell a cut the agreement allowed from a change to the agreement. |

The shape gate proves that every required heading is present and no extra one was added. It cannot prove that a section was filled in, and it cannot express the example fence, the title-line identity, or the initiative token's resolution. Those belong to the cross-file checker.
