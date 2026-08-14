# ADR-0035: An Amends path may be promised

## Context and Problem Statement

A story that creates a document cannot satisfy an existence check on the path it names before the work is done.

## Considered Options

- A `new:` marker that promises the document and exempts the path from the existence check
- Stub files created to satisfy the check — the record would instruct people to create empty documents so a machine stops complaining; the check starts serving itself
- Exempting only until the story starts — the exemption would need a state machine over lanes for a distinction reviews already hold

## Decision Outcome

Chosen option: a promised path behind `new:` — the exemption is visible in the record and judged in review. An `Amends` assertion opening with `new:` promises the document: the path is exempt from the existence check for the whole life of the entry, while every shape rule (relative, no dot segments, leading inline-code token) still applies. Whether the promise was kept is a review responsibility.

## Consequences

- Good: the gate stays strict for the common case (a named existing document) and honest about the exception, and `new:` markers are cheap to scan in review.
- Bad: a broken promise is invisible to every gate; only review catches it.

## Status

Accepted
