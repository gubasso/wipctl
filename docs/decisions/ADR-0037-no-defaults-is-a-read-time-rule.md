# ADR-0037: No defaults is a read-time rule

## Context and Problem Statement

ADR-0006 forbids config defaults, yet the scaffold writes `.wipctl.toml` with a start date it resolved (given or today's) and `length_days = 14` as its one baked value. The two statements read as a contradiction unless the moment the rule binds is stated.

## Considered Options

- Scope the no-defaults rule to read time: a verb finding a required key absent fails; the scaffold generates a complete file the operator owns
- Make the scaffold prompt for every value — turns one adoption question into a questionnaire, and the answers are still editable text a person reviews either way
- Strip the generated values and scaffold an incomplete config — the scaffold's own self-check would fail the zone it just emitted

## Decision Outcome

Chosen option: scope the rule to read time — the harm ADR-0006 names is a tool inventing a value for a key nobody wrote, and generation is not invention: the value lands in the file, visible, diffable, and owned by the operator from the moment it is written. A verb reading the config never supplies a value for an absent key; the scaffold emits every required key with a concrete value (the resolved `plan_dir`, the given or current start date, `length_days = 14`), and a gated invocation passes `--iteration-start` explicitly. The reference pages own the binding statements of both halves.

## Consequences

- Good: the scaffolded zone passes its own self-check immediately, and the read-time rule keeps every verb honest about absent keys.
- Bad: an operator who never reviews the generated cadence keeps a value they did not choose, knowingly accepted as the cost of a working first zone.

## Status

Accepted

Amends ADR-0006 — the no-defaults rule binds verbs reading the config, not the scaffold generating it.
