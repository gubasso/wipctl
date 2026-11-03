# Ranking Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`ranking:eligibility-is-derived-from-three-edges` — Eligibility is derived from three edges](#rankingeligibility-is-derived-from-three-edges--eligibility-is-derived-from-three-edges)
  - [`ranking:an-order-is-computed-and-never-stored` — An order is computed and never stored](#rankingan-order-is-computed-and-never-stored--an-order-is-computed-and-never-stored)
  - [`ranking:a-delay-cost-preference-orders-before-size` — Delay cost orders before size in a scaffolded plan](#rankinga-delay-cost-preference-orders-before-size--delay-cost-orders-before-size-in-a-scaffolded-plan)
  - [`ranking:a-preference-name-is-known-and-appears-once` — A preference name is known and appears once](#rankinga-preference-name-is-known-and-appears-once--a-preference-name-is-known-and-appears-once)
  - [`ranking:constraints-lead-and-are-fixed` — Constraints lead and are fixed](#rankingconstraints-lead-and-are-fixed--constraints-lead-and-are-fixed)
  - [`ranking:the-ranking-procedure-is-total` — The ranking procedure is total](#rankingthe-ranking-procedure-is-total--the-ranking-procedure-is-total)
  - [`ranking:an-ordering-input-reads-the-record-alone` — An ordering input reads the record alone](#rankingan-ordering-input-reads-the-record-alone--an-ordering-input-reads-the-record-alone)
  - [`ranking:the-final-tie-break-is-the-full-id` — The final tie-break is the full id](#rankingthe-final-tie-break-is-the-full-id--the-final-tie-break-is-the-full-id)
  - [`ranking:the-dependency-graph-is-acyclic` — The dependency graph is acyclic](#rankingthe-dependency-graph-is-acyclic--the-dependency-graph-is-acyclic)
  - [`ranking:a-node-renders-relative-to-the-invoking-plan` — A node renders relative to the invoking plan](#rankinga-node-renders-relative-to-the-invoking-plan--a-node-renders-relative-to-the-invoking-plan)
  - [`ranking:a-work-lane-entry-is-unblocked` — A work-lane entry is unblocked](#rankinga-work-lane-entry-is-unblocked--a-work-lane-entry-is-unblocked)
  - [`ranking:membership-is-never-an-edge` — Membership is never an edge](#rankingmembership-is-never-an-edge--membership-is-never-an-edge)
  - [`ranking:the-closed-lane-orders-by-date` — The closed lane orders by close date](#rankingthe-closed-lane-orders-by-date--the-closed-lane-orders-by-close-date)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What makes an entry eligible, and how the tool computes each lane's order. Eligibility is derived from three edges and never stored. The boundary runs at membership: the lane file says which entries belong to a lane, and this domain orders them.

## Requirements

### `ranking:eligibility-is-derived-from-three-edges` — Eligibility is derived from three edges

The implementation MUST derive eligibility from an entry's dependencies, blocking questions, and blocking watches, and MUST NOT store it.

#### Scenario: A blocked flag is proposed

- GIVEN a request to mark blocked entries
- WHEN the three edges already answer it
- THEN the flag is refused, because a stored copy of a derived fact drifts on the first edit to any edge

Verify: `cargo nextest run --test ranking`

### `ranking:an-order-is-computed-and-never-stored` — An order is computed and never stored

The implementation MUST compute every lane's order with the ranking procedure on each read and treat lane-file sequence as membership alone.

#### Scenario: A stored order is requested

- GIVEN a request to store, fix, or hand-edit a lane's order
- WHEN the ranking procedure already computes it
- THEN there is nothing to store and nothing to repair, and changing file sequence changes no rendered order

Verify: `cargo nextest run --test ranking`

### `ranking:a-delay-cost-preference-orders-before-size` — Delay cost orders before size in a scaffolded plan

The scaffolded ranking preferences MUST place `delay-cost` before `points-ascending`.

#### Scenario: A scaffolded plan has immediate and small work

- GIVEN two eligible entries where one has immediate delay cost and three points and the other has one point
- WHEN the scaffolded preferences compare the entries
- THEN the entry with immediate delay cost sorts first

Verify: `cargo nextest run --test ranking`

### `ranking:a-preference-name-is-known-and-appears-once` — A preference name is known and appears once

When ranking preferences load, the implementation MUST accept only `delay-cost` and `points-ascending`, and each name MUST appear at most once.

#### Scenario: A preference list repeats one unknown name

- GIVEN a preference list containing `priority` twice
- WHEN the configuration loads
- THEN it fails naming the valid set and the repetition, because the unknown comparison has no meaning and the second occurrence is unreachable

Verify: `cargo nextest run --test configuration`

### `ranking:constraints-lead-and-are-fixed` — Constraints lead and are fixed

The ranking procedure MUST apply eligibility and same-lane dependency constraints before every configured preference.

#### Scenario: Immediate work is blocked

- GIVEN a blocked entry with immediate delay cost and an eligible entry with deferred delay cost
- WHEN the ranking procedure compares the entries
- THEN the eligible entry sorts first because a preference cannot override a constraint

Verify: `cargo nextest run --test ranking`

### `ranking:the-ranking-procedure-is-total` — The ranking procedure is total

The ranking procedure MUST order every pair of entries in one lane so that no two entries compare equal.

#### Scenario: Every preference ties

- GIVEN two entries with the same constraints, close date, delay cost, and points
- WHEN the procedure reaches its final tie-break
- THEN one id sorts first, so the preview and take verbs read one defined head

Verify: `cargo nextest run --test ranking`

### `ranking:an-ordering-input-reads-the-record-alone` — An ordering input reads the record alone

Every ranking constraint, preference, and tie-break MUST read the current record's entries and plan configuration alone.

#### Scenario: Two clones read one commit

- GIVEN two clones at one commit with the same record and configuration
- WHEN each computes a lane's order
- THEN both produce one order because no ordering input reads a clock, history, directory listing, or machine-local state

Verify: `cargo nextest run --test ranking`

### `ranking:the-final-tie-break-is-the-full-id` — The final tie-break is the full id

The final tie-break MUST compare each full id lexically as one opaque string.

#### Scenario: Two ids share every visible part

- GIVEN two entries tied on every earlier key
- WHEN the final tie-break compares their ids
- THEN it parses no date, prefix, or filename from either id and compares each whole value once

Verify: `cargo nextest run --test ranking`

### `ranking:the-dependency-graph-is-acyclic` — The dependency graph is acyclic

The dependency graph MUST be acyclic over the attached closure, and an entry MUST NOT depend on itself.

#### Scenario: Two plans need each other

- GIVEN an entry here depending on an entry there, and that entry depending back
- WHEN validation runs from either side
- THEN it fails naming the same members, each written in the form that names its record. No order satisfies a cycle, and a cycle that crosses a boundary is still one cycle

Verify: `cargo nextest run --test validation`

### `ranking:a-node-renders-relative-to-the-invoking-plan` — A node renders relative to the invoking plan

When a view names a graph node, it MUST print the bare id for the invoking plan's own entry, `<alias>#<id>` with this plan's alias for a peer this plan declares, and `<plan_id>:<id>` for a plan no row of this plan names.

#### Scenario: A cycle runs two hops away

- GIVEN `alpha` declares `beta`, and `beta` declares `gamma`
- WHEN `alpha` reports a cycle through an entry of `gamma`
- THEN it prints that node as `<plan_id>:<id>`, because the alias `beta` gives `gamma` resolves in no table `alpha` has

Verify: `cargo nextest run --test ranking`

### `ranking:a-work-lane-entry-is-unblocked` — A work-lane entry is unblocked

An entry in the in-flight or review lane MUST have every dependency closed and MUST be free of every open question.

#### Scenario: A dependency sits in another plan

- GIVEN an entry in flight whose prefixed dependency is still open in its peer
- WHEN validation runs
- THEN it fails. A peer's entry is an entry and its lane is a lane, so this rule needed no edit to reach across a boundary

Verify: `cargo nextest run --test validation`

### `ranking:membership-is-never-an-edge` — Membership is never an edge

An epic field and an initiative section MUST NOT enter the dependency graph, sequence anything, or affect eligibility.

#### Scenario: An epic's members are ordered

- GIVEN entries sharing one epic
- WHEN the order is computed
- THEN membership contributes nothing, because membership is not sequencing at any tier

Verify: `cargo nextest run --test ranking`

### `ranking:the-closed-lane-orders-by-date` — The closed lane orders by close date

The ranking procedure MUST compare closed entries by close date ascending before its other stages and treat an absent close date as equal.

#### Scenario: An entry reopens and closes again

- GIVEN a gap left by a departure from the closed lane
- WHEN the entry closes again on a new date
- THEN the computed order places it by that date, and the old gap is not a defect

Verify: `cargo nextest run --test ranking`

## Unenforced rules

| Rule                                              | Why no command decides it                                                              |
| ------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `ranking:eligibility-is-derived-from-three-edges` | Whether a proposed field restates a derived fact is a reading of what the field means. |

A landed record's ranking is settled by a person. The procedure applies stated facts and breaks only otherwise indistinguishable ties.
