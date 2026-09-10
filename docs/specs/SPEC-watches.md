# Watches Specification

<!--TOC-->

- [Purpose](#purpose)
- [Section shape](#section-shape)
- [Reference or phrase](#reference-or-phrase)
- [Requirements](#requirements)
  - [`watches:a-watch-blocks-something` — A watch blocks something](#watchesa-watch-blocks-something--a-watch-blocks-something)
  - [`watches:a-watch-names-what-clears-it` — A watch names what clears it](#watchesa-watch-names-what-clears-it--a-watch-names-what-clears-it)
  - [`watches:a-watch-states-a-testable-exit` — A watch states a testable exit](#watchesa-watch-states-a-testable-exit--a-watch-states-a-testable-exit)
  - [`watches:a-watch-carries-a-section-per-part` — A watch carries a section per part](#watchesa-watch-carries-a-section-per-part--a-watch-carries-a-section-per-part)
  - [`watches:a-watch-is-never-a-dependency` — A watch is never a dependency](#watchesa-watch-is-never-a-dependency--a-watch-is-never-a-dependency)
  - [`watches:a-cleared-watch-leaves-the-file` — A cleared watch leaves the file](#watchesa-cleared-watch-leaves-the-file--a-cleared-watch-leaves-the-file)
  - [`watches:a-watched-entry-is-not-eligible` — A watched entry is not eligible](#watchesa-watched-entry-is-not-eligible--a-watched-entry-is-not-eligible)
  - [`watches:a-blocked-id-exists` — A blocked id exists](#watchesa-blocked-id-exists--a-blocked-id-exists)
  - [`watches:a-stale-watch-is-reported` — A stale watch is reported](#watchesa-stale-watch-is-reported--a-stale-watch-is-reported)
  - [`watches:a-reading-carries-its-date-and-its-observation` — A reading carries its date and its observation](#watchesa-reading-carries-its-date-and-its-observation--a-reading-carries-its-date-and-its-observation)
  - [`watches:a-cadence-is-optional-at-both-levels` — A cadence is optional at both levels](#watchesa-cadence-is-optional-at-both-levels--a-cadence-is-optional-at-both-levels)
  - [`watches:a-cold-reading-is-announced-and-never-gates` — A cold reading is announced and never gates](#watchesa-cold-reading-is-announced-and-never-gates--a-cold-reading-is-announced-and-never-gates)
- [Unenforced rules](#unenforced-rules)
- [Page budget](#page-budget)

<!--TOC-->

## Purpose

The earned `watches.md` file at the plan zone root holds items this project waits on and does not own. A watch blocks named entries until a person confirms its exit condition. A project that waits on nothing carries no file, and an absent file fails no check.

A question ends when this project decides. A watch ends when somebody else acts. The watch domain owns the file, each section, and its blocking claim. The ranking domain owns what eligibility does with that claim.

A watch is local to the record that declares it. The attached closure does not read a peer's watches because this record cannot check or clear them. A peer's watch-blocked entry remains open in its own lane, and a prefixed dependency on that entry blocks here.

Validation includes the watch count in its census and reports zero when `watches.md` is absent. The count describes the record and does not add a default file.

`Q-` names a question section and `W-` names a watch section. A watch id is unique within `watches.md`, so a reader can cite it. Section ids do not enter the entry id namespace or consume tombstones.

## Section shape

```markdown
## W-the-workflow-ships-upstream — The convention carries the Scorecard workflow

Raised: 2026-09-09

Watch: `issues#117`

Until: closed as completed, and the landed version carries the workflow.

Blocks: hand-over-the-workflow — nothing exists to hand over until it ships.

Read: 2026-09-09 — open, and the snippet list names no such workflow.

Every: 30 days
```

The heading carries the id and text. `Raised` dates the watch. `Watch` names who or what clears it. `Until` states the exit condition. `Blocks` names the affected entries. `Read` dates the latest observation. The optional `Every` line sets this watch's reading cadence. The parts appear in that order.

The `Read` line carries the `YYYY-MM-DD` date when a person looked, an em dash, and one sentence describing what they saw. The sentence records a dated observation rather than a remote field. It can say that an item was open because the date keeps that statement true as an observation about that day.

A reading is cold when its date is older than the watch's `Every` period or, without that line, the project's `stale_after.watch_days` period. Neither level has a default. A project that sets neither receives no cold-reading warning.

The two project thresholds stay separate. `days` measures this project's own work going quiet, while `watch_days` measures an outside item going unread. A release cut monthly and a ticket triaged daily can also use different `Every` periods.

The remote system remains the source of truth under `external-sources:a-source-ref-is-stated-and-never-verified`. No check reads a tracker. A person reads the item, tests the exit condition, and edits the record.

A workflow verb announces a cold-reading count on stderr at exit 0 and names `wipctl stale` as the view that shows the watches. It folds both entry and watch counts against the one clock reading required by `metrics:the-clock-is-read-once`.

The count includes a watch whose `Read` date is older than its threshold and whose blocked work remains open. It excludes a watch read inside the threshold. It also excludes a watch whose blocked entries have all closed, because `watches:a-stale-watch-is-reported` already reports that record error.

Clearing a watch creates no separate event store. The section enters and leaves `watches.md` through ordinary commits, so `git log` on that file records when the block appeared and lifted, with its author and date. A future blocked-duration measure can fold from that history without storing or computing the measure here.

## Reference or phrase

The `Watch` line has two forms.

| Form      | Shape                                                                                                                                     |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| reference | The whole value is one inline-code `<alias>#<key>` token. The alias resolves in the plan configuration's `sources` section.               |
| phrase    | The value is a 60 to 400 character line. It names the actor who clears the block, such as a credential owner or a person who must decide. |

The reference form reuses the source reference whole. Its url template turns the key into a link. The phrase bound rejects `tbd` and nothing else. A reader decides whether the phrase names a real actor.

## Requirements

### `watches:a-watch-blocks-something` — A watch blocks something

Every watch section MUST carry a `Blocks` line naming at least one entry id.

#### Scenario: A watch blocks nothing

- GIVEN a watch left after its only entry is deleted
- WHEN validation runs
- THEN it fails because a watch that blocks no existing work belongs outside the record

Verify: `cargo nextest run --test validation`

### `watches:a-watch-names-what-clears-it` — A watch names what clears it

Every watch section MUST carry a `Watch` line containing one declared source reference or one bounded phrase.

#### Scenario: A watch names an undeclared source

- GIVEN a watch whose reference uses an alias absent from the `sources` section
- WHEN validation runs
- THEN it fails naming the alias because the record cannot resolve the item

Verify: `cargo nextest run --test validation`

### `watches:a-watch-states-a-testable-exit` — A watch states a testable exit

Every watch section MUST carry an `Until` line whose condition a person can test today.

#### Scenario: A watch waits until an item is fixed upstream

- GIVEN an outside item that can close without a fix reaching this project
- WHEN a reader tests the stated exit
- THEN the condition gives no answer, so it names closure and what must carry the fix

Verify: reviewer confirms every `Until` line can answer yes or no from evidence available today

### `watches:a-watch-carries-a-section-per-part` — A watch carries a section per part

Every watch section MUST carry its id and text, `Raised`, `Watch`, `Until`, `Blocks`, and `Read` in the stated order.

#### Scenario: A watch carries no reading

- GIVEN a watch with an outside item and no dated observation
- WHEN the document check runs
- THEN it fails because a reader cannot tell whether the block was read today or months ago

Verify: `cargo nextest run --test documents`

### `watches:a-watch-is-never-a-dependency` — A watch is never a dependency

A watch id MUST NOT appear in `needs`, enter the dependency graph, or contribute to a derived number, and the `Blocks` line MUST name the affected entries instead.

#### Scenario: A watch id appears in needs

- GIVEN an entry whose `needs` list names a watch
- WHEN validation runs
- THEN it fails because the record cannot sequence work it does not own

Verify: `cargo nextest run --test validation`

### `watches:a-cleared-watch-leaves-the-file` — A cleared watch leaves the file

When a watch meets its exit condition, the record MUST delete its section and MUST NOT retain a cleared state, strikethrough, or closed section.

#### Scenario: A person confirms the outside action

- GIVEN a watch whose exit condition is met
- WHEN the person updates the record
- THEN the section leaves the file because an earned file holds unmet watches alone

Verify: reviewer confirms a cleared watch is deleted instead of marked

### `watches:a-watched-entry-is-not-eligible` — A watched entry is not eligible

While an open watch names an entry on its `Blocks` line, the implementation MUST treat that entry as ineligible.

#### Scenario: The scheduled head waits on an outside item

- GIVEN a scheduled entry blocked by an open watch
- WHEN ranking derives eligibility
- THEN the entry is ineligible because work this record cannot order must happen first

Verify: `cargo nextest run --test ranking`

### `watches:a-blocked-id-exists` — A blocked id exists

Every id named on a watch's `Blocks` line MUST exist in some lane.

#### Scenario: A blocked entry is deleted

- GIVEN a watch naming an id that no longer exists
- WHEN validation runs
- THEN it fails because the watch blocks no record entry

Verify: `cargo nextest run --test validation`

### `watches:a-stale-watch-is-reported` — A stale watch is reported

At least one blocked id MUST still be open, and the implementation MUST report a watch whose blocked entries have all closed.

#### Scenario: The last blocked entry closes

- GIVEN a watch whose every blocked entry is finished
- WHEN validation runs
- THEN the watch is reported as stale because nobody needs to keep reading its outside item

Verify: `cargo nextest run --test validation`

### `watches:a-reading-carries-its-date-and-its-observation` — A reading carries its date and its observation

Every watch section MUST carry a `Read` line containing a `YYYY-MM-DD` date, an em dash, and a nonempty observation.

#### Scenario: A reading names the observed state

- GIVEN a `Read` line saying that the outside item was open on its stated date
- WHEN the document check runs
- THEN the line passes because it records what a person saw on that date rather than claiming a current remote state

Verify: `cargo nextest run --test documents`

### `watches:a-cadence-is-optional-at-both-levels` — A cadence is optional at both levels

Where a cold-reading cadence is configured, the implementation MUST use the watch's optional `Every` period before the project's optional `stale_after.watch_days` period, and without either it MUST announce no cold reading.

#### Scenario: A watch overrides the project cadence

- GIVEN a project cadence of 14 days and a watch cadence of 30 days
- WHEN the reading is 20 days old
- THEN no cold reading is announced because the period belongs to the item being watched

Verify: `cargo nextest run --test validation`

### `watches:a-cold-reading-is-announced-and-never-gates` — A cold reading is announced and never gates

When at least one open watch has a reading older than its resolved cadence, the implementation MUST announce the count once on stderr at exit 0 and name `wipctl stale` as the view that shows the watches.

#### Scenario: Two watch readings are cold

- GIVEN two watches past their resolved cadence and still blocking open entries
- WHEN a workflow verb finishes
- THEN one warning line reports both watches and the resolution while stdout and the exit code keep their ordinary result

Verify: `cargo nextest run --test verb_contracts`

## Unenforced rules

| Rule                                      | Why no command decides it                                                                             |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `watches:a-watch-names-what-clears-it`    | A count gates the phrase, but only a reader can decide whether it names the actor who clears a block. |
| `watches:a-watch-states-a-testable-exit`  | Whether a sentence states a condition a person can test is a reading rather than a text match.        |
| `watches:a-cleared-watch-leaves-the-file` | The record contains no remote fact that lets a command decide whether the exit condition is met.      |

## Page budget

The page budget reserves room for thirteen requirement blocks within the 300-line cap. The reading cadence is the split boundary if the page approaches that cap. This page spends two of the five prohibition lines and reserves three.
