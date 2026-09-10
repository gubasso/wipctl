# Repair Specification

<!--TOC-->

- [Purpose](#purpose)
- [The slot repair](#the-slot-repair)
- [Requirements](#requirements)
  - [`repair:a-slot-repair-moves-local-state-in-order` — A slot repair moves local state in order](#repaira-slot-repair-moves-local-state-in-order--a-slot-repair-moves-local-state-in-order)
  - [`repair:a-slot-repair-locks-both-names` — A slot repair locks both names](#repaira-slot-repair-locks-both-names--a-slot-repair-locks-both-names)
  - [`repair:an-explicit-slot-name-is-free` — An explicit slot name is free](#repairan-explicit-slot-name-is-free--an-explicit-slot-name-is-free)
  - [`repair:a-slot-repair-is-machine-local` — A slot repair is machine-local](#repaira-slot-repair-is-machine-local--a-slot-repair-is-machine-local)
  - [`repair:the-repair-is-deterministic` — The repair is deterministic](#repairthe-repair-is-deterministic--the-repair-is-deterministic)
  - [`repair:the-repair-is-identity-preserving` — The repair is identity-preserving on legal input](#repairthe-repair-is-identity-preserving--the-repair-is-identity-preserving-on-legal-input)
  - [`repair:the-repair-is-idempotent` — The repair is idempotent](#repairthe-repair-is-idempotent--the-repair-is-idempotent)
  - [`repair:a-stamped-instant-is-read-once` — A stamped instant is read once](#repaira-stamped-instant-is-read-once--a-stamped-instant-is-read-once)
  - [`repair:a-writer-refuses-broken-content` — A writer refuses broken content](#repaira-writer-refuses-broken-content--a-writer-refuses-broken-content)
  - [`repair:a-write-lands-through-a-temporary-file` — A write lands through a temporary file](#repaira-write-lands-through-a-temporary-file--a-write-lands-through-a-temporary-file)
  - [`repair:a-multi-item-operation-is-all-or-nothing` — A multi-item operation is all or nothing](#repaira-multi-item-operation-is-all-or-nothing--a-multi-item-operation-is-all-or-nothing)
  - [`repair:the-repair-never-changes-a-lane` — The repair never changes a lane](#repairthe-repair-never-changes-a-lane--the-repair-never-changes-a-lane)
  - [`repair:the-repair-never-names-a-plan-or-a-dependency` — The repair never names a plan or a dependency](#repairthe-repair-never-names-a-plan-or-a-dependency--the-repair-never-names-a-plan-or-a-dependency)
  - [`repair:the-slot-report-writes-nothing` — The slot report writes nothing](#repairthe-slot-report-writes-nothing--the-slot-report-writes-nothing)
  - [`repair:the-repair-removes-an-orphaned-suppression` — The repair removes an orphaned suppression](#repairthe-repair-removes-an-orphaned-suppression--the-repair-removes-an-orphaned-suppression)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What every writer owes the bytes it touches, and what each supported repair may change. The boundary runs at the act. This domain covers the mechanics of writing. The transactions domain covers the lock and the commit that wrap it, and each verb's own domain covers what it writes.

## The slot repair

```text
wipctl fix --registry [<name>]
```

Without a name, the repair derives the project slug and escalates it until free. With a name, it uses that name and refuses an occupied destination.

The repair takes the locks for the current and new slot names in lexical order. It then moves three filesystem parts in order:

```text
1. move the slot
2. move the state directory
3. remove the old cache directory
```

Each boundary is resumable. A later run finds the moved slot, then completes any state move and cache removal left under the old name.

The repair writes no project file because a peer slot has no host repository. It commits and replicates nothing. It accepts the project underfoot and refuses the plan targeting flag.

```text
wipctl: slot renamed: payments -> payments-ben
wipctl: state moved:  <state dir>/projects/payments-ben
wipctl: cache removed: <cache dir>/projects/payments
```

## Requirements

### `repair:a-slot-repair-moves-local-state-in-order` — A slot repair moves local state in order

When a slot repair runs, the implementation MUST move the slot, move its state, and remove its old cache in order across resumable boundaries.

#### Scenario: A run stops after moving the slot

- GIVEN a slot under its new name and state under its old name
- WHEN the operator runs the same repair again
- THEN the repair moves the state and removes the old cache without merging either directory

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-slot-repair-locks-both-names` — A slot repair locks both names

When a slot repair runs, the implementation MUST take both slot-name locks in lexical order and hold them through the filesystem changes.

#### Scenario: Two repairs exchange names

- GIVEN two concurrent repairs whose current and requested names cross
- WHEN both take their locks
- THEN lexical order prevents each repair from holding the lock that the other needs

Verify: `cargo nextest run --test writer_guarantees`

### `repair:an-explicit-slot-name-is-free` — An explicit slot name is free

Where an operator supplies a slot name, the implementation MUST use it when free and refuse it when its destination exists.

#### Scenario: The requested name is occupied

- GIVEN a repair naming a slot that another plan already occupies
- WHEN the repair checks the destination
- THEN it refuses that one condition and leaves both slots unchanged

Verify: `cargo nextest run --test verb_contracts`

### `repair:a-slot-repair-is-machine-local` — A slot repair is machine-local

When a slot repair runs, the implementation MUST change only this machine's registry, state, cache, and name locks.

#### Scenario: A plan is attached only as a peer

- GIVEN a slot whose plan has no host repository on this machine
- WHEN its local repair runs from the project underfoot
- THEN no committed file changes and no remote receives a write

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-is-deterministic` — The repair is deterministic

The same record, the same invocation, and the same stamped instant MUST produce the same bytes.

#### Scenario: The same repair runs on two machines

- GIVEN one record and one invocation
- WHEN the repair runs on each
- THEN the bytes match, because a writer whose output varies cannot be reviewed as a diff

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-is-identity-preserving` — The repair is identity-preserving on legal input

A record and local state that already satisfy the repair target MUST pass through unchanged, byte for byte, including the presence or absence of a final newline.

#### Scenario: The repair finds no supported defect

- GIVEN a record and local state that already satisfy the requested repair
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

### `repair:a-stamped-instant-is-read-once` — A stamped instant is read once

Where a verb stamps an instant, the implementation MUST read the clock exactly once per operation.

#### Scenario: One operation writes two stamps

- GIVEN a transition writing an event and a close date
- WHEN both are stamped
- THEN they carry one instant, because two clock reads make one operation look like two

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-writer-refuses-broken-content` — A writer refuses broken content

A writer MUST NOT operate on a record whose content checks fail, and MUST exit with the check status naming each failure.

#### Scenario: A writer is asked to change a cyclic record

- GIVEN a record with a dependency cycle
- WHEN the writer runs
- THEN it refuses, because a write on broken content can hide the original failure

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-write-lands-through-a-temporary-file` — A write lands through a temporary file

A writer MUST write through a temporary file beside the target, and MUST rename it into place only when the content differs.

#### Scenario: A writer is interrupted mid-write

- GIVEN a process killed during a write
- WHEN the record is read afterwards
- THEN the target is whole and no temporary file remains, because a half-written record is worse than an unwritten one

Verify: `cargo nextest run --test writer_guarantees`

### `repair:a-multi-item-operation-is-all-or-nothing` — A multi-item operation is all or nothing

Where one repository operation covers many record items, the implementation MUST complete all of them or none.

#### Scenario: One fragment of a drain is incomplete

- GIVEN a drain over four fragments, one of them missing a field
- WHEN it runs
- THEN nothing lands, because a partial drain leaves an operator guessing which half happened

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-never-changes-a-lane` — The repair never changes a lane

The repair MUST leave every entry in its current lane.

#### Scenario: A local repair runs beside a plan record

- GIVEN entries distributed across the five lanes
- WHEN the repair runs
- THEN every entry stays in its lane, because moving an entry is a transition and a transition owes a journal event

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-repair-never-names-a-plan-or-a-dependency` — The repair never names a plan or a dependency

The repair MUST NOT write the peer table and MUST NOT edit a dependency list, bare or prefixed.

#### Scenario: A dependency names an alias the table no longer has

- GIVEN a record whose dependency points at a peer nothing declares
- WHEN the repair runs
- THEN it changes neither file. Choosing a peer is a naming judgment, and a dependency is a claim about the work that no tool can re-derive

Verify: `cargo nextest run --test writer_guarantees`

### `repair:the-slot-report-writes-nothing` — The slot report writes nothing

The slot report MUST write nothing and MUST refuse when content checks fail, exiting with the check status.

#### Scenario: A slot report runs over a broken record

- GIVEN a record with a failing content check
- WHEN the slot report is requested
- THEN it refuses, because a slot report over a broken record is advice about garbage

Verify: `cargo nextest run --test verb_contracts`

### `repair:the-repair-removes-an-orphaned-suppression` — The repair removes an orphaned suppression

When the repair finds a suppression whose id names no entry, the implementation MUST remove that suppression and report its id.

#### Scenario: A suppressed entry is deleted

- GIVEN a suppression whose entry no longer exists
- WHEN the repair runs
- THEN it removes and reports the suppression, because local state has no reminder left to hide

Verify: `cargo nextest run --test writer_guarantees`
