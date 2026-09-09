# Metrics Specification

<!--TOC-->

- [Purpose](#purpose)
- [The three flow measures](#the-three-flow-measures)
- [Requirements](#requirements)
  - [`metrics:a-measure-is-folded-from-the-journal` — A measure is folded from the journal](#metricsa-measure-is-folded-from-the-journal--a-measure-is-folded-from-the-journal)
  - [`metrics:an-entry-outside-this-record-is-never-a-quantity` — An entry outside this record is never a quantity](#metricsan-entry-outside-this-record-is-never-a-quantity--an-entry-outside-this-record-is-never-a-quantity)
  - [`metrics:the-clock-is-read-once` — The clock is read once](#metricsthe-clock-is-read-once--the-clock-is-read-once)
  - [`metrics:an-unmeasurable-entry-is-reported-unmeasured` — An unmeasurable entry is reported unmeasured](#metricsan-unmeasurable-entry-is-reported-unmeasured--an-unmeasurable-entry-is-reported-unmeasured)
  - [`metrics:an-out-of-order-journal-is-not-folded` — An out-of-order journal is not folded](#metricsan-out-of-order-journal-is-not-folded--an-out-of-order-journal-is-not-folded)
  - [`metrics:a-measure-is-the-same-on-any-clone` — A measure is the same on any clone](#metricsa-measure-is-the-same-on-any-clone--a-measure-is-the-same-on-any-clone)
  - [`metrics:a-lane-without-a-measure-is-a-usage-error` — A lane without a measure is a usage error](#metricsa-lane-without-a-measure-is-a-usage-error--a-lane-without-a-measure-is-a-usage-error)
  - [`metrics:the-stale-view-shares-the-flow-fold` — The stale view shares the flow fold](#metricsthe-stale-view-shares-the-flow-fold--the-stale-view-shares-the-flow-fold)
  - [`metrics:the-threshold-is-undefaulted-and-never-gates` — The threshold is undefaulted and never gates](#metricsthe-threshold-is-undefaulted-and-never-gates--the-threshold-is-undefaulted-and-never-gates)
  - [`metrics:only-a-delivered-close-counts` — Only a delivered close counts](#metricsonly-a-delivered-close-counts--only-a-delivered-close-counts)
  - [`metrics:an-empty-window-renders-at-zero` — An empty window renders at zero](#metricsan-empty-window-renders-at-zero--an-empty-window-renders-at-zero)
  - [`metrics:a-changed-cadence-restarts-the-series` — A changed cadence restarts the series](#metricsa-changed-cadence-restarts-the-series--a-changed-cadence-restarts-the-series)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Age, dwell, rework, and delivery per window. Every number here is folded from the record, never from repository archaeology. Age and staleness are two things: age is the number of days, and the `stale_after` threshold is the line drawn across it. The boundary runs at the source: this domain owns the measures, the transition journal domain owns the events they fold, and the rendering domain owns how a bar is drawn.

## The three flow measures

| Measure | Definition                                                                                           |
| ------- | ---------------------------------------------------------------------------------------------------- |
| age     | from the first entry into the in-flight lane to the entry's last event when closed, otherwise to now |
| dwell   | the summed intervals spent in the review lane, counting an open interval through now                 |
| rework  | entries into the in-flight lane beyond the first                                                     |

A duration is displayed in whole days, floored.

## Requirements

### `metrics:a-measure-is-folded-from-the-journal` — A measure is folded from the journal

Every flow measure MUST be folded from the transition journal, and MUST NOT be read from repository history, commit metadata, or a remote.

#### Scenario: The same record is measured on a shallow clone

- GIVEN a checkout with truncated history
- WHEN the measures run
- THEN they match the full clone's numbers, because the journal is the source and history only witnesses it

Verify: `cargo nextest run --test metrics`

### `metrics:an-entry-outside-this-record-is-never-a-quantity` — An entry outside this record is never a quantity

Every derived number MUST be computed from this record's own entries alone: velocity, epic and initiative arithmetic, flow, staleness, and every board count.

#### Scenario: A peer's entry closes

- GIVEN an entry in an attached peer, closed inside this window
- WHEN velocity is computed
- THEN it counts nothing, because a prefixed dependency is an edge and an ordering fact, and never a quantity this record delivered

Verify: `cargo nextest run --test metrics`

### `metrics:the-clock-is-read-once` — The clock is read once

The implementation MUST read the clock once per invocation and MUST use that instant for every open interval.

#### Scenario: A long run measures many entries

- GIVEN several entries with open intervals
- WHEN the fold runs
- THEN all of them are measured against one instant, because two clock reads make one report internally inconsistent

Verify: `cargo nextest run --test metrics`

### `metrics:an-unmeasurable-entry-is-reported-unmeasured` — An unmeasurable entry is reported unmeasured

An entry with no journal MUST be reported unmeasured with its reason, and MUST NOT be reported as zero.

#### Scenario: An entry has never moved

- GIVEN an entry captured and never started
- WHEN the measures run
- THEN it reports unmeasured, because zero says the work took no time and unmeasured says nothing is known

Verify: `cargo nextest run --test metrics`

### `metrics:an-out-of-order-journal-is-not-folded` — An out-of-order journal is not folded

An entry whose journal instants decrease anywhere MUST be reported unmeasured, and MUST NOT be folded into a negative, clamped, or skipped interval.

#### Scenario: Two machines wrote events with drifting clocks

- GIVEN a journal whose instants decrease
- WHEN the fold runs
- THEN the entry reports unmeasured, because the record already carries the warning and a pretended number is worse than none

Verify: `cargo nextest run --test metrics`

### `metrics:a-measure-is-the-same-on-any-clone` — A measure is the same on any clone

The measures MUST be read-only and local, and MUST leave the record and the exit code untouched.

#### Scenario: A measure is proposed as a gate

- GIVEN a request to fail a commit on an age threshold
- WHEN it is weighed
- THEN it is refused, because a measure that gates turns a report into a rule nobody agreed to

Verify: `cargo nextest run --test metrics`

### `metrics:a-lane-without-a-measure-is-a-usage-error` — A lane without a measure is a usage error

Where a lane measure is requested for a lane that has none, the implementation MUST report a usage error naming the lane.

#### Scenario: A planning lane is asked for its dwell

- GIVEN a request for a lane measure the lane does not carry
- WHEN it parses
- THEN it is a usage error, because only the two work lanes carry a lane measure

Verify: `cargo nextest run --test verb_contracts`

### `metrics:the-stale-view-shares-the-flow-fold` — The stale view shares the flow fold

The stale view MUST take its ages from the same fold as the flow table, and MUST show an unmeasured entry with its reason.

#### Scenario: Two views show one entry's age

- GIVEN an entry in flight
- WHEN both views render
- THEN the numbers agree, because two folds of one journal disagree on the first edge case

Verify: `cargo nextest run --test metrics`

### `metrics:the-threshold-is-undefaulted-and-never-gates` — The threshold is undefaulted and never gates

Where the configuration carries a `stale_after` table, the stale view MUST mark rows past its threshold, and its absence MUST leave every row unmarked.

#### Scenario: A project sets no threshold

- GIVEN a configuration with no `stale_after` table
- WHEN the view renders
- THEN no row is marked, because there is no default threshold to mark against

Verify: `cargo nextest run --test metrics`

### `metrics:only-a-delivered-close-counts` — Only a delivered close counts

The delivery measure MUST count only entries closed as done, at their own points, in the window of their own close date.

#### Scenario: A reshaped entry and its successor both close

- GIVEN a reshaped entry whose successor later closes as done
- WHEN the windows are totalled
- THEN only the successor's points are counted, in the successor's window, so no points are ever counted twice

Verify: `cargo nextest run --test metrics`

### `metrics:an-empty-window-renders-at-zero` — An empty window renders at zero

A window with no closes MUST render at zero rather than being omitted, and the current window MUST be marked partial.

#### Scenario: A quiet fortnight

- GIVEN a window in which nothing closed
- WHEN the series renders
- THEN the zero row appears, because an omitted window makes a gap read as a shorter history

Verify: `cargo nextest run --test metrics`

### `metrics:a-changed-cadence-restarts-the-series` — A changed cadence restarts the series

Where the window anchor or length changes, the implementation MUST restart the window series and MUST NOT stitch old windows onto new.

#### Scenario: A team changes its cadence

- GIVEN a new window length
- WHEN the series renders
- THEN it starts again from the anchor, because a window redefined mid-series compares two different things

Verify: `cargo nextest run --test metrics`

## Unenforced rules

| Rule                                         | Why no command decides it                                                                    |
| -------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `metrics:a-measure-is-the-same-on-any-clone` | Whether a proposed consumer of a measure gates on it is a reading of what the consumer does. |

The measures are the three flow numbers and delivery per window. Forecasting, averaging, and counting anything but points are outside this domain.
