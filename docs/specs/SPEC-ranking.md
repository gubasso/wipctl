# Ranking Specification

<!--TOC-->

- [Purpose](#purpose)
- [A node, and how it is written for a reader](#a-node-and-how-it-is-written-for-a-reader)
- [Requirements](#requirements)
  - [`ranking:eligibility-is-derived-from-two-edges` — Eligibility is derived from two edges](#rankingeligibility-is-derived-from-two-edges--eligibility-is-derived-from-two-edges)
  - [`ranking:an-eligible-entry-sits-above-an-ineligible-one` — An eligible entry sits above an ineligible one](#rankingan-eligible-entry-sits-above-an-ineligible-one--an-eligible-entry-sits-above-an-ineligible-one)
  - [`ranking:an-entry-sits-below-what-it-needs` — An entry sits below what it needs](#rankingan-entry-sits-below-what-it-needs--an-entry-sits-below-what-it-needs)
  - [`ranking:the-dependency-graph-is-acyclic` — The dependency graph is acyclic](#rankingthe-dependency-graph-is-acyclic--the-dependency-graph-is-acyclic)
  - [`ranking:a-work-lane-entry-is-unblocked` — A work-lane entry is unblocked](#rankinga-work-lane-entry-is-unblocked--a-work-lane-entry-is-unblocked)
  - [`ranking:membership-is-never-an-edge` — Membership is never an edge](#rankingmembership-is-never-an-edge--membership-is-never-an-edge)
  - [`ranking:the-closed-lane-orders-by-date` — The closed lane orders by close date](#rankingthe-closed-lane-orders-by-date--the-closed-lane-orders-by-close-date)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What makes an entry eligible, and what a legal lane order is. Eligibility is derived from two edges and never stored. The boundary runs at the order: this domain says which orders are legal, while the repair domain says how an illegal one is restored.

## A node, and how it is written for a reader

The graph is built over pairs, and the pair is what makes the proof sound.

```text
a node       (plan_uid, entry_id). The uid, not the alias, because an
             alias is local to the plan that declares it and two plans
             can spell one peer two ways.

an edge      one dependency, resolved in the record it was written in,
             and pointing at the pair it names.
```

Rendering is the other half, and it is relative to the plan the invocation resolved to.

```text
the invoking plan's own entry     the bare id
a peer this plan declares         <alias>#<id>, using THIS plan's alias
a plan no row of this plan names  <uid>:<id>, with that plan's project_id
                                  on the following line
```

The third case is real whenever a cycle runs through a plan two hops away. Reprinting the middle plan's own alias prints a name that resolves in no table the reader has.

The third form uses a separator no dependency accepts, so it cannot be read back as a local one. A project id is a slug, and an alias is a slug, so `<project_id>#<id>` parses as a prefixed dependency. Where this plan's table happens to bind that same slug to a different peer, that form names the wrong plan while looking correct. The uid resolves everywhere and belongs to no table, so it leads.

This is also why a cycle report is stable when the same cycle is found from either side. The nodes are the same pairs, and only the rendering changes.

## Requirements

### `ranking:eligibility-is-derived-from-two-edges` — Eligibility is derived from two edges

The implementation MUST derive eligibility from an entry's dependencies and its blocking questions, and MUST NOT store it.

#### Scenario: A blocked flag is proposed

- GIVEN a request to mark blocked entries
- WHEN the two edges already answer it
- THEN the flag is refused, because a stored copy of a derived fact drifts on the first edit to either edge

Verify: `cargo nextest run --test ranking`

### `ranking:an-eligible-entry-sits-above-an-ineligible-one` — An eligible entry sits above an ineligible one

In the scheduled lane, an eligible entry MUST NOT sit below an ineligible one.

#### Scenario: The lane holds work that can start

- GIVEN a scheduled lane holding at least one eligible entry
- WHEN a reader takes the head
- THEN it is startable, because the topmost entry is what to start and a blocked head makes the lane a puzzle

Verify: `cargo nextest run --test ranking`

### `ranking:an-entry-sits-below-what-it-needs` — An entry sits below what it needs

In the planning lanes, an entry MUST NOT sit above an entry it depends on, counting same-lane edges alone.

#### Scenario: A dependency sits in another lane

- GIVEN an entry whose dependency is already in flight
- WHEN the order is checked
- THEN the cross-lane edge is not counted, because ranking orders one lane and lanes are not ranked against each other

Verify: `cargo nextest run --test ranking`

### `ranking:the-dependency-graph-is-acyclic` — The dependency graph is acyclic

The dependency graph MUST be acyclic over the attached closure, and an entry MUST NOT depend on itself.

#### Scenario: Two plans need each other

- GIVEN an entry here depending on an entry there, and that entry depending back
- WHEN validation runs from either side
- THEN it fails naming the same members, each written in the form that names its record. No order satisfies a cycle, and a cycle that crosses a boundary is still one cycle

Verify: `cargo nextest run --test validation`

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

The closed lane MUST be ordered by close date ascending, stable within a day, and an out-of-sequence date MUST be a warning.

#### Scenario: An entry reopens and closes again

- GIVEN a gap left by a departure from the closed lane
- WHEN the order is checked
- THEN the gap is not a defect, while an out-of-sequence date is a warning the reader decides about

Verify: `cargo nextest run --test ranking`

## Unenforced rules

| Rule                                            | Why no command decides it                                                              |
| ----------------------------------------------- | -------------------------------------------------------------------------------------- |
| `ranking:eligibility-is-derived-from-two-edges` | Whether a proposed field restates a derived fact is a reading of what the field means. |

A landed record's ranking is settled by a person. The repair restores legality and never chooses between two legal orders.
