# A touch list contains the amended paths

## Context and Problem Statement

Nothing in the record says which files a story is likely to reach, so two entries that will collide look exactly like two that will not. A story already names the documents its work must leave changed, in its `Amends` section, and a second list of changed paths would store one fact twice.

## Considered Options

- `two fields with a containment rule` — chosen.
- `two independent lists` — rejected: the charter refuses a second store, and the two disagree on the first edit.
- `a touch list alone, with Amends deleted` — rejected: an assertion carries a per-path obligation, a promise marker, and an optional typed rule delta a flat path list cannot express.
- `Amends alone, with no touch list` — rejected: the obligation set is a subset, and the other changed files are the ones that collide.
- `a glob or a directory prefix as a touch item` — rejected: a pattern language brings containment questions and false precision the first version does not need.
- `a touch count feeding rank or points` — rejected: the charter refuses a priority a machine authors, and points count judgments rather than files.

## Decision Outcome

Chosen option: `two fields with a containment rule` — every path in a story's `Amends` section appears in that entry's touch list, so one rule decides the relation and the two can never disagree. The list holds at most 12 unique exact paths, and an empty sequence predicts that no file changes. The checker reads each assertion's leading path, removes the `new:` marker where present, and requires that exact path in the list. The check spans a document and a lane file, so the cross-file half owns it.

Enforced by `touch-prediction:an-amended-path-is-a-touched-path`.

## Consequences

- Good: a reader sees an entry's surface area and any overlap with another entry, with no second store to keep true.
- Bad: an author writes the amended paths twice, and the cross-file check keeps them in step rather than the file itself.

## Status

Accepted
