# ADR-0053: Taking work is a verb of its own; next stays read-only

## Context and Problem Statement

With several agents pulling from one lane, every agent that runs `next` sees the same head, so all but one lose the race at `move` and start over. Read-then-claim needs an atomic form — without costing `next` its no-lock, never-gates guarantee.

## Considered Options

- A new verb, `start`: read and take atomically inside one lock holding
- A `--take` flag on `next` — a flag that changes a pure read into a write is two verbs wearing one name, and it would put a lock inside the one verb promised never to take one
- Retry loops over `move` — works, but every lost race is a wasted preflight and a confused agent; the collision is designed out instead

## Decision Outcome

Chosen option: `start`. Inside one lock holding it reads at current HEAD, chooses the head of `todo`, runs the full `move`-to-`doing` preflight, writes, repairs ranking, commits, and releases — the id is chosen inside the lock, so two agents starting at the same instant take two different entries. It prints the line `next` would have printed for the entry it took. Nothing startable is exit 1 naming why, so a fan-out loop terminates instead of spinning. The name reads as one idea with its twin: `next` tells you, `start` takes it. `claim` was rejected because the record already uses that word for position, rank, and membership claims. `next` is unchanged, and `move` keeps exit 2 on already-in-destination as the lost-race signal for takes by id.

## Consequences

- Good: agents fan out onto different entries without negotiating, and the pairing rule of ADR-0015 covers the twins.
- Bad: one more writer, and one more verb page one letter away from its twin.

## Status

Accepted
