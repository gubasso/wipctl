# ADR-0005: The plan record stays in version control

## Context and Problem Statement

A record could live in a database, a service, or a per-user store, each promising faster queries or richer coordination. The method's promise is a plan that travels with the project it plans.

## Considered Options

- Plain files, versioned with the project
- An authoritative database — fails the clean-clone test: a fresh clone would not carry the plan; and it splits the record's history from the code's
- A sync service — adds an availability dependency to a method whose whole promise is plain files

## Decision Outcome

Chosen option: plain files versioned with the project — a clone carries the whole record. A branch is the record's proposal mechanism; a merge is its coordination mechanism; a clone is its distribution mechanism. No database, no service, and no per-user store holds any fact a gate consults. A user-level path may hold only a disposable cache that no verb reads back.

## Consequences

- Good: concurrency is solved the version-control way — disjoint files (capture), a zone lock (one working tree), and a derived order every clone agrees on (the drain).
- Bad: every question must be answerable by parsing files, which bounds query cost and shapes the canonical subset.

## Status

Accepted
