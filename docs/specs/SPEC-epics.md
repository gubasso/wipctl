# Epics Specification

<!--TOC-->

- [Purpose](#purpose)
- [The point arithmetic](#the-point-arithmetic)
- [Requirements](#requirements)
  - [`epics:the-arithmetic-has-one-implementation` — The arithmetic has one implementation](#epicsthe-arithmetic-has-one-implementation--the-arithmetic-has-one-implementation)
  - [`epics:membership-is-searched-from-the-entries` — Membership is searched from the entries](#epicsmembership-is-searched-from-the-entries--membership-is-searched-from-the-entries)
  - [`epics:the-plan-is-the-transitive-closure` — The plan is the transitive closure](#epicsthe-plan-is-the-transitive-closure--the-plan-is-the-transitive-closure)
  - [`epics:an-outside-prerequisite-is-marked-never-hidden` — An outside prerequisite is marked, never hidden](#epicsan-outside-prerequisite-is-marked-never-hidden--an-outside-prerequisite-is-marked-never-hidden)
  - [`epics:the-order-is-not-a-schedule` — The order is not a schedule](#epicsthe-order-is-not-a-schedule--the-order-is-not-a-schedule)
  - [`epics:a-reshaped-member-follows-one-link` — A reshaped member follows one link](#epicsa-reshaped-member-follows-one-link--a-reshaped-member-follows-one-link)
  - [`epics:a-bare-invocation-lists` — A bare invocation lists](#epicsa-bare-invocation-lists--a-bare-invocation-lists)
  - [`epics:an-unjoined-epic-appears-at-zero` — An unjoined epic appears at zero](#epicsan-unjoined-epic-appears-at-zero--an-unjoined-epic-appears-at-zero)
  - [`epics:no-verb-reports-a-staleness-verdict` — No verb reports a staleness verdict](#epicsno-verb-reports-a-staleness-verdict--no-verb-reports-a-staleness-verdict)
  - [`epics:the-resolution-is-derived-every-call` — The resolution is derived on every call](#epicsthe-resolution-is-derived-every-call--the-resolution-is-derived-on-every-call)
  - [`epics:the-cache-is-authoritative-for-nothing` — The cache is authoritative for nothing](#epicsthe-cache-is-authoritative-for-nothing--the-cache-is-authoritative-for-nothing)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Resolving one epic into its execution plan, and rolling every epic up into one row each. The boundary runs at the tier. This domain owns the epic's resolution and the arithmetic every tier shares. The initiatives domain owns the tier above, and the documents domain owns the epic document's shape.

## The point arithmetic

| Outcome    | Contribution                                                      |
| ---------- | ----------------------------------------------------------------- |
| `done`     | delivers its own points                                           |
| `cut`      | contributes nothing and stays in the total                        |
| `reshaped` | delivers its own points exactly when its successor closed as done |
| reopened   | counts as open                                                    |

## Requirements

### `epics:the-arithmetic-has-one-implementation` — The arithmetic has one implementation

One implementation MUST compute the point arithmetic for every tier.

#### Scenario: A number appears in two views

- GIVEN a single epic shown in the resolution and in the rollup
- WHEN both render
- THEN the numbers agree, because two implementations of one sum disagree on the first edge case

Verify: `cargo nextest run --test epics`

### `epics:membership-is-searched-from-the-entries` — Membership is searched from the entries

The implementation MUST search membership from the entries' epic fields, and the epic document MUST hold no member list.

#### Scenario: A member is added

- GIVEN an entry gaining an epic field
- WHEN the epic resolves
- THEN the member appears with no edit to the epic document. A member list is a second store that goes stale on the first change

Verify: `cargo nextest run --test epics`

### `epics:the-plan-is-the-transitive-closure` — The plan is the transitive closure

The resolution MUST be the transitive closure of the members' dependency edges, and MUST drop closed entries from the groups.

#### Scenario: A member depends on a chain

- GIVEN a member whose dependency itself depends on another entry
- WHEN the epic resolves
- THEN the whole chain appears, because a plan missing a prerequisite is a plan that stalls

Verify: `cargo nextest run --test epics`

### `epics:an-outside-prerequisite-is-marked-never-hidden` — An outside prerequisite is marked, never hidden

A prerequisite outside the epic MUST be included and marked by what it serves.

#### Scenario: A member waits on another epic's work

- GIVEN a dependency belonging to a different epic
- WHEN the epic resolves
- THEN it appears marked with what it serves, because hiding it hides the reason the epic is stalled

Verify: `cargo nextest run --test epics`

### `epics:the-order-is-not-a-schedule` — The order is not a schedule

The resolution MUST order entries topologically, breaking ties by record position, and MUST partition them into eligible and blocked groups keeping that order.

#### Scenario: A consumer picks work from the plan

- GIVEN an eligible group of three entries
- WHEN the consumer takes one
- THEN any of the three is correct, because the order is not a schedule and not a ranking

Verify: `cargo nextest run --test epics`

### `epics:a-reshaped-member-follows-one-link` — A reshaped member follows one link

A reshaped member MUST deliver its points exactly when its successor closed as done, following one link and no more.

#### Scenario: A successor is itself reshaped

- GIVEN a chain of two reshaped members
- WHEN the arithmetic runs
- THEN the first stays undelivered, and a successor that is itself a member counts separately as itself

Verify: `cargo nextest run --test epics`

### `epics:a-bare-invocation-lists` — A bare invocation lists

A bare invocation of the resolution verb MUST list the epic ids and MUST NOT fall back to the rollup.

#### Scenario: A reader asks which epics exist

- GIVEN a bare invocation
- WHEN it runs
- THEN it lists the ids, because that asks which epics exist rather than how they are going, and each verb's usage names the other

Verify: `cargo nextest run --test verb_contracts`

### `epics:an-unjoined-epic-appears-at-zero` — An unjoined epic appears at zero

An epic no entry joined MUST appear at zero rather than being omitted.

#### Scenario: An epic is written before its stories

- GIVEN a legal first draft with no members
- WHEN the rollup renders
- THEN it appears at zero, because an omitted epic reads as an epic that does not exist

Verify: `cargo nextest run --test epics`

### `epics:no-verb-reports-a-staleness-verdict` — No verb reports a staleness verdict

The implementation MUST NOT report a staleness verdict or retirement advice at any tier.

#### Scenario: An epic has nothing open

- GIVEN an epic whose members are all closed
- WHEN the rollup renders
- THEN it states that nothing remains open. Whether to retire the epic is a review question

Verify: `cargo nextest run --test epics`

### `epics:the-resolution-is-derived-every-call` — The resolution is derived on every call

The resolution MUST be derived on every call and MUST be stored nowhere in the record.

#### Scenario: A resolved plan is proposed as a file

- GIVEN a request to keep the resolution in the zone
- WHEN the members change
- THEN the stored copy is wrong, so the resolution is computed each time

Verify: `cargo nextest run --test epics`

### `epics:the-cache-is-authoritative-for-nothing` — The cache is authoritative for nothing

Where the resolution is written to the cache, a missing, stale, or corrupt cache MUST change no answer and MUST fail nothing.

#### Scenario: A cache file is deleted

- GIVEN a consumer that wrote the payload to the cache
- WHEN the file is gone
- THEN every verb answers the same, because the cache is rewritten on each call and read back by nothing

Verify: `cargo nextest run --test epics`

## Unenforced rules

| Rule                                        | Why no command decides it                                                        |
| ------------------------------------------- | -------------------------------------------------------------------------------- |
| `epics:no-verb-reports-a-staleness-verdict` | Whether a proposed field is a verdict or a fact is a reading of what it asserts. |

An unknown id is a usage error naming the directory searched and the listing that shows what exists. It is a bad invocation and not a failed check.
