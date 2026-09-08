# Lanes and ranking

## The record's shape

The lane files are the record. The file is the lane, so there is no `status` field. The position is the ranking, so there is no `priority` field. This is what makes a lane move a reviewable diff and a rank change a one-line relocation.

The lane entry carries what a scanner needs: type, points, summary, dependencies, epic, labels, and outcome. The story document carries what a session needs: goal, example, scope, references, acceptance, and tasks. The zone holds intent, so it is allowed to become false as work moves. That is why it must hold no durable fact. Durable facts live in the host project's documents, as [host-integration.md](./host-integration.md) states.

## Eligibility

An entry is eligible when every id in its `needs` is closed and no open question blocks it. Blocked is always derived from those two edge kinds, never a lane and never a field. As a result, a view and a gate can never disagree about it.

## The two ranking rules

- R1. In `todo.yml`, no ineligible entry sits above an eligible one. Whenever the lane holds a startable entry, the head is one, so the preview verb is a head read and not a search.
- R2. In `backlog.yml` and `todo.yml`, no entry sits above an entry it needs. Only same-lane edges count.

The two work lanes need no ranking rules. Their entry gate already requires closed dependencies and no blocking question. The closed lane is ordered by close date ascending, stable within a day. An out-of-order date is a warning the repair resolves, and a gap left by a reopening is not a defect.

## Machine legality, human priority

The checker decides whether an order is legal. It never decides which legal order is right. The repair restores legality with the smallest disturbance and treats every legal permutation as a fixed point. That split is why the repair is non-canonical and why no hook invokes it. It is also why the slot report names the legal candidates per position instead of choosing one.
