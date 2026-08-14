# ADR-0028: A concurrent capture is a lane delta

## Context and Problem Statement

Two sessions adding work must not conflict in the lane files, and must not create a second copy of
the record to merge later.

## Considered Options

- A pending fragment stating a lane delta
- A parallel snapshot of the plan directory — a second record; every later reconciliation is a
  merge of two truths
- Version-control worktrees or branches as the capture mechanism — demands a git practice, which
  exceeds the host assumption
- A merge driver resolving lane conflicts — a driver that resolves a rank is a machine choosing
  priority

## Decision Outcome

Chosen option: a lane delta in a pending fragment — captures stay disjoint files and the lane files
stay the one ranked truth. A capture writes a story document plus one pending fragment stating a
delta: the lane the entry wants, the position it claims, the instant, and the entry itself —
nothing a lane file already holds. Two bounds are fixed here: the `pending/` directory is created
on demand and its absence is legal; and a fragment may target the two planning lanes only, because
landing into a work lane is a transition owing a journal event.

## Consequences

- Good: parallel captures are disjoint files that merge with no conflict.
- Bad: a fragment can drift from the lane files, which is why the drain reports drift instead of
  trusting fragments.

## Status

Accepted

Amended by ADR-0038 — the no-conflict merge consequence is conditional on no uid collision, with
re-minting as the recovery.
