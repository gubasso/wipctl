# The writer lock is per project per machine and protects a transaction

## Context and Problem Statement

One live record per project is shared by every worktree and clone on the machine, so the lost update is no longer confined to one checkout. The lock that prevents it must widen with the record, without becoming a scheduler.

## Considered Options

- An exclusive per-project, per-machine lock held for one transaction
- The existing per-zone lock — a zone per checkout no longer exists; two checkouts would hold two locks over one record
- A lockless operation log with after-the-fact merge — sound, but it stops the lanes being the directly edited, directly ranked files the method rests on
- A durable queue — needs heartbeats, priorities, dead-waiter reaping, and a resident process; a second workflow system beside the one being built

## Decision Outcome

Chosen option: the transaction lock, keyed by `project_id`, living under the runtime directory. It is held from the deciding read to the final commit, rank repair included — never for a session; an agent implementing a story holds nothing. Hold times are subsecond, so bounded wait plus retry is the whole queue at the only depth that occurs. A waiting writer names the holder; expiry is exit 3; a dead holder's lock is released, never inherited. Readers never take it. The runtime directory is the one base directory the base-directory specification leaves without a default, so the replacement directory is named, warned about, and reported by `doctor` — an unstated fallback would be no lock at all in cron jobs and containers, exactly where agents run.

## Consequences

- Good: the same guarantee ADR-two-writers-cannot-lose-a-transition gave one checkout now spans the machine, and contention is measured in milliseconds.
- Bad: no lock spans machines, and none is pretended — between machines, reconciliation is `sync`'s job.

## Status

Accepted

Supersedes ADR-two-writers-cannot-lose-a-transition.
