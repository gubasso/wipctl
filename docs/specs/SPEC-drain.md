# Drain Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`drain:the-order-is-total-and-derived` — The order is total and derived](#drainthe-order-is-total-and-derived--the-order-is-total-and-derived)
  - [`drain:the-drain-reads-no-history` — The drain reads no history](#drainthe-drain-reads-no-history--the-drain-reads-no-history)
  - [`drain:the-report-changes-nothing` — The report changes nothing](#drainthe-report-changes-nothing--the-report-changes-nothing)
  - [`drain:a-lost-position-lands-at-the-bottom` — A lost position lands at the bottom](#draina-lost-position-lands-at-the-bottom--a-lost-position-lands-at-the-bottom)
  - [`drain:the-drain-is-all-or-nothing` — The drain is all or nothing](#drainthe-drain-is-all-or-nothing--the-drain-is-all-or-nothing)
  - [`drain:an-incomplete-fragment-is-refused` — An incomplete fragment is refused](#drainan-incomplete-fragment-is-refused--an-incomplete-fragment-is-refused)
  - [`drain:landing-writes-no-journal-event` — Landing writes no journal event](#drainlanding-writes-no-journal-event--landing-writes-no-journal-event)
  - [`drain:the-drain-repairs-no-ranking` — The drain repairs no ranking](#drainthe-drain-repairs-no-ranking--the-drain-repairs-no-ranking)
  - [`drain:a-second-run-lands-nothing` — A second run lands nothing](#draina-second-run-lands-nothing--a-second-run-lands-nothing)

<!--TOC-->

## Purpose

The drain reconciles pending fragments into the lane files, in an order every machine derives identically, with every conflict reported rather than silently merged. The boundary runs at the fragment: the pending fragment domain owns the file and its drift vocabulary, and this domain owns the landing.

## Requirements

### `drain:the-order-is-total-and-derived` — The order is total and derived

The drain MUST land fragments ascending by the stated capture instant, with a residual tie on the full id compared lexically as one string.

#### Scenario: Two machines drain the same fragments

- GIVEN one set of fragments on two machines
- WHEN each drains
- THEN the orders agree, because the key is total and nothing parses an id into parts

Verify: `cargo nextest run --test drain`

### `drain:the-drain-reads-no-history` — The drain reads no history

The drain MUST NOT read repository history, a clock, a commit, or a reflog.

#### Scenario: History is rewritten between two runs

- GIVEN a squashed or truncated history
- WHEN the drain runs
- THEN the order is unchanged, because it derives from the fragments alone

Verify: `cargo nextest run --test drain`

### `drain:the-report-changes-nothing` — The report changes nothing

The report mode MUST print the resolved landing order and every drift line, and MUST leave the zone byte-for-byte unchanged, fragments included.

#### Scenario: An operator previews a landing

- GIVEN a report over several fragments
- WHEN it completes
- THEN nothing changed, and an absent or empty capture directory succeeds saying there is nothing to land

Verify: `cargo nextest run --test verb_contracts`

### `drain:a-lost-position-lands-at-the-bottom` — A lost position lands at the bottom

Where a fragment's position target is gone, moved, or closed, the fragment MUST land at the bottom of its claimed lane.

#### Scenario: A position target closed while the capture waited

- GIVEN a fragment positioned beneath an entry that has since closed
- WHEN the drain runs
- THEN it lands at the bottom and the drift is reported, because rank is a claim and only a person makes one

Verify: `cargo nextest run --test drain`

### `drain:the-drain-is-all-or-nothing` — The drain is all or nothing

Any content-check failure or a cyclic landed result MUST refuse the whole drain, and no flag MUST allow a partial landing.

#### Scenario: One fragment of four is broken

- GIVEN a set whose landed result is cyclic
- WHEN the drain runs
- THEN nothing lands, because a partial drain leaves an operator guessing which half happened

Verify: `cargo nextest run --test drain`

### `drain:an-incomplete-fragment-is-refused` — An incomplete fragment is refused

The drain MUST refuse a fragment missing a field a lane entry requires, naming the fragment, the field, and the edit that completes it.

#### Scenario: A capture was committed without its points

- GIVEN a fragment written with the point value absent
- WHEN the drain runs
- THEN it refuses naming the file and the field, because the drain must not write a lane entry the lane schema rejects

Verify: `cargo nextest run --test drain`

### `drain:landing-writes-no-journal-event` — Landing writes no journal event

The drain MUST NOT write a journal event, and MUST NOT land into a work lane.

#### Scenario: A fragment claims a work lane

- GIVEN a fragment whose claimed lane is in flight
- WHEN the drain runs
- THEN it refuses, because landing is a creation and a landed entry's journal begins at its first move

Verify: `cargo nextest run --test drain`

### `drain:the-drain-repairs-no-ranking` — The drain repairs no ranking

The drain MUST report a resulting ranking failure and MUST NOT repair it.

#### Scenario: A landing leaves an illegal order

- GIVEN fragments landing above entries they depend on
- WHEN the drain completes
- THEN the ranking failure is reported and the repair stays a separate act. A landing that silently reorders is a machine making a rank claim

Verify: `cargo nextest run --test drain`

### `drain:a-second-run-lands-nothing` — A second run lands nothing

After a successful drain, a second run MUST land nothing and succeed.

#### Scenario: The drain runs twice

- GIVEN a capture directory emptied by the first run
- WHEN the second runs
- THEN it says there is nothing to land, because the shared writer discipline makes a re-run idempotent

Verify: `cargo nextest run --test drain`
