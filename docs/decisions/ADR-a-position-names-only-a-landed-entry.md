# A position names only a landed entry

## Context and Problem Statement

Two sessions in sequence is the ordinary case: the second wants to capture beneath what the first just captured, so the desired position target exists only as a fragment. May `after` name it?

## Considered Options

- `after` names a landed entry only; a fragment target is drift
- Resolving fragment-to-fragment positions — gives the drain a two-phase order, doubles every drift class (each rule must now consider a target that is itself pending, drifted, or withdrawn), and makes the landing order depend on resolution order
- Refusing the capture outright — punishes the capture for the record's state at drain time, which the capturing session cannot know

## Decision Outcome

Chosen option: a position names only a landed entry — the drain stays one-phase and its drift vocabulary stays small. A position whose target is another fragment is reported as drift, and the fragment lands at the bottom of its claimed lane, said out loud. The operator on the common path captures with `after: null` and ranks after the drain, or drains first and then captures.

## Consequences

- Good: the drain stays one-phase with a small drift vocabulary, and the repair's `--slots` report makes the post-drain ranking cheap.
- Bad: the common path pays an extra drain or a post-drain ranking, knowingly.

## Status

Accepted
