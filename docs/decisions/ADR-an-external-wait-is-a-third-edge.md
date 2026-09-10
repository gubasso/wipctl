# An external wait is a third edge

## Context and Problem Statement

Eligibility already derives from dependencies and open questions. A watch also stops named work, but the project does not own the action that clears it.

## Considered Options

- `a third eligibility edge` — chosen.
- `a blocked field` — rejected: the field duplicates facts already present in the record and drifts when a watch changes.
- `a dependency edge` — rejected: dependencies sequence work the record can order, while a watch names an outside action.

## Decision Outcome

Chosen option: `a third eligibility edge` — ranking derives eligibility from dependencies, questions, and watches without storing the answer.

Enforced by `ranking:eligibility-is-derived-from-three-edges`.

## Consequences

- Good: the scheduled head cannot appear startable while an outside action blocks it.
- Good: an in-flight row can show a wait that appeared after work started.
- Bad: this record ignores watches declared by a peer and relies on dependencies to peer entries.

## Status

Accepted
