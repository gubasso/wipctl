# A suppression is local and never replicates

## Context and Problem Statement

A stale reminder can stay useful to one operator and become noise to another. Silencing it in the plan record would make one operator's choice travel to every clone.

## Considered Options

- Store suppressions in this machine's state directory — chosen.
- Store suppressions in the plan configuration — rejected: the silence would replicate to the whole team.
- Store a flag on the lane entry — rejected: the record would gain a local preference as a durable fact.
- Store suppressions in the cache directory — rejected: the cache is disposable and no verb reads it back.

## Decision Outcome

Chosen option: store suppressions in this machine's state directory. A suppression hides one reminder from the ordinary stale surfaces and leaves every record fact and measure visible elsewhere.

Enforced by `metrics:a-suppression-is-local-and-never-replicates`, `metrics:a-suppression-hides-a-reminder-and-never-a-number`, and `metrics:a-suppression-is-announced-in-the-view-it-hides`.

## Consequences

- Good: one operator can silence a reminder without changing the team's record.
- Good: the ordinary stale view announces every hidden reminder.
- Bad: each machine manages its suppressions separately.

## Status

Accepted
