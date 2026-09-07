# Transitions Specification

<!--TOC-->

- [Purpose](#purpose)
- [The move preflight, in order](#the-move-preflight-in-order)
- [Requirements](#requirements)
  - [`transitions:a-lane-change-is-the-verb` — A lane change is the verb and never an edit](#transitionsa-lane-change-is-the-verb--a-lane-change-is-the-verb-and-never-an-edit)
  - [`transitions:the-preflight-runs-entire-and-in-order` — The preflight runs entire and in order](#transitionsthe-preflight-runs-entire-and-in-order--the-preflight-runs-entire-and-in-order)
  - [`transitions:a-lost-race-is-a-usage-error` — A lost race is a usage error](#transitionsa-lost-race-is-a-usage-error--a-lost-race-is-a-usage-error)
  - [`transitions:arrival-makes-no-rank-claim` — Arrival makes no rank claim](#transitionsarrival-makes-no-rank-claim--arrival-makes-no-rank-claim)
  - [`transitions:a-close-date-is-pinned-not-backdated` — A close date is pinned, not backdated](#transitionsa-close-date-is-pinned-not-backdated--a-close-date-is-pinned-not-backdated)
  - [`transitions:an-emptied-lane-redeclares-itself` — An emptied lane redeclares itself](#transitionsan-emptied-lane-redeclares-itself--an-emptied-lane-redeclares-itself)
  - [`transitions:reopening-strips-and-records` — Reopening strips and records](#transitionsreopening-strips-and-records--reopening-strips-and-records)
  - [`transitions:a-dry-run-takes-no-lock` — A dry run takes no lock](#transitionsa-dry-run-takes-no-lock--a-dry-run-takes-no-lock)
  - [`transitions:taking-work-is-atomic` — Taking work is atomic](#transitionstaking-work-is-atomic--taking-work-is-atomic)
  - [`transitions:taking-prints-what-the-preview-would` — Taking prints what the preview would](#transitionstaking-prints-what-the-preview-would--taking-prints-what-the-preview-would)
  - [`transitions:nothing-startable-terminates-the-loop` — Nothing startable terminates the loop](#transitionsnothing-startable-terminates-the-loop--nothing-startable-terminates-the-loop)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

The lane change as a recorded event, and the verb that takes the next entry atomically. An edit changes where an entry sits and records no when, so a lane change is always a verb. The boundary runs at the event: this domain owns the move and the take, while the transition journal domain owns the stream they append to.

## The move preflight, in order

1. The record already validates.
2. The destination is a real lane.
3. The id is in some lane.
4. The entry is not already in the destination.
5. The closing flags are consistent.
6. An entry entering a work lane has every dependency closed and no blocking question.

## Requirements

### `transitions:a-lane-change-is-the-verb` — A lane change is the verb and never an edit

A lane change MUST go through the move verb, and MUST NOT be made by editing a lane file.

#### Scenario: An operator moves an entry by hand

- GIVEN an entry relocated in the files
- WHEN validation runs
- THEN the lane-agreement gate fails, because an edit changes where an entry sits and records no when

Verify: `cargo nextest run --test validation`

### `transitions:the-preflight-runs-entire-and-in-order` — The preflight runs entire and in order

The implementation MUST run the whole preflight in the stated order before the first byte, and a refused move MUST leave every file byte-identical.

#### Scenario: An entry enters a work lane with an open dependency

- GIVEN an entry whose dependency is not closed
- WHEN the move runs
- THEN it fails naming the dependency and the act that unblocks it, and nothing is written

Verify: `cargo nextest run --test verb_contracts`

### `transitions:a-lost-race-is-a-usage-error` — A lost race is a usage error

When an entry is already in the destination lane, the implementation MUST report a usage error rather than a failed check.

#### Scenario: Two agents move one entry

- GIVEN a second agent whose target was already taken
- WHEN the move runs
- THEN it is a usage error so a retry loop cannot spin, and the message names taking the next entry instead

Verify: `cargo nextest run --test verb_contracts`

### `transitions:arrival-makes-no-rank-claim` — Arrival makes no rank claim

A moved entry's original lines MUST be appended at the bottom of the destination lane, and MUST NOT be inserted anywhere else.

#### Scenario: An entry arrives in a ranked lane

- GIVEN a destination whose order a person arranged
- WHEN the entry lands at the bottom
- THEN the repair restores legality and any finer rank stays a person's act, and appending keeps the closed lane in ascending date order

Verify: `cargo nextest run --test writer_guarantees`

### `transitions:a-close-date-is-pinned-not-backdated` — A close date is pinned, not backdated

Where a close date is given, it MUST equal the day of the once-read transition instant, and a mismatch MUST be a usage error.

#### Scenario: An operator supplies yesterday's date

- GIVEN an explicit date that is not today
- WHEN the move runs
- THEN it is a usage error, because the closing journal event must enter the closed lane on the entry's own close date

Verify: `cargo nextest run --test verb_contracts`

### `transitions:an-emptied-lane-redeclares-itself` — An emptied lane redeclares itself

A lane emptied by a move MUST redeclare an empty sequence, and a lane gaining its first entry MUST convert that declaration back.

#### Scenario: The last entry leaves a lane

- GIVEN a lane whose only entry moves away
- WHEN the write completes
- THEN the file declares an empty sequence, because a lane key with nothing under it is not the shape the schema accepts

Verify: `cargo nextest run --test validation`

### `transitions:reopening-strips-and-records` — Reopening strips and records

When an entry leaves the closed lane, the block MUST travel with its closing fields stripped, and the departure event MUST be appended.

#### Scenario: Reopened work enters a work lane

- GIVEN an entry moving from closed into review
- WHEN the move runs
- THEN it is gated like any other entry into a work lane, and re-closing later records a fresh outcome without recovering the prior one

Verify: `cargo nextest run --test validation`

### `transitions:a-dry-run-takes-no-lock` — A dry run takes no lock

A dry run MUST run the whole preflight, print the same report, warn that nothing was written, leave every file byte-identical, and take no lock.

#### Scenario: An operator previews a close

- GIVEN a dry run of a closing move
- WHEN it completes
- THEN no lock was taken, because a run that writes nothing is a read

Verify: `cargo nextest run --test verb_contracts`

### `transitions:taking-work-is-atomic` — Taking work is atomic

The take verb MUST read the head, preflight, write, repair, and commit inside one lock holding, choosing the id inside that lock.

#### Scenario: Two agents take work at the same instant

- GIVEN two agents running the take verb together
- WHEN both proceed
- THEN they get two different entries, because the second reads a lane whose head the first already removed

Verify: `cargo nextest run --test writer_guarantees`

### `transitions:taking-prints-what-the-preview-would` — Taking prints what the preview would

The take verb MUST print exactly the line the preview verb prints for the entry it took.

#### Scenario: A caller parses the output of either verb

- GIVEN one line naming the id, the title, the points, and the absolute path
- WHEN either verb prints it
- THEN the shapes match, and the one-line output owes no machine format

Verify: `cargo nextest run --test verb_contracts`

### `transitions:nothing-startable-terminates-the-loop` — Nothing startable terminates the loop

Where nothing is startable, the take verb MUST fail naming why, and MUST NOT take an entry out of a work lane.

#### Scenario: A fan-out loop finds an empty lane

- GIVEN a scheduled lane that is empty or whose head is ineligible
- WHEN the take verb runs
- THEN it fails naming the reason so the loop terminates, and entries already in flight do not stop it

Verify: `cargo nextest run --test verb_contracts`

## Unenforced rules

| Rule                                    | Why no command decides it                                                                  |
| --------------------------------------- | ------------------------------------------------------------------------------------------ |
| `transitions:a-lane-change-is-the-verb` | The gate catches a hand edit through lane agreement, but nothing proves the verb was used. |

The preview verb tells a reader what is next and the take verb takes it. With several agents pulling from one lane, every agent running the preview sees the same head, and all but one lose the race. The take verb exists to remove that case. The two verbs are twins and each names the other.
