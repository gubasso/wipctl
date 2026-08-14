# ADR-0029: The drain orders by the instant the capture stated

## Context and Problem Statement

When several fragments land at once, their relative order must be derived identically on every
machine, from the record alone.

## Considered Options

- Ascending by the stated `captured` instant, ties broken on the uid
- Topological history order — reads repository history, which the charter excludes as a store, and
  shallow clones cannot answer
- Filesystem timestamps — destroyed by checkout, copy, and archive
- A counter file — forbidden by ADR-0026
- Fractional indexing between neighbours — encodes rank into the id space, and rank is a claim a
  person makes after landing

## Decision Outcome

Chosen option: order by the stated instant — the only candidate that is a record fact. The drain
lands fragments ascending by `captured`, with ties broken on the uid. Nothing reads repository
history, a filesystem timestamp, a counter, or a clock at drain time. The cost is stated plainly:
clock skew between machines can order two captures differently than wall-clock truth, and the
method accepts it, because the stated instant is a record fact and every alternative is not.

## Consequences

- Good: `land --report` is reproducible everywhere, including depth-1 clones.
- Bad: fine-grained ordering between near-simultaneous captures is settled by a person after the
  drain, with the repair's help.

## Status

Accepted
