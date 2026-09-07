# Repair Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`repair:the-repair-is-deterministic` — The repair is deterministic](#repairthe-repair-is-deterministic--the-repair-is-deterministic)
  - [`repair:the-repair-is-identity-preserving` — The repair is identity-preserving on legal input](#repairthe-repair-is-identity-preserving--the-repair-is-identity-preserving-on-legal-input)
  - [`repair:the-repair-is-idempotent` — The repair is idempotent](#repairthe-repair-is-idempotent--the-repair-is-idempotent)
  - [`repair:the-repair-is-non-canonical` — The repair is non-canonical](#repairthe-repair-is-non-canonical--the-repair-is-non-canonical)
  - [`repair:an-entry-block-moves-as-its-lines` — An entry block moves as its original lines](#repairan-entry-block-moves-as-its-lines--an-entry-block-moves-as-its-original-lines)
  - [`repair:a-stamped-instant-is-read-once` — A stamped instant is read once](#repaira-stamped-instant-is-read-once--a-stamped-instant-is-read-once)
  - [`repair:a-writer-refuses-broken-content` — A writer refuses broken content](#repaira-writer-refuses-broken-content--a-writer-refuses-broken-content)
  - [`repair:a-write-lands-through-a-temporary-file` — A write lands through a temporary file](#repaira-write-lands-through-a-temporary-file--a-write-lands-through-a-temporary-file)
  - [`repair:a-multi-item-operation-is-all-or-nothing` — A multi-item operation is all or nothing](#repaira-multi-item-operation-is-all-or-nothing--a-multi-item-operation-is-all-or-nothing)
  - [`repair:only-the-repair-reorders` — Only the repair reorders](#repaironly-the-repair-reorders--only-the-repair-reorders)
  - [`repair:the-repair-never-changes-a-lane` — The repair never changes a lane](#repairthe-repair-never-changes-a-lane--the-repair-never-changes-a-lane)
  - [`repair:the-slot-report-writes-nothing` — The slot report writes nothing](#repairthe-slot-report-writes-nothing--the-slot-report-writes-nothing)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What every writer owes the bytes it touches, and what the rank repair guarantees on top of that. The boundary runs at the act. This domain covers the mechanics of writing. The transactions domain covers the lock and the commit that wrap it, and each verb's own domain covers what it writes.

## Requirements

### `repair:the-repair-is-deterministic` — The repair is deterministic

The same record, the same invocation, and the same stamped instant MUST produce the same bytes.

#### Scenario: The same repair runs on two machines

- GIVEN one record and one invocation
- WHEN the repair runs on each
- THEN the bytes match, because a writer whose output varies cannot be reviewed as a diff

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-is-identity-preserving` — The repair is identity-preserving on legal input

A record that is already legal MUST pass through the repair unchanged, byte for byte, including the presence or absence of a final newline.

#### Scenario: The repair runs on a clean record

- GIVEN a record in a legal order
- WHEN the repair runs
- THEN nothing changes and no commit is owed, because a writer that always writes teaches its operator to stop reading its diffs

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-is-idempotent` — The repair is idempotent

Running the repair twice MUST equal running it once.

#### Scenario: The repair runs in a loop

- GIVEN a record repaired once
- WHEN the repair runs again
- THEN the result is unchanged, because a writer that oscillates has no fixed point to review

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-is-non-canonical` — The repair is non-canonical

The repair MUST restore legality and MUST NOT impose a canonical order.

#### Scenario: A person chose a legal order

- GIVEN a legal permutation a reader arranged deliberately
- WHEN the repair runs
- THEN it is a fixed point and is not disturbed, because any legal permutation is as correct as any other

Verify: `cargo nextest run --test writer_guarantees`

### `repair:an-entry-block-moves-as-its-lines` — An entry block moves as its original lines

A writer MUST relocate an entry block as its original lines and MUST NOT re-serialise the file.

#### Scenario: A lane file carries comments

- GIVEN a file with notes and blank lines between entries
- WHEN an entry moves
- THEN the comments and blank lines survive, because re-serialising discards everything the format does not model

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-stamped-instant-is-read-once` — A stamped instant is read once

Where a verb stamps an instant, the implementation MUST read the clock exactly once per operation.

#### Scenario: One operation writes two stamps

- GIVEN a transition writing an event and a close date
- WHEN both are stamped
- THEN they carry one instant, because two clock reads make one operation look like two

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-writer-refuses-broken-content` — A writer refuses broken content

A writer MUST NOT operate on a record whose content checks fail, and MUST exit with the check status naming each failure.

#### Scenario: A repair is asked to fix a cyclic graph

- GIVEN a record with a dependency cycle
- WHEN the repair runs
- THEN it refuses, because no legal order exists and inventing one hides the cycle

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-write-lands-through-a-temporary-file` — A write lands through a temporary file

A writer MUST write through a temporary file beside the target, and MUST rename it into place only when the content differs.

#### Scenario: A writer is interrupted mid-write

- GIVEN a process killed during a write
- WHEN the record is read afterwards
- THEN the target is whole and no temporary file remains, because a half-written record is worse than an unwritten one

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-multi-item-operation-is-all-or-nothing` — A multi-item operation is all or nothing

Where an operation covers many items, the implementation MUST complete all of them or none.

#### Scenario: One fragment of a drain is incomplete

- GIVEN a drain over four fragments, one of them missing a field
- WHEN it runs
- THEN nothing lands, because a partial drain leaves an operator guessing which half happened

Verify: `cargo nextest run --test writer_guarantees`

### `repair:only-the-repair-reorders` — Only the repair reorders

Only the rank repair MAY reorder a lane, invoked on explicit request or after a bottom-of-lane insertion.

#### Scenario: A drain would leave a ranking failure

- GIVEN fragments landing beneath an entry they depend on
- WHEN the drain completes
- THEN it reports the ranking failure and runs no repair, because a landing that silently reorders is a machine making a rank claim

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-never-changes-a-lane` — The repair never changes a lane

The repair MUST NOT move an entry to another lane, and a removal MUST NOT reorder.

#### Scenario: An ineligible entry sits in the scheduled lane

- GIVEN an entry the repair ranks below the eligible ones
- WHEN the repair runs
- THEN it stays in its lane, because moving an entry is a transition and a transition owes a journal event

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-slot-report-writes-nothing` — The slot report writes nothing

The slot report MUST write nothing and MUST refuse when content checks fail, exiting with the check status.

#### Scenario: A slot report runs over a broken record

- GIVEN a record with a failing content check
- WHEN the report is asked for legal candidates per position
- THEN it refuses, because a slot report over a broken record is advice about garbage

Verify: `cargo nextest run --test verb_contracts`

## Unenforced rules

| Rule                              | Why no command decides it                                                         |
| --------------------------------- | --------------------------------------------------------------------------------- |
| `repair:only-the-repair-reorders` | Whether a proposed verb reorders as a side effect is a reading of what it writes. |

The repair applies a stable topological sort over same-lane dependency edges in the planning lanes. In the scheduled lane it partitions eligible entries above ineligible ones. It sorts the closed lane by close date, stable on current position, so same-day closes keep their order.
