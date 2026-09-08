# An initiative is reported, never executed

## Context and Problem Statement

`epic <id>` resolves an epic into an execution plan. The obvious symmetry is `initiative <id>` doing the same over every member epic's members — and the symmetry is wrong for the method's own reasons.

## Considered Options

- Decompose only: one row per member epic, with the entry-level plan staying the epic verb's answer
- Full resolution across two membership hops — an eligible set spanning four epics invites a session to take work from an end state nobody is currently pursuing, and pays the traversal cost ADR-an-epic-is-a-field-and-a-document warned about
- A flag choosing between the two — a flag that changes the return shape is two verbs wearing one name

## Decision Outcome

Chosen option: decompose only. A session works one end state at a time, so the entry-level execution plan has exactly one home. `initiative <id>` reports one row per member epic, each carrying the epic's own arithmetic; the rollup and the listing mirror the epic pair under the pairing rule of ADR-one-verb-resolves-one-epic-and-another-summarises-them-all. Membership is searched from the epic documents' `Initiative` sections; the initiative document holds no member list. An initiative no epic has joined appears at zero rather than being omitted. Point arithmetic is shared with `epic` and `epics` — one implementation computes all three tiers, so a number cannot appear twice with two values. The rollup reports no staleness verdict and no retirement advice; retirement is a review question at every tier. Everything is derived on every call and stored nowhere.

## Consequences

- Good: no new eligibility surface, no new ranking surface, and the dashboard grows no panel.
- Bad: a reader who wants every startable entry under an initiative runs the epic verb once per member epic — the initiative verb's output is the list to run it against.

## Status

Accepted
