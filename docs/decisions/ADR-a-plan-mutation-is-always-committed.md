# A plan mutation is always committed, in the plan trunk's grammar

## Context and Problem Statement

ADR-a-transition-commit-is-generated-and-opt-in made the transition commit opt-in, which was right while the record lived in the host's working tree: the operator owned the history the commit landed in, and a project could reasonably want the plan out of its log. The plan repository is the tool's own history, and a mutation that is not committed is a mutation the next reader and the next machine cannot see.

## Considered Options

- Every semantic mutation is one commit on the plan trunk, with no configuration that turns it off
- Keep the opt-in — opting out would opt out of the record: `sync` replicates commits, so an uncommitted mutation never leaves the machine
- Batch commits on some cadence — invents an interval nobody asked for, and a crash between batches loses the boundary between mutations

## Decision Outcome

Chosen option: always committed. One semantic mutation is one transaction and one commit — a `move` touching two lanes and a journal is one commit, never one commit per keystroke. The tool stages only paths it owns, and the fixed message grammar extends the transition-commit shape: `plan: capture <id>`, `plan: move <id> from <from> to <to>`, `plan: close <id> as <outcome>`, `plan: land <n> fragments`, `plan: rename <old> to <new>`, `plan: fix ranking`, `plan: delete <id>`. The grammar is the tool's, not the project's, because the plan repository has one writer. The host repository gains no commit from any verb — what was opt-in is now absent rather than mandatory on that side, and the `commit` table leaves the host config.

## Consequences

- Good: plan history is uniform, searchable, and complete; replication has something to replicate.
- Bad: the plan trunk's log is verbose by design, one commit per mutation.

## Status

Accepted

Supersedes ADR-a-transition-commit-is-generated-and-opt-in.
