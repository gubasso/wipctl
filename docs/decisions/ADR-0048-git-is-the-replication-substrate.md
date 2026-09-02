# ADR-0048: Git is the plan repository's replication substrate

## Context and Problem Statement

The plan repository needs history, replication between machines, and hosting an operator can choose — capabilities the record's plain files do not provide on their own.

## Considered Options

- Git, as the plan repository's own substrate: one trunk, commits made by the tool, push as backup and replication
- A bespoke sync protocol — reimplements replication that git already does well, and forfeits every forge an operator already has
- File synchronisation tools — a file transport does not understand the record's semantics; reconciliation must be semantic, or a machine chooses content
- No replication — the record dies with the disk, and a second machine starts blind

## Decision Outcome

Chosen option: git. The plan repository is a plain git repository with exactly one published branch, created by the tool. Push is backup and replication, never part of the write transaction. `sync` is fetch, reconcile, validate, fast-forward push; reconciliation is semantic, and a genuinely concurrent transition of one entry is reported as a conflict naming both sides, never resolved by whichever line won a textual merge. No force push, ever. The reference already names git and no other version control (ADR-0036); this record inverts the relationship — git was one opt-in convenience at the host's edge, and is now the substrate the plan repository stands on, while the host repository receives no git invocation from any verb.

## Consequences

- Good: replication, history, hosting, and branch protection come from a tool every operator already runs, and the record stays plain files.
- Bad: the program now requires git wherever a plan repository exists, rather than only where a config opted in.

## Status

Accepted

Supersedes ADR-0036.
