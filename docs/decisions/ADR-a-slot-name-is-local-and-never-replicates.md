# A slot name is local and never replicates

## Context and Problem Statement

A plan needs a readable directory on each machine. A committed identity cannot safely name that directory because another machine can already use the same name.

## Considered Options

- A local slug stored only as the slot directory, minted by postfix escalation — chosen.
- A committed slug used as both identity and directory — rejected because a replicated rename can collide with a local slot.
- An opaque identity used as the directory — rejected because people meet the directory in paths and diagnostics.

## Decision Outcome

Chosen option: a local slot name. Each machine mints a readable slug once and records it only as the directory basename. The name does not replicate, so another machine can choose a different free name for the same plan. The last escalation step uses a sequential number, so a free name always exists.

Enforced by `slot-naming:a-slot-name-is-minted-once`, `slot-naming:a-slot-name-is-recorded-only-as-the-directory`, `slot-naming:a-slot-name-never-replicates`, and `slot-naming:a-taken-name-escalates`.

## Consequences

- Good: a name collision on one machine does not change any committed file.
- Bad: two machines can show the same plan under different readable names.

## Status

Accepted
