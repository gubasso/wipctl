# Plan documents

The heading shapes of the documents in the plan zone: the story, epic, and initiative sequences, the question section, and the watch section. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## The three heading sequences

| Story            | Epic           | Initiative     |
| ---------------- | -------------- | -------------- |
| —                | `Initiative`   | —              |
| `Goal`           | `Goal`         | `Goal`         |
| `Example`        | `Example`      | `Example`      |
| `Core`           | `Core`         | `Core`         |
| `In scope`       | —              | —              |
| `Out of scope`   | `Out of scope` | `Out of scope` |
| `Reads`          | `Reads`        | `Reads`        |
| `Amends`         | `Amends`       | `Amends`       |
| `Sources`        | `Sources`      | `Sources`      |
| `Acceptance`     | —              | —              |
| —                | `Done when`    | `Done when`    |
| `Tasks`          | —              | —              |
| `Rabbit holes`   | —              | —              |
| `Refused option` | —              | —              |
| `Revisions`      | `Revisions`    | `Revisions`    |

Rules that bind this section: `documents:every-heading-is-present-in-order`.

## The question section

```markdown
## Q-which-proof-closes-the-risk — Which proof closes the risk?

Raised: 2026-08-14

Blocks: choose-the-boundary, verify-the-contract — both entries depend on the answer.

Exit: a decision record, measured against a worked example of both choices.
```

Rules that bind this section: `open-questions:a-section-carries-its-four-parts`.

## The watch section

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

The `Read` line carries the `YYYY-MM-DD` date when a person looked, an em dash, and one sentence describing what they saw.

A reading is cold when its date is older than the watch's `Every` period or, without that line, the project's `stale_after.watch_days` period. Neither level has a default. A project that sets neither receives no cold-reading warning.

The remote system remains the source of truth under `external-sources:a-source-ref-is-stated-and-never-verified`. No check reads a tracker. A person reads the item, tests the exit condition, and edits the record.

A workflow verb announces a cold-reading count on stderr at exit 0 and names `wipctl stale` as the view that shows the watches. It folds both entry and watch counts against the one clock reading required by `metrics:the-clock-is-read-once`.

The count includes a watch whose `Read` date is older than its threshold and whose blocked work remains open. It excludes a watch read inside the threshold. It also excludes a watch whose blocked entries have all closed, because `watches:a-stale-watch-is-reported` already reports that record error.

The source view marks a reference-form watch as `still listed`, `no longer listed`, or `unjudged`. The words `no longer listed` mean only that the item left the command's open list. A person compares that fact with the `Until` condition. The view reports a phrase-form watch as `skipped` because it has no alias or key to compare.

The mark is not a check, and it changes no exit code. It is not a clear, so the watch stays until a person edits the record. It is not eligibility, which derives from `watches.md` rather than the view. It is not a store under `external-sources:the-view-cache-is-authoritative-for-nothing`, because every run derives the report again.

Rules that bind this section: `watches:a-watch-carries-a-section-per-part`, `watches:a-reading-carries-its-date-and-its-observation`, `watches:a-cadence-is-optional-at-both-levels`, `external-sources:a-source-ref-is-stated-and-never-verified`, `watches:a-cold-reading-is-announced-and-never-gates`, `metrics:the-clock-is-read-once`, `watches:a-stale-watch-is-reported`, `watches:the-view-reports-a-watch-and-decides-nothing`, `watches:a-watched-entry-is-not-eligible`, `external-sources:the-view-cache-is-authoritative-for-nothing`.

## Reference or phrase

The `Watch` line has two forms.

| Form      | Shape                                                                                                                                     |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| reference | The whole value is one inline-code `<alias>#<key>` token. The alias resolves in the plan configuration's `sources` section.               |
| phrase    | The value is a 60 to 400 character line. It names the actor who clears the block, such as a credential owner or a person who must decide. |

The reference form reuses the source reference whole. Its url template turns the key into a link. The phrase bound rejects `tbd` and nothing else. A reader decides whether the phrase names a real actor.

Rules that bind this section: `watches:a-watch-names-what-clears-it`.
