# The method and the tooling share one licence

## Context and Problem Statement

Two deliverables ship here and are judged as one: a planning method, and the tooling that proves a plan record obeys it. The tooling carries a dual code licence. The method is prose, and prose is where a project reaches for a content licence instead. Without one answer, a method rule quoted into a design document is a separate licensing question every time.

## Considered Options

- One dual code licence over both halves.
- A content licence over the method and a code licence over the distribution.
- A dual licence with no stated grant for a contribution.

## Decision Outcome

Chosen option: `one dual code licence over both halves` — the scaffold copies method text into a target project as that project's own file, and the project edits it at once. Attribution and change-marking obligations attach badly to a file whose purpose is rewriting in place, and the scaffold cannot carry them forward. The split was rejected for that reason. The third option was rejected because a contribution then arrives under no stated terms, so the README states the grant.

The canon this project installs its documentation method from splits the two. Its method is read and cited rather than copied into a target repository and edited there. Different artifact, different answer.

Enforced by `quality-gates:the-published-artifact-carries-its-licence`.

## Consequences

- Good: a method rule quoted into a document raises no separate licensing question.
- Good: no per-file licence boundary to assert in prose and gate with nothing.
- Bad: the method text carries no attribution obligation, so a fork restates it without credit.
- Bad: the writing pattern vendored by the documentation instance stays a second set of terms in the tree, and a packager reads it separately.

## Status

Accepted
