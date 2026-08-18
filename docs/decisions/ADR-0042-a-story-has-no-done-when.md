# ADR-0042: A story has no Done when

## Context and Problem Statement

A story carried twelve required headings, and two of them stated the same fact: `Acceptance` holds falsifiable assertions, each naming its test, and `Done when` restated the closing condition in prose. Two statements of one condition drift, and every story paid a heading whose honest content was "all of the above pass".

## Considered Options

- Remove `Done when` from stories and keep it on epics.
- Keep both headings and let review police the redundancy.
- Remove `Acceptance` and make `Done when` the single closing statement.

## Decision Outcome

Chosen option: `remove Done when from stories and keep it on epics` — a story's closing condition is its `Acceptance` section: the story closes when every assertion's named test passes and every counted judgment is made. A prose restatement beside it is a second owner of the same fact.

The epic keeps `Done when` because there it is not a restatement: an epic has no `Acceptance`, no session implements it, and a cut member closes without delivering, so the epic needs an observable end state that is deliberately not "every member closed". Removing `Acceptance` instead would trade falsifiable assertions for prose, which is backwards.

The story sequence is now eleven headings, and `Done when` joins the headings deliberately absent from a story for the inverse of the epic's reason.

## Consequences

- Good: one owner for a story's closing condition, and one less heading to fill on every story.
- Good: the asymmetry teaches the difference between the two documents: assertions close a story, an end state closes an epic.
- Bad: a closing condition that spans assertions — an ordering, a demo, a rollout step — must now be written as an assertion with a named test, which takes more care than a sentence did.
- Bad: existing story documents and the gated headings array must change together.

## Status

Accepted
