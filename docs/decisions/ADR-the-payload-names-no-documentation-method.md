# The payload names no documentation method

## Context and Problem Statement

ADR-the-host-stays-unassumed settled that the host stays unassumed: this method requires a documentation directory and asserts nothing else about it. That doctrine is stated in prose and checked by nothing, and prose drifts — a page gains an example naming one method, a scaffolded `AGENTS.md` gains a sentence about specifications, and the assumption is back without any decision having been taken. The typed rule delta of ADR-an-amends-item-may-carry-a-typed-rule-delta makes the risk concrete, because it borrows an id grammar from hosts that keep requirement-level specifications and could be read as adopting the method those hosts run.

## Considered Options

- A test over the shipped payload, denying documentation-method product names.
- A review checklist item.
- A gate over every page in `docs/`.
- Leave ADR-the-host-stays-unassumed stated and unchecked.

## Decision Outcome

Chosen option: `a test over the shipped payload` — the check denies the names of documentation-method products and their tooling, and nothing else. Generic convention that many methods share, such as a `SPEC-` or `ADR-` filename in an illustration, stays legal, because the reference must be able to show what an `Amends` item looks like against a host that keeps specifications.

The scope is the payload rather than every page, since the payload is what reaches an adopting project. A checklist item was rejected as the weaker form of a check a command can make; a gate over all of `docs/` would reject the illustrations the reference needs.

## Consequences

- Good: ADR-the-host-stays-unassumed's doctrine fails a test rather than a reading when it breaks.
- Good: the borrowed id grammar of ADR-an-amends-item-may-carry-a-typed-rule-delta stays a shared convention rather than an adopted method.
- Bad: a denylist cannot see a method it has not heard of.
- Bad: the boundary between a product name and a generic convention is a judgment the list encodes and does not explain.

## Status

Accepted
