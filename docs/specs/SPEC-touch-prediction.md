# Touch Prediction Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`touch-prediction:an-amended-path-is-a-touched-path` — An amended path is a touched path](#touch-predictionan-amended-path-is-a-touched-path--an-amended-path-is-a-touched-path)
  - [`touch-prediction:a-touch-list-is-stated-never-verified` — A touch list is stated, never verified](#touch-predictiona-touch-list-is-stated-never-verified--a-touch-list-is-stated-never-verified)
  - [`touch-prediction:a-touch-list-is-bounded-and-exact` — A touch list is bounded and exact](#touch-predictiona-touch-list-is-bounded-and-exact--a-touch-list-is-bounded-and-exact)
  - [`touch-prediction:a-prediction-never-sequences-and-never-ranks` — A prediction never sequences and never ranks](#touch-predictiona-prediction-never-sequences-and-never-ranks--a-prediction-never-sequences-and-never-ranks)
  - [`touch-prediction:a-touch-item-is-not-a-pattern` — A touch item is not a pattern](#touch-predictiona-touch-item-is-not-a-pattern--a-touch-item-is-not-a-pattern)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

Two entries that will collide look exactly like two that will not. This domain gives every entry a bounded, imprecise list of the paths its work probably changes, so a reader sizing an epic sees its surface area and two agents see where they meet. The boundary runs at the prediction. This domain owns the field, its bound, and its relation to the outbound assertions a story document already carries. The lane file domain owns the row the field sits in. The documents domain owns the `Amends` section. The epics and initiatives domains own the union and the overlap set folded from the field.

A prediction is imprecise on purpose. It is a statement about what the author expects to change, made before the work starts, and nothing here checks it against a tree.

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

Verify: `cargo nextest run --test validation`, and a reviewer confirms that each listed path is a file the work changes and not one it merely reads

### `touch-prediction:a-touch-list-is-bounded-and-exact` — A touch list is bounded and exact

A touch list MUST carry at most 12 unique exact paths, each relative, with no dot segment and no whitespace, resolving against the host's project root as an `Amends` path does, and an empty sequence MUST mean the prediction that no file changes.

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

## Unenforced

| Rule                                                     | Reviewer confirms                                                                                                           |
| -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `touch-prediction:a-touch-list-is-stated-never-verified` | each listed path is a file the work changes and not one it merely reads, because which one it is is a reading of the intent |
