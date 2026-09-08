# Ranking Specification

## Purpose

What makes an entry eligible, and what a legal lane order is. Eligibility is derived from two edges and never stored. The boundary runs at the order: this domain says which orders are legal, while the repair domain says how an illegal one is restored.

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

The dependency graph MUST be acyclic, and an entry MUST NOT depend on itself.

#### Scenario: Two entries need each other

- GIVEN a cycle between two entries
- WHEN validation runs
- THEN it fails naming the members, because no order satisfies a cycle and every ordering rule assumes one exists

Verify: `cargo nextest run --test validation`

### `ranking:a-work-lane-entry-is-unblocked` — A work-lane entry is unblocked

An entry in the in-flight or review lane MUST have every dependency closed and MUST be free of every open question.

#### Scenario: A question is raised against work already started

- GIVEN an entry in flight and a question naming it
- WHEN validation runs
- THEN it fails, because either the question is stale or the entry moved illegally, and both need a person

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
