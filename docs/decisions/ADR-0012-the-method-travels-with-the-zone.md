# ADR-0012: The method travels with the zone

## Context and Problem Statement

An agent or newcomer working in an adopting project needs the method without leaving the project or fetching anything.

## Considered Options

- One self-sufficient method document written into the zone
- Linking to hosted documentation — fails offline and versions independently of the build that scaffolded the zone
- Copying the full documentation set into the zone — a second source of truth at drift scale; the zone wants the working method, not the library

## Decision Outcome

Chosen option: one self-sufficient `AGENTS.md` in the zone — the method survives alone, offline, at the version the scaffold carries. The document holds both heading sequences, the entry fields and the canonical subset, the ranking rules, the move-and-close procedure, capture and drain, the reference split, what no gate catches, and the gates as literal commands. Gate names are substituted at emission from what the build actually carries; a verb the build lacks is named as absent rather than promised. The document carries no relative links into the product's own repository, because it must survive alone.

## Consequences

- Good: version skew is handled at emission time by substitution, and the method document is the one file the tool owns inside a zone it created and may update on a re-run of the scaffold.
- Bad: the zone restates method facts the library also holds, a duplication accepted for self-sufficiency.

## Status

Accepted
