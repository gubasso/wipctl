# Rename Specification

<!--TOC-->

- [Purpose](#purpose)
- [The write set](#the-write-set)
- [Requirements](#requirements)
  - [`rename:a-rename-holds-the-invariant` — A rename holds the title and id invariant](#renamea-rename-holds-the-invariant--a-rename-holds-the-title-and-id-invariant)
  - [`rename:a-rename-takes-a-title-an-id-or-both` — A rename takes a title, an id, or both](#renamea-rename-takes-a-title-an-id-or-both--a-rename-takes-a-title-an-id-or-both)
  - [`rename:the-write-set-covers-every-reference` — The write set covers every reference](#renamethe-write-set-covers-every-reference--the-write-set-covers-every-reference)
  - [`rename:history-travels-with-the-entry` — History travels with the entry](#renamehistory-travels-with-the-entry--history-travels-with-the-entry)
  - [`rename:the-new-id-mints-like-a-fresh-one` — The new id mints like a fresh one](#renamethe-new-id-mints-like-a-fresh-one--the-new-id-mints-like-a-fresh-one)
  - [`rename:the-old-id-is-burned` — The old id is burned](#renamethe-old-id-is-burned--the-old-id-is-burned)
  - [`rename:a-collision-recovery-burns-nothing` — A collision recovery burns nothing](#renamea-collision-recovery-burns-nothing--a-collision-recovery-burns-nothing)
  - [`rename:a-collision-recovery-rephrases` — A collision recovery rephrases](#renamea-collision-recovery-rephrases--a-collision-recovery-rephrases)
  - [`rename:a-rename-is-narrated-in-revisions` — A rename is narrated in revisions](#renamea-rename-is-narrated-in-revisions--a-rename-is-narrated-in-revisions)
  - [`rename:a-refused-rename-writes-nothing` — A refused rename writes nothing](#renamea-refused-rename-writes-nothing--a-refused-rename-writes-nothing)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Changing an artifact's title, its id, or both, holding the two in agreement. Renaming is a supported operation with rules, not a drift a reader has to tolerate. It binds every artifact the identity invariant binds: a story, an epic, an initiative, and a question. The boundary runs at the identity: the ids domain owns the invariant and the burn, and this domain owns the operation that changes one.

## The write set

```text
the lane entry's id, when an entry is renamed
the story document and its title line
the story's sibling directory, when one exists
the epic or initiative document, when one is renamed
the question's heading, when a question is renamed
the journal, history travelling with the entry
the pending fragment, when the entry is still a fragment
every dependency entry naming the old id, in every lane
every successor field naming the old id
every blocking line naming the old id
every epic field naming the old id, when an epic is renamed
every initiative section naming the old id, when an initiative is renamed
```

A question has no lane entry, no journal, and no fragment, so its rename touches its heading and its inbound references alone.

## Requirements

### `rename:a-rename-holds-the-invariant` — A rename holds the title and id invariant

The given id MUST be the resulting title's slug plus at most a qualifying postfix, and any other id MUST be a usage error.

#### Scenario: An unrelated id is passed

- GIVEN an id that is not the new title's slug or a postfix of it
- WHEN the invocation parses
- THEN it is a usage error, because the invocation itself is wrong, while every check against the record is a failed check instead

Verify: `cargo nextest run --test verb_contracts`

### `rename:a-rename-takes-a-title-an-id-or-both` — A rename takes a title, an id, or both

The rename verb MUST accept a new title, an explicit id, or both, and MUST treat neither as a usage error.

#### Scenario: A rename lands on a taken slug

- GIVEN a new title whose own slug is unavailable
- WHEN the operator supplies the postfixed id alongside it
- THEN both are taken together, which is the same pairing capture offers

Verify: `cargo nextest run --test verb_contracts`

### `rename:the-write-set-covers-every-reference` — The write set covers every reference

A rename MUST rewrite every artifact and every reference that names the old id, in one transaction.

#### Scenario: A dependency in another lane names the entry

- GIVEN an entry referenced from a planning lane and a blocking line
- WHEN the rename runs
- THEN both are rewritten, because a rename leaving one reference behind creates the dangling id it exists to prevent

Verify: `cargo nextest run --test verb_contracts`

### `rename:history-travels-with-the-entry` — History travels with the entry

A renamed entry's journal, document, sibling directory, and fragment MUST travel whole to the new id.

#### Scenario: A renamed entry is measured for flow

- GIVEN an entry with several transitions
- WHEN it is renamed
- THEN its arrival times are unchanged, because the history is the entry's and not the name's

Verify: `cargo nextest run --test verb_contracts`

### `rename:the-new-id-mints-like-a-fresh-one` — The new id mints like a fresh one

The new id MUST be minted under the same lock, against the live record and every tombstone, and a held id MUST be refused.

#### Scenario: A rename targets a deleted title

- GIVEN a new id held by a deletion tombstone
- WHEN the rename runs
- THEN it refuses naming the tombstone, because rename opens no second way to claim a burned name

Verify: `cargo nextest run --test verb_contracts`

### `rename:the-old-id-is-burned` — The old id is burned

A rename MUST write a fresh one-line tombstone at the old id whose event names the rename.

#### Scenario: An old reference survives somewhere outside the record

- GIVEN a link to the old id in a document the record does not own
- WHEN a reader follows it
- THEN it fails loudly against the tombstone rather than resolving to nothing, and rename opens no second way for an id to become free

Verify: `cargo nextest run --test verb_contracts`

### `rename:a-collision-recovery-burns-nothing` — A collision recovery burns nothing

Where a rename resolves a reported cross-machine id collision, the implementation MUST leave no tombstone, and MUST report that the id stayed unburned.

#### Scenario: Two machines minted one slug

- GIVEN a collision the reconciliation reported
- WHEN the losing capture is renamed
- THEN the id is not freed, because the surviving capture keeps it. Burning it refuses that claimant its own id

Verify: `cargo nextest run --test sync`

### `rename:a-collision-recovery-rephrases` — A collision recovery rephrases

A rename resolving a collision MUST take an id carrying no qualifying postfix, and a postfixed one MUST be a failed check.

#### Scenario: A postfix is passed during a recovery

- GIVEN the losing capture holding the base slug at the moment of the rename
- WHEN a postfixed id is passed
- THEN it is refused. This rename frees the base without burning it, so the committed result carries a postfix qualifying nothing

Verify: `cargo nextest run --test sync`

### `rename:a-rename-is-narrated-in-revisions` — A rename is narrated in revisions

A rename MUST be recorded as a dated revision line naming the old id, in the shapes that carry a revisions section.

#### Scenario: A question is renamed

- GIVEN a question, which carries no revisions section
- WHEN it is renamed
- THEN it owes no line, because its rename is carried by the heading itself and by the plan trunk commit

Verify: `cargo nextest run --test verb_contracts`

### `rename:a-refused-rename-writes-nothing` — A refused rename writes nothing

A rename MUST run every check before the first byte, MUST commit once, and MUST leave every file byte-identical when refused.

#### Scenario: The new id turns out to be taken

- GIVEN a rename whose target id is held
- WHEN the preflight refuses
- THEN no file changed, because a rename that half-applies leaves references pointing at two names

Verify: `cargo nextest run --test writer_guarantees`

## Unenforced rules

| Rule                                       | Why no command decides it                                                         |
| ------------------------------------------ | --------------------------------------------------------------------------------- |
| `rename:a-rename-is-narrated-in-revisions` | Whether the line explains the rename to a later reader is a reading of the prose. |

Rephrasing is what a collision asks for anyway: two captures that slug identically wanted different names. The report emits one line per rewritten path and offers no machine format, so it owes no schema.
