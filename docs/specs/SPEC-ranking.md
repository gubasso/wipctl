# Ranking Specification

<!--TOC-->

- [Purpose](#purpose)
- [A node, and how it is written for a reader](#a-node-and-how-it-is-written-for-a-reader)
- [The key chain](#the-key-chain)
- [Requirements](#requirements)
  - [`ranking:eligibility-is-derived-from-three-edges` — Eligibility is derived from three edges](#rankingeligibility-is-derived-from-three-edges--eligibility-is-derived-from-three-edges)
  - [`ranking:an-order-is-computed-and-never-stored` — An order is computed and never stored](#rankingan-order-is-computed-and-never-stored--an-order-is-computed-and-never-stored)
  - [`ranking:a-class-orders-below-eligibility-and-above-size` — A class orders below eligibility and above size](#rankinga-class-orders-below-eligibility-and-above-size--a-class-orders-below-eligibility-and-above-size)
  - [`ranking:the-key-chain-is-total` — The key chain is total](#rankingthe-key-chain-is-total--the-key-chain-is-total)
  - [`ranking:a-key-reads-the-record-alone` — A key reads the record alone](#rankinga-key-reads-the-record-alone--a-key-reads-the-record-alone)
  - [`ranking:the-residual-key-is-the-full-id` — The residual key is the full id](#rankingthe-residual-key-is-the-full-id--the-residual-key-is-the-full-id)
  - [`ranking:the-dependency-graph-is-acyclic` — The dependency graph is acyclic](#rankingthe-dependency-graph-is-acyclic--the-dependency-graph-is-acyclic)
  - [`ranking:a-work-lane-entry-is-unblocked` — A work-lane entry is unblocked](#rankinga-work-lane-entry-is-unblocked--a-work-lane-entry-is-unblocked)
  - [`ranking:membership-is-never-an-edge` — Membership is never an edge](#rankingmembership-is-never-an-edge--membership-is-never-an-edge)
  - [`ranking:the-closed-lane-orders-by-date` — The closed lane orders by close date](#rankingthe-closed-lane-orders-by-date--the-closed-lane-orders-by-close-date)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What makes an entry eligible, and how the tool computes each lane's order. Eligibility is derived from three edges and never stored. The boundary runs at membership: the lane file says which entries belong to a lane, and this domain orders them.

## A node, and how it is written for a reader

The graph is built over pairs, and the pair is what makes the proof sound.

```text
a node       (plan_id, entry_id). The plan id, not the alias, because an
             alias is local to the plan that declares it and two plans
             can spell one peer two ways.

an edge      one dependency, resolved in the record it was written in,
             and pointing at the pair it names.
```

Rendering is the other half, and it is relative to the plan the invocation resolved to.

```text
the invoking plan's own entry     the bare id
a peer this plan declares         <alias>#<id>, using THIS plan's alias
a plan no row of this plan names  <plan_id>:<id>
```

The third case is real whenever a cycle runs through a plan two hops away. Reprinting the middle plan's own alias prints a name that resolves in no table the reader has.

The third form uses a separator no dependency accepts, so it cannot be read as a local dependency. The plan identity resolves everywhere and belongs to no alias table.

This is also why a cycle report is stable when the same cycle is found from either side. The nodes are the same pairs, and only the rendering changes.

A watch is legal on work already in flight. A question means the project does not know what to build, so the entry leaves the work lane. A watch means the project knows what to build and the outside world moved after work started. Sending that entry back to the scheduled lane would falsely say it never started and its clock never ran. The watch keeps that blocked row visible where it carries the most signal.

## The key chain

The record uses one ordering procedure. It compares close date first, ascending, for entries that carry one. Entries without a close date compare equal on that key. The remaining chain reads top to bottom:

```text
1. eligible before ineligible        derived from dependencies, blocking questions, and blocking watches
2. topological over same-lane needs  derived; the hard constraint
3. class of service                  stated; expedite, standard, intangible
4. points ascending                  stated; smaller work first
5. full id, lexical                  residual; makes the chain total
```

The first two keys express graph constraints by construction. Eligibility sorts first, and a same-lane dependency sorts before the entry that needs it.

Class is the only field a person states for ordering alone. It creates no dependency edge, changes no eligibility result, and enters no measure because it is not an event.

Smaller work first shortens the average wait. The three-point cap bounds what a larger entry can wait behind because larger work splits along its judgments.

One record-wide procedure avoids four more chains to configure. The closed lane is the only lane with a distinct key, and the close-date comparison is equal everywhere else.

## Requirements

### `ranking:eligibility-is-derived-from-three-edges` — Eligibility is derived from three edges

The implementation MUST derive eligibility from an entry's dependencies, blocking questions, and blocking watches, and MUST NOT store it.

#### Scenario: A blocked flag is proposed

- GIVEN a request to mark blocked entries
- WHEN the three edges already answer it
- THEN the flag is refused, because a stored copy of a derived fact drifts on the first edit to any edge

Verify: `cargo nextest run --test ranking`

### `ranking:an-order-is-computed-and-never-stored` — An order is computed and never stored

The implementation MUST compute every lane's order on each read and treat lane-file sequence as membership alone.

#### Scenario: A stored order is requested

- GIVEN a request to store, fix, or hand-edit a lane's order
- WHEN the key chain already computes it
- THEN there is nothing to store and nothing to repair, and changing file sequence changes no rendered order

Verify: `cargo nextest run --test ranking`

### `ranking:a-class-orders-below-eligibility-and-above-size` — A class orders below eligibility and above size

The order computation MUST compare class after eligibility and same-lane needs and before points.

#### Scenario: An expedited entry is blocked

- GIVEN an expedite entry with an open dependency and an eligible intangible entry in the same lane
- WHEN the chain compares them
- THEN the eligible intangible entry sorts first, because expedite does not make blocked work startable

Verify: `cargo nextest run --test ranking`

### `ranking:the-key-chain-is-total` — The key chain is total

The key chain MUST order every pair of entries in one lane so that no two entries compare equal.

#### Scenario: Every stated key ties

- GIVEN two entries with the same eligibility, same-lane needs relation, close date, class, and points
- WHEN the chain reaches its residual key
- THEN one id sorts first, so the preview and take verbs read one defined head

Verify: `cargo nextest run --test ranking`

### `ranking:a-key-reads-the-record-alone` — A key reads the record alone

Every ordering key MUST read the current record's entries alone.

#### Scenario: Two clones read one commit

- GIVEN two clones at one commit with the same record
- WHEN each computes a lane's order
- THEN both produce one order, because no key reads a clock, history, a directory listing, or machine-local state

Verify: `cargo nextest run --test ranking`

### `ranking:the-residual-key-is-the-full-id` — The residual key is the full id

The residual key MUST compare each full id lexically as one opaque string.

#### Scenario: Two ids share every visible part

- GIVEN two entries tied on every earlier key
- WHEN the residual key compares their ids
- THEN it parses no date, prefix, or filename from either id and compares each whole value once

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

The order computation MUST compare closed entries by close date ascending before the five-key chain and treat an absent close date as equal.

#### Scenario: An entry reopens and closes again

- GIVEN a gap left by a departure from the closed lane
- WHEN the entry closes again on a new date
- THEN the computed order places it by that date, and the old gap is not a defect

Verify: `cargo nextest run --test ranking`

## Unenforced rules

| Rule                                              | Why no command decides it                                                              |
| ------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `ranking:eligibility-is-derived-from-three-edges` | Whether a proposed field restates a derived fact is a reading of what the field means. |

A landed record's ranking is settled by a person. The key chain applies stated facts and breaks only otherwise indistinguishable ties.
