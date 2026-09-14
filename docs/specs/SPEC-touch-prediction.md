# Touch Prediction Specification

<!--TOC-->

- [Purpose](#purpose)
- [The field](#the-field)
- [The relation to the amended paths](#the-relation-to-the-amended-paths)
- [Requirements](#requirements)
  - [`touch-prediction:an-amended-path-is-a-touched-path` — An amended path is a touched path](#touch-predictionan-amended-path-is-a-touched-path--an-amended-path-is-a-touched-path)
  - [`touch-prediction:a-touch-list-is-stated-never-verified` — A touch list is stated, never verified](#touch-predictiona-touch-list-is-stated-never-verified--a-touch-list-is-stated-never-verified)
  - [`touch-prediction:a-touch-list-is-bounded-and-exact` — A touch list is bounded and exact](#touch-predictiona-touch-list-is-bounded-and-exact--a-touch-list-is-bounded-and-exact)
  - [`touch-prediction:a-prediction-never-sequences-and-never-ranks` — A prediction never sequences and never ranks](#touch-predictiona-prediction-never-sequences-and-never-ranks--a-prediction-never-sequences-and-never-ranks)
  - [`touch-prediction:a-touch-item-is-not-a-pattern` — A touch item is not a pattern](#touch-predictiona-touch-item-is-not-a-pattern--a-touch-item-is-not-a-pattern)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Two entries that will collide look exactly like two that will not. This domain gives every entry a bounded, imprecise list of the paths its work probably changes, so a reader sizing an epic sees its surface area and two agents see where they meet. The boundary runs at the prediction. This domain owns the field, its bound, and its relation to the outbound assertions a story document already carries. The lane file domain owns the row the field sits in. The documents domain owns the `Amends` section. The epics and initiatives domains own the union and the overlap set folded from the field.

A prediction is imprecise on purpose. It is a statement about what the author expects to change, made before the work starts, and nothing here checks it against a tree.

## The field

```yaml
touches: [src/http/limit.rs, src/http/mod.rs, docs/specs/SPEC-http.md]
```

```yaml
touches: []
```

Each item is a unique path, relative, carrying no dot segment and no whitespace, resolving against the host's project root exactly as an `Amends` path does. It predicts a file the work will create, modify, or remove. Reading or opening a file does not put it in the list. Existence is never checked, so a path that does not exist yet is ordinary, because a story can create it.

The empty flow sequence is the decided form for a story that changes no file. It says that the author considered the question and predicted nothing. The key is required on every lane entry, so an absent key is an incomplete entry rather than a second meaning. A textual sentinel such as `none` states nothing the empty sequence does not.

The list is a one-line flow sequence, because `lane-file:the-yaml-subset-is-narrow` admits nothing else. It carries at most 12 paths. That bound limits the coordination surface a reader compares. It is not a complexity measure, and it changes no point value. A story predicting more than 12 changed paths splits along its independently reviewable outcomes, or narrows its stated scope, before it enters a lane.

## The relation to the amended paths

A story document's `Amends` section is already a gated, file-precise list of paths the work changes. A free-standing touch list would be a second store of that fact, which the charter refuses. The two are not the same list: `Amends` is an obligation and a subset, and the other files the work changes are what can collide and are recorded nowhere else.

The resolution is containment rather than independence. Every path in a story's `Amends` section appears in that entry's touch list. One rule decides it, and the two lists can never disagree.

The checker reads each outbound item's leading inline-code token, removes the `new:` promise marker where the item carries one, and requires the resulting exact path in `touches`. The check spans a document and a lane file, so it belongs to the cross-file half rather than to either schema.

## Requirements

### `touch-prediction:an-amended-path-is-a-touched-path` — An amended path is a touched path

Where a story carries an outbound assertion, that assertion's path MUST appear in its entry's touch list.

#### Scenario: An author adds a second amended document

- GIVEN an entry whose touch list matches its one existing assertion
- WHEN the author adds a second `Amends` item and leaves the lane entry alone
- THEN the cross-file check fails naming both files, because one fact recorded twice disagrees on the first edit

Verify: `cargo nextest run --test validation`

### `touch-prediction:a-touch-list-is-stated-never-verified` — A touch list is stated, never verified

The implementation MUST accept each touch path as opaque past its shape, and MUST read it as a predicted change rather than as a file the work opens.

#### Scenario: A story predicts a file it will create

- GIVEN a touch list naming a path no repository holds yet
- WHEN validation runs
- THEN it passes, because the record cannot see the host's tree and a prediction that had to exist already could never name new work

Verify: `cargo nextest run --test validation`

### `touch-prediction:a-touch-list-is-bounded-and-exact` — A touch list is bounded and exact

A touch list MUST carry at most 12 unique exact paths, and an empty sequence MUST mean the prediction that no file changes.

#### Scenario: An entry predicts twenty changed files

- GIVEN work reaching across twenty paths
- WHEN the author writes them all
- THEN the schema refuses the list, and the entry splits along its outcomes, because a comparison surface nobody reads coordinates nothing

Verify: `cargo nextest run --test schemas`

### `touch-prediction:a-prediction-never-sequences-and-never-ranks` — A prediction never sequences and never ranks

An overlap between two touch lists MUST be reported as advisory, and MUST NOT gate a transition, add a dependency edge, or enter the order computation.

#### Scenario: Two open entries predict one file

- GIVEN two entries whose lists share a path
- WHEN the board and the epic detail render
- THEN the overlap is shown and neither entry moves, because `lane-file:needs-is-the-only-sequencing-fact` owns sequencing and the charter refuses a priority a machine authors

Verify: `cargo nextest run --test ranking`

### `touch-prediction:a-touch-item-is-not-a-pattern` — A touch item is not a pattern

Each touch item MUST be one exact path, and a broader surface MUST be stated as exact paths or split before the bound rather than as a glob or a directory prefix.

#### Scenario: An author reaches for a directory

- GIVEN work across a whole source directory
- WHEN the author writes that directory as one item
- THEN the schema refuses it, because exact paths give a deterministic overlap set and a pattern language brings containment questions and false precision

Verify: `cargo nextest run --test schemas`

## Unenforced rules

| Rule                                                     | Why no command decides it                                                                           |
| -------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `touch-prediction:a-touch-list-is-stated-never-verified` | Whether a listed path is a file the work changes or one it merely reads is a reading of the intent. |
