# Lanes and ranking

## The record's shape

The lane files are the record. The file is the lane — no `status` field — and the position is the
ranking — no `priority` field. This is what makes a lane move a reviewable diff and a rank change
a one-line relocation.

The lane entry carries what a scanner needs (type, points, summary, dependencies, epic, labels,
outcome); the story document carries what a session needs (goal, example, scope, sources,
acceptance, tasks). The zone holds intent, so it is allowed to become false as work moves — which
is why it must hold no durable fact. Durable facts live in the host project's documents; see
[host-integration.md](./host-integration.md).

## Eligibility

An entry is eligible when every id in its `needs` is in `closed.yml` and no open question blocks
it. Blocked is always derived from those two edge kinds — never a lane, never a field — so a view
and a gate can never disagree about it.

## The two ranking rules

- R1 — in `todo.yml`, no ineligible entry sits above an eligible one. Consequence: whenever the
  lane holds a startable entry, the head is one, and `next` is a head read, not a search.
- R2 — in `backlog.yml` and `todo.yml`, no entry sits above an entry it needs. Only same-lane
  edges count.

`doing` and `review` need no ranking rules: their entry gate already requires closed dependencies
and no blocking question. `closed.yml` is ordered by close date ascending, stable within a day; an
out-of-order date is a warning the repair resolves, and a gap left by a reopening is not a
defect.

## Machine legality, human priority

The checker decides whether an order is legal; it never decides which legal order is right. The
repair (`wipctl fix`) restores legality with the smallest disturbance and treats every legal
permutation as a fixed point. That split is why the repair is non-canonical, why no hook invokes
it, and why `fix --slots` reports the legal candidates per position instead of choosing one.
