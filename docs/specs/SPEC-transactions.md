# Transactions Specification

<!--TOC-->

- [Purpose](#purpose)
- [The commit grammar](#the-commit-grammar)
- [Requirements](#requirements)
  - [`transactions:a-writer-holds-the-lock` — A writer holds the lock for one transaction](#transactionsa-writer-holds-the-lock--a-writer-holds-the-lock-for-one-transaction)
  - [`transactions:preflight-precedes-the-first-byte` — Preflight precedes the first byte](#transactionspreflight-precedes-the-first-byte--preflight-precedes-the-first-byte)
  - [`transactions:every-mutation-ends-in-one-commit` — Every mutation ends in one commit](#transactionsevery-mutation-ends-in-one-commit--every-mutation-ends-in-one-commit)
  - [`transactions:the-commit-grammar-is-the-tools` — The commit grammar is the tool's](#transactionsthe-commit-grammar-is-the-tools--the-commit-grammar-is-the-tools)
  - [`transactions:the-host-gains-no-commit` — The host gains no commit](#transactionsthe-host-gains-no-commit--the-host-gains-no-commit)
  - [`transactions:the-plan-hooks-are-never-bypassed` — The plan hooks are never bypassed](#transactionsthe-plan-hooks-are-never-bypassed--the-plan-hooks-are-never-bypassed)
  - [`transactions:a-waiting-writer-names-the-holder` — A waiting writer names the holder](#transactionsa-waiting-writer-names-the-holder--a-waiting-writer-names-the-holder)
  - [`transactions:the-wait-is-bounded` — The wait is bounded](#transactionsthe-wait-is-bounded--the-wait-is-bounded)
  - [`transactions:a-stale-holding-is-released` — A stale holding is released](#transactionsa-stale-holding-is-released--a-stale-holding-is-released)
  - [`transactions:the-lock-is-not-a-record` — The lock is not a record](#transactionsthe-lock-is-not-a-record--the-lock-is-not-a-record)
  - [`transactions:a-reader-never-takes-the-lock` — A reader never takes the lock](#transactionsa-reader-never-takes-the-lock--a-reader-never-takes-the-lock)
  - [`transactions:push-is-not-part-of-the-transaction` — Push is not part of the transaction](#transactionspush-is-not-part-of-the-transaction--push-is-not-part-of-the-transaction)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What a verb that writes the record must hold, and what its transaction must end in. Ten verbs write a maintained record: `new`, `land`, `move`, `start`, `delete`, `rename`, `fix`, `sync`, `resolve`, and `peer`. Closing is not a verb of its own, because it is `move --to closed`. The boundary runs at the write. This domain covers the lock, the preflight, and the commit. Each verb's own domain covers what it writes.

## The commit grammar

```text
plan: capture <id>
plan: move <id> from <from> to <to>
plan: close <id> as <outcome>
plan: land <n> fragments
plan: rename <old> to <new>
plan: fix ranking
plan: delete <id>
plan: reconcile <n> changes
plan: resolve <id> keeping <side>
```

## Requirements

### `transactions:a-writer-holds-the-lock` — A writer holds the lock for one transaction

When a verb writes a maintained record, it MUST hold the exclusive writer lock from the deciding read to the final commit.

#### Scenario: Two worktrees of one project write at once

- GIVEN two checkouts of the same project on one machine
- WHEN both write
- THEN the lock spans both, because it is keyed by project rather than by checkout. The mint reads the record it writes into

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:preflight-precedes-the-first-byte` — Preflight precedes the first byte

Before a writer writes anything, the implementation MUST run every check the write depends on, against the record at the current commit, inside the lock.

#### Scenario: A write is refused halfway

- GIVEN a check that fails only after two files are written
- WHEN the check runs first instead
- THEN a refused operation leaves every file byte-identical, because a half-written record is worse than a refused one

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:every-mutation-ends-in-one-commit` — Every mutation ends in one commit

Every semantic mutation MUST end in one commit on the plan trunk, inside the same transaction, staging only the paths the tool owns.

#### Scenario: A project wants the commit turned off

- GIVEN a request for a setting that batches plan commits
- WHEN the configuration is read
- THEN no such setting exists, because a record whose history is optional is a record nobody can replay

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:the-commit-grammar-is-the-tools` — The commit grammar is the tool's

The implementation MUST write each commit message in the fixed grammar above, and MUST NOT take the message from the project.

#### Scenario: A project has its own commit convention

- GIVEN a host repository with a different message style
- WHEN a plan mutation commits
- THEN the tool's grammar is used, because the messages are read back by the tool and not only by people

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:the-host-gains-no-commit` — The host gains no commit

A verb MUST NOT create a commit in the host repository.

#### Scenario: A capture runs inside the source tree

- GIVEN a capture from a working directory in the host
- WHEN the transaction ends
- THEN the host's history is untouched, because the record lives in its own repository

Verify: `cargo nextest run --test journey`

### `transactions:the-plan-hooks-are-never-bypassed` — The plan hooks are never bypassed

The implementation MUST run the plan repository's own hooks on every commit, and MUST NOT bypass them.

#### Scenario: A hook refuses the commit

- GIVEN a record written and a hook that refuses
- WHEN the transaction ends
- THEN the record stays written and the paths stay staged, and the run fails naming that state. A hidden abnormal state is the one nobody diagnoses

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:a-waiting-writer-names-the-holder` — A waiting writer names the holder

While a writer waits for the lock, the implementation MUST report that it waits, for which process, and for how long.

#### Scenario: A writer waits behind another

- GIVEN a lock held by a running verb
- WHEN a second writer starts
- THEN it names the holder and the elapsed time, because a silent wait is indistinguishable from a hang

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:the-wait-is-bounded` — The wait is bounded

The wait for the lock MUST be bounded, and on expiry the verb MUST exit naming the holder and the judgment to make.

#### Scenario: A holder is stuck rather than busy

- GIVEN a lock held past the bound
- WHEN the wait expires
- THEN the message says a transaction takes milliseconds, so the holder is stuck, and names both waiting and ending it as the choice

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:a-stale-holding-is-released` — A stale holding is released

Where the lock's holder is no longer alive, the implementation MUST release the lock rather than inherit it.

#### Scenario: A writer is killed mid-transaction

- GIVEN a lock whose holding process is gone
- WHEN the next writer starts
- THEN the lock is released and taken. A dead holder otherwise blocks the project until somebody finds the file

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:the-lock-is-not-a-record` — The lock is not a record

The lock MUST live outside the plan repository, MUST be absent from a clone, and MUST answer no question about the plan.

#### Scenario: Someone asks the lock who is working

- GIVEN a lock file on one machine
- WHEN it is read for that answer
- THEN it answers nothing, because it protects a transaction and not a session, and there is no durable queue and no promise of fairness

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:a-reader-never-takes-the-lock` — A reader never takes the lock

A read-only verb MUST NOT take the lock and MUST NOT block.

#### Scenario: A board renders during a write

- GIVEN a writer holding the lock
- WHEN a reader runs
- THEN the reader returns the committed state immediately, because blocking a view on a subsecond write buys nothing

Verify: `cargo nextest run --test writer_guarantees`

### `transactions:push-is-not-part-of-the-transaction` — Push is not part of the transaction

The implementation MUST NOT push as part of a mutation's transaction.

#### Scenario: A machine is offline

- GIVEN a capture on a machine with no network
- WHEN the transaction ends
- THEN the commit lands locally and replication stays its own verb. A write that needs a network fails on a train

Verify: `cargo nextest run --test writer_guarantees`

## Unenforced rules

| Rule                                    | Why no command decides it                                                                             |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `transactions:the-lock-is-not-a-record` | Whether a proposed lock field answers a question about the plan is a reading of what the field means. |
