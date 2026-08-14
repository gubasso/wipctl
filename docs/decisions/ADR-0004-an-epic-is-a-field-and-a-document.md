# ADR-0004: An epic is a field and a document

## Context and Problem Statement

Some end states are visible from no single story. Naming them must not create a second store of membership or a covert scheduling mechanism.

## Considered Options

- One epic document plus an optional `epic` field on entries, membership pointing entry to document
- A member list in the epic document — a second store of membership, stale on the first entry change
- Epics as dependencies — an epic would sequence work, and ordering already has one owner, `needs`
- Nested epics — a plan becomes a filing system; every consumer pays traversal for the rare deep case

## Decision Outcome

Chosen option: a field and a document — membership points one way and nothing gains a second store. An epic is one document under `epics/` plus an optional `epic` field on entries. Ids come from the shared namespace, so one id names one thing. The document lists no members, carries no status, and has no parent. The reference is gated (the document must exist); the epic is never a dependency and never enters ranking. Completion is derived: closed out when every member is closed — and closed out is not done, which is why the document carries `Done when`.

## Consequences

- Good: an empty epic is a legal first draft, and epic arithmetic (delivered against promised) is derivable from lane files alone.
- Bad: retirement is a review question; no threshold derives it.

## Status

Accepted
