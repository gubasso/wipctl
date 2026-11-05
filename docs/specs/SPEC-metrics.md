# Metrics Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`metrics:a-measure-is-folded-from-the-journal` — A measure is folded from the journal](#metricsa-measure-is-folded-from-the-journal--a-measure-is-folded-from-the-journal)
  - [`metrics:an-entry-outside-this-record-is-never-a-quantity` — An entry outside this record is never a quantity](#metricsan-entry-outside-this-record-is-never-a-quantity--an-entry-outside-this-record-is-never-a-quantity)
  - [`metrics:the-clock-is-read-once` — The clock is read once](#metricsthe-clock-is-read-once--the-clock-is-read-once)
  - [`metrics:a-duration-is-shown-in-whole-days` — A duration is shown in whole days](#metricsa-duration-is-shown-in-whole-days--a-duration-is-shown-in-whole-days)
  - [`metrics:an-unmeasurable-entry-is-reported-unmeasured` — An unmeasurable entry is reported unmeasured](#metricsan-unmeasurable-entry-is-reported-unmeasured--an-unmeasurable-entry-is-reported-unmeasured)
  - [`metrics:an-out-of-order-journal-is-not-folded` — An out-of-order journal is not folded](#metricsan-out-of-order-journal-is-not-folded--an-out-of-order-journal-is-not-folded)
  - [`metrics:a-measure-is-the-same-on-any-clone` — A measure is the same on any clone](#metricsa-measure-is-the-same-on-any-clone--a-measure-is-the-same-on-any-clone)
  - [`metrics:a-lane-without-a-measure-is-a-usage-error` — A lane without a measure is a usage error](#metricsa-lane-without-a-measure-is-a-usage-error--a-lane-without-a-measure-is-a-usage-error)
  - [`metrics:the-stale-view-shares-the-flow-fold` — The stale view shares the flow fold](#metricsthe-stale-view-shares-the-flow-fold--the-stale-view-shares-the-flow-fold)
  - [`metrics:the-threshold-is-undefaulted-and-never-gates` — The threshold is undefaulted and never gates](#metricsthe-threshold-is-undefaulted-and-never-gates--the-threshold-is-undefaulted-and-never-gates)
  - [`metrics:a-stale-row-is-announced-once-per-invocation` — A stale row is announced once per invocation](#metricsa-stale-row-is-announced-once-per-invocation--a-stale-row-is-announced-once-per-invocation)
  - [`metrics:a-suppression-is-local-and-never-replicates` — A suppression is local and never replicates](#metricsa-suppression-is-local-and-never-replicates--a-suppression-is-local-and-never-replicates)
  - [`metrics:a-hide-names-one-end-condition` — A hide names one end condition](#metricsa-hide-names-one-end-condition--a-hide-names-one-end-condition)
  - [`metrics:a-suppression-hides-a-reminder-and-never-a-number` — A suppression hides a reminder and never a number](#metricsa-suppression-hides-a-reminder-and-never-a-number--a-suppression-hides-a-reminder-and-never-a-number)
  - [`metrics:a-suppression-is-announced-in-the-view-it-hides` — A suppression is announced in the view it hides](#metricsa-suppression-is-announced-in-the-view-it-hides--a-suppression-is-announced-in-the-view-it-hides)
  - [`metrics:a-marked-row-is-never-acted-on-without-an-answer` — A marked row is never acted on without an answer](#metricsa-marked-row-is-never-acted-on-without-an-answer--a-marked-row-is-never-acted-on-without-an-answer)
  - [`metrics:only-a-delivered-close-counts` — Only a delivered close counts](#metricsonly-a-delivered-close-counts--only-a-delivered-close-counts)
  - [`metrics:an-empty-window-renders-at-zero` — An empty window renders at zero](#metricsan-empty-window-renders-at-zero--an-empty-window-renders-at-zero)
  - [`metrics:a-changed-cadence-restarts-the-series` — A changed cadence restarts the series](#metricsa-changed-cadence-restarts-the-series--a-changed-cadence-restarts-the-series)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

Age, dwell, rework, and delivery per window. Every number here is folded from the record, never from repository archaeology. Age and staleness are two things: age is the number of days, and the `stale_after` threshold is the line drawn across it. The boundary runs at the source: this domain owns the measures, the transition journal domain owns the events they fold, and the rendering domain owns how a bar is drawn.

## Requirements

### `metrics:a-measure-is-folded-from-the-journal` — A measure is folded from the journal

Every flow measure MUST be folded from the transition journal, and MUST NOT be read from repository history, commit metadata, or a remote.

#### Scenario: The same record is measured on a shallow clone

- GIVEN a checkout with truncated history
- WHEN the measures run
- THEN they match the full clone's numbers, because the journal is the source and history only witnesses it

The measures are the three flow numbers and delivery per window. Forecasting, averaging, and counting anything but points are outside this domain.

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

### `metrics:a-duration-is-shown-in-whole-days` — A duration is shown in whole days

When a view displays a flow duration, it MUST show it in whole days, floored.

#### Scenario: An entry entered review 36 hours ago

- GIVEN an open dwell of one day and twelve hours
- WHEN the flow table renders
- THEN it shows 1, because a floored day never claims time that has not passed

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

Verify: `cargo nextest run --test metrics`, and a reviewer confirms that no proposed consumer of a measure gates on it

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

### `metrics:a-stale-row-is-announced-once-per-invocation` — A stale row is announced once per invocation

When a workflow verb other than `stale` finds rows that the ordinary stale view shows, it MUST announce their count once per invocation on stderr at exit 0 and name `wipctl stale` as the resolution.

#### Scenario: A workflow verb renders several views

- GIVEN four rows that the ordinary stale view shows
- WHEN one workflow verb renders several views
- THEN one warning announces four stale entries and names the stale verb, because repeated warnings hide the fact they report

Verify: `cargo nextest run --test verb_contracts`

The warning makes every workflow verb perform the age fold before it prints. A verb that did not read the journal before now reads it.

The fold reads the transition journal and reads the clock once under the existing rules. A cached count would duplicate state that the record already holds.

The same clock reading folds each watch's `Read` date against its resolved cold-reading threshold. A watch counts when that date is older than the threshold and at least one blocked entry remains open. A watch whose blocked entries have all closed is already reported under `watches:a-stale-watch-is-reported`, so the cold-reading count excludes it.

The entry threshold measures this project's work going quiet. The watch threshold measures an outside item going unread, so each uses its own period.

### `metrics:a-suppression-is-local-and-never-replicates` — A suppression is local and never replicates

When an operator hides or shows a stale row, the implementation MUST write only this machine's suppression file and leave the plan record, cache, and remote unchanged.

#### Scenario: An operator hides a stale row on one clone

- GIVEN two machines attached to the same plan
- WHEN an operator hides one stale row on the first machine
- THEN the second machine still shows the row, because a local reminder is not a team decision

Verify: `cargo nextest run --test metrics`

### `metrics:a-hide-names-one-end-condition` — A hide names one end condition

A hide request MUST name an existing entry and exactly one end condition, or fail as a usage error naming both flags, and only the hide and show forms MUST write the suppression file, while a read path ignores and reports a suppression whose id names no entry.

#### Scenario: A hide names no duration

- GIVEN `wipctl stale --hide the-stale-alarm` with neither `--for` nor `--forever`
- WHEN the verb parses it
- THEN it exits 2 naming both flags, because a default duration is a value nobody chose

Verify: `cargo nextest run --test metrics`

### `metrics:a-suppression-hides-a-reminder-and-never-a-number` — A suppression hides a reminder and never a number

While a suppression is active, the implementation MUST omit its row from the ordinary stale view and warning count while preserving it on the board, in flow, in `wipctl doctor`, and in `stale --all`.

#### Scenario: A suppressed row appears on the board

- GIVEN a stale row with an active suppression
- WHEN the board and flow table render
- THEN the row and its age remain visible, because a suppression removes a reminder rather than work or a measure

Verify: `cargo nextest run --test metrics`

### `metrics:a-suppression-is-announced-in-the-view-it-hides` — A suppression is announced in the view it hides

When the ordinary stale view omits active suppressions, it MUST report their count on stderr and name `wipctl stale --all` as the view that shows them.

#### Scenario: Three stale rows are suppressed

- GIVEN three active suppressions and one visible stale row
- WHEN the ordinary stale view renders
- THEN stderr reports three suppressed rows and names the complete view, because an invisible silence can outlive its reason

The announcement is exactly:

```text
wipctl: 3 rows are suppressed; `wipctl stale --all` shows them
```

Verify: `cargo nextest run --test verb_contracts`

### `metrics:a-marked-row-is-never-acted-on-without-an-answer` — A marked row is never acted on without an answer

When an agent reads a marked row, it MUST NOT close, cut, split, reshape, or move that entry before a person answers what the record should do.

#### Scenario: An old row has no recent branch commits

- GIVEN a stale entry with an inactive named branch
- WHEN an agent gathers that evidence
- THEN it offers evidence-based options and waits for an answer, because age and branch activity cannot decide what the business still wants

Verify: reviewer confirms the agent protocol requires a person's answer before any record change

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

## Unenforced

| Rule                                                       | Reviewer confirms                                                                                                                                           |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `metrics:a-measure-is-the-same-on-any-clone`               | no proposed consumer of a measure gates on it, because whether a proposed consumer gates on a measure is a reading of what the consumer does                |
| `metrics:a-marked-row-is-never-acted-on-without-an-answer` | the agent protocol requires a person's answer before any record change, because whether an action follows a person's answer is a reading of the interaction |
