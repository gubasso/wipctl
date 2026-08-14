# ADR-0033: The host stays unassumed

## Context and Problem Statement

A planning method that dictates a documentation methodology excludes every project that already has
one — which is every project worth adopting into.

## Considered Options

- Assume exactly one thing: a documentation directory exists
- Requiring a documentation structure — a closer fit for projects that share it, exclusion for the
  rest; and the method's own value does not depend on it
- Gating `Governed by` — inbound sources are a judgment about the host's documents that resolves
  differently in every project; a gate would enforce the one thing the method refuses to assume

## Decision Outcome

Chosen option: the host stays unassumed — one assumption, a documentation directory. The product
asserts nothing about that directory's organisation, whether the project keeps decision records, or
what kind of document an outbound reference names. The config's path keys have no defaults, and
none may be added. The gate checks that a named `Amends` path resolves — nothing more; whether it
was the right document is a review responsibility, stated as such. `Governed by` paths are entirely
unchecked.

## Consequences

- Good: adoption costs one answered question.
- Bad: the method's honesty burden grows — every check it cannot run must be named as a review
  responsibility, which the review checklist carries.

## Status

Accepted
