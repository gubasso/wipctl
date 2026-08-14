# wipctl fix

The rank repair: the only verb whose whole purpose is writing, restoring a legal order without inventing a preferred one.

## Usage

```text
wipctl fix [--slots] [<plan-dir>]
```

Flags: `--slots`, `--help` (usage on stdout, exit 0). Unknown option: exit 2 `fix: unknown option <x>`. At most one positional.

## Default mode — the repair

Rewrites lane order for `backlog.yml`, `todo.yml`, and `closed.yml`, under the writer's four guarantees ([../../validation/repair.md](../../validation/repair.md)):

- deterministic;
- identity-preserving on legal input — byte-identical, final newline included;
- idempotent;
- non-canonical — any other legal permutation is also a fixed point, so a legal human order is never disturbed.

Ordering applied: `backlog` and `todo` get a stable topological sort over same-lane `needs` edges; `todo` first partitions eligible entries above ineligible ones; `closed` sorts by close date, stable on current position, so same-day closes keep their order. Entry blocks MUST be relocated as their original lines — nothing is re-serialised, so comments, notes, and blank lines survive. An entry MUST NOT change lane.

The repair MUST refuse to write when any content check fails or the graph is cyclic, exiting with the check status. After writing it MUST re-run the check pass and return that status. It MUST hold the zone lock for the whole operation.

A hook MUST NOT invoke the repair: validation MUST stay falsifiable, and a gate that rewrites the thing it gates is not a gate.

## `--slots` — the report

This mode MUST write nothing. For `backlog` then `todo`, reports the legal candidate ids per position:

```text
$ wipctl fix --slots docs/plan
backlog.yml
  1: proxy-guide-53c9 release-readiness-77f0
  2: release-readiness-77f0
todo.yml
  1: rate-limit-the-search-endpoint-a7f3
  2: profile-composition-e01a
```

The report MUST refuse (exiting with the check status) when content checks fail — a slot report over a broken record would be advice about garbage.
