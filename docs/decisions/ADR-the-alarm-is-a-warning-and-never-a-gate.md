# The alarm is a warning and never a gate

## Context and Problem Statement

The stale threshold catches work that may have lost attention. A separate view reports stale rows only when a reader asks for it, so the threshold can stay silent.

## Considered Options

- Warn once from every workflow verb except the stale view itself — chosen.
- Fail a workflow verb when stale rows exist — rejected: a measure would become a gate on work.
- Store a cached stale count — rejected: the record already holds every fact needed to derive the count.

## Decision Outcome

Chosen option: warn once from every workflow verb except the stale view itself. The warning reaches stderr at exit 0 and names the command that shows the rows.

Enforced by `metrics:a-stale-row-is-announced-once-per-invocation` and `metrics:a-measure-is-the-same-on-any-clone`.

## Consequences

- Good: stale work reaches a reader during ordinary workflow operations.
- Good: stdout and the exit code keep their existing meanings.
- Bad: every workflow verb performs the age fold before it prints.

## Status

Accepted
