# A class of service is the one stated ordering key

## Context and Problem Statement

Eligibility and dependencies constrain an order, while points estimate reviewer judgment. Entries that remain tied need one lightweight way for a person to state how delay affects them.

The field needs a small shared vocabulary. A local scale would give each project a different meaning for the same rank claim.

## Considered Options

- An optional three-value class of service — chosen.
- A numeric priority — rejected: a number invites another number and another tie-break.
- A cost-of-delay score — rejected: several estimates on every entry add more judgment than the method needs.
- A fixed-date class — rejected: it needs a date on open work and creates a second stored ordering input.
- No stated ordering field — rejected: graph constraints and size do not express the shape of an entry's cost of delay.

## Decision Outcome

Chosen option: an optional `class` field with `expedite`, `standard`, and `intangible`, ordered as written. An absent class sorts at the `standard` position without adding a value on read.

The name is class rather than priority. A priority is a number. A class is a policy about a kind of work, and this vocabulary is standing rather than local.

Class follows eligibility and same-lane dependencies, then precedes points. It affects no edge, eligibility result, or measure. Multiple expedite entries warn and remain legal, so the tool shows the condition without choosing priority.

Current requirements: `lane-file:delay-cost-records-an-exception`, `lane-file:absent-delay-cost-is-ordinary`, `ranking:a-delay-cost-preference-orders-before-size`, and `messages:multiple-immediate-delay-costs-name-the-count`.

## Consequences

- Good: one stated value distinguishes entries that graph constraints and size leave tied.
- Good: existing records keep their meaning because an absent class sorts as standard.
- Bad: readers and writers must recognize one more optional field.

## Status

Superseded

Superseded by [ADR-delay-cost-names-the-ordering-fact.md](./ADR-delay-cost-names-the-ordering-fact.md).
