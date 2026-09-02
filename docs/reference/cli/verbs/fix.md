# wipctl fix

The rank repair: the only verb whose whole purpose is writing, restoring a legal order without inventing a preferred one.

## Usage

```text
wipctl fix [--slots]
```

Flags: `--slots`, `--help` (usage on stdout, exit 0). Unknown option: exit 2 `fix: unknown option <x>`. No positional.

## Default mode — the repair

Rewrites lane order for `backlog.yml`, `todo.yml`, and `closed.yml`, under the writer's four guarantees ([../../validation/repair.md](../../validation/repair.md)):

- deterministic;
- identity-preserving on legal input — byte-identical, final newline included;
- idempotent;
- non-canonical — any other legal permutation is also a fixed point, so a legal human order is never disturbed.

Ordering applied: `backlog` and `todo` get a stable topological sort over same-lane `needs` edges; `todo` first partitions eligible entries above ineligible ones; `closed` sorts by close date, stable on current position, so same-day closes keep their order. Entry blocks MUST be relocated as their original lines — nothing is re-serialised, so comments, notes, and blank lines survive. An entry MUST NOT change lane.

The repair MUST refuse to write when any content check fails or the graph is cyclic, exiting with the check status, naming each failure and the edit that clears it. After writing it MUST re-run the check pass, return that status, and commit `plan: fix ranking`. It MUST hold the lock for the whole transaction, the commit included.

A hook MUST NOT invoke the repair: validation MUST stay falsifiable, and a gate that rewrites the thing it gates is not a gate.

## `--slots` — the report

This mode MUST write nothing. For `backlog` then `todo`, reports the legal candidate ids per position:

```text
$ wipctl fix --slots
backlog.yml
  1: proxy-guide release-readiness
  2: release-readiness
todo.yml
  1: rate-limit-the-search-endpoint
  2: profile-composition
```

The report MUST refuse (exiting with the check status) when content checks fail — a slot report over a broken record would be advice about garbage.
