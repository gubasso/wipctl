# Deletion Specification

## Purpose

Removal of an entry that never belonged in the record, leaving nothing but its journal. Deletion is not a cut. A cut entry was decided against. It stays in the closed lane, with its points in the epic denominator. A deleted entry keeps no lane presence at all. The boundary runs at the tombstone. This domain owns the removal, and the ids domain owns what the burned name means afterwards.

The refusal covers this record. A plan that needs this id through a prefixed dependency is not consulted, because no record stores who depends on it. Deleting an entry another plan names is legal here and fails there, at that plan's next validation, in a message naming this plan. Where the id is one another team relies on, tell them. That is a review responsibility, and the review spec carries it.

## Requirements

### `deletion:a-referenced-id-is-not-deleted` — A referenced id is not deleted

The implementation MUST refuse a deletion while anything in this record still names the id, and MUST name the referrer and the resolution.

#### Scenario: A peer depends on the target

- GIVEN an entry another plan reaches through a prefixed dependency
- WHEN the deletion runs
- THEN it succeeds, because no record stores who depends on it and the refusal can only see this one. That plan learns at its next validation, in a message naming this plan, so the operator tells its owners

Verify: `cargo nextest run --test verb_contracts`

### `deletion:deletion-leaves-a-tombstone` — Deletion leaves a tombstone

Deletion MUST remove the lane entry and the story document, MUST append a deletion event to the entry's journal, and MUST commit.

#### Scenario: An entry that never moved is deleted

- GIVEN an entry with no journal
- WHEN the deletion runs
- THEN the journal is created carrying one event, because the tombstone is the whole of what a deleted entry keeps

Verify: `cargo nextest run --test verb_contracts`

### `deletion:deletion-touches-only-its-own-entry` — Deletion touches only its own entry

Deletion MUST NOT remove an epic document, an initiative document, a sibling artifact directory, or any document the story referenced.

#### Scenario: A story has a sibling directory

- GIVEN a story with a dataset beside it
- WHEN the entry is deleted
- THEN the directory remains, because a verb that removes what it did not create removes work nobody agreed to lose

Verify: `cargo nextest run --test verb_contracts`

### `deletion:a-refused-deletion-writes-nothing` — A refused deletion writes nothing

Deletion MUST run every check before the first byte, MUST hold the lock for the whole transaction, and MUST leave every file byte-identical when refused.

#### Scenario: A reference is found halfway

- GIVEN a check that fails only after the lane entry is removed
- WHEN the check runs first instead
- THEN nothing is written, because a half-deleted entry is a record neither state describes

Verify: `cargo nextest run --test writer_guarantees`

### `deletion:a-dry-run-takes-no-lock` — A dry run takes no lock

A dry run MUST run the whole preflight, print the same report, warn that nothing was written, leave every file byte-identical, and take no lock.

#### Scenario: An operator previews a deletion

- GIVEN a dry run over an entry with references
- WHEN it completes
- THEN the refusal is reported without a lock, because a run that writes nothing is a read

Verify: `cargo nextest run --test verb_contracts`

### `deletion:there-is-no-undelete` — There is no undelete

The implementation MUST NOT offer an undelete verb, and the journal MUST NOT gain a reason column to support one.

#### Scenario: A deletion is regretted

- GIVEN an entry removed by mistake
- WHEN its author wants it back
- THEN the plan trunk's history holds the bytes, because the journal stays three columns and records no reason

Verify: `cargo nextest run --test verb_contracts`
