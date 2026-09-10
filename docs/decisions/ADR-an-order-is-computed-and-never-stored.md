# An order is computed and never stored

## Context and Problem Statement

Lane-file position stores a rank even when nobody chose one. A stored order can drift from eligibility and dependencies, so the record needs checks and repair rules only to restore it.

The closed lane and fragment drain already derive deterministic orders from record facts. The same approach can make every lane order total and reproducible.

## Considered Options

- One record-wide ordering procedure with a total key chain — chosen.
- Lane-file position stores rank — rejected: position makes an accidental placement a priority claim and creates an order that can become illegal.
- One chain per lane — rejected: four extra declarations buy nothing when only closed entries carry a distinct key.
- A numeric priority field — rejected: equal values need another tie-break and store another ordering claim.

## Decision Outcome

Chosen option: one record-wide ordering procedure with a total key chain. The procedure compares close dates first where present, then eligibility, same-lane dependencies, points ascending, and the full id lexically. A missing close date compares equal on that key.

The lane file stores membership alone. The tool computes order on every read from current record facts. The residual id comparison makes the chain total and treats the id as one opaque string.

Enforced by `ranking:an-order-is-computed-and-never-stored`, `ranking:the-key-chain-is-total`, `ranking:a-key-reads-the-record-alone`, and `ranking:the-residual-key-is-the-full-id`.

## Consequences

- Good: stored order cannot drift, no ranking repair remains, and two clones at one commit derive one order.
- Good: changing lane-file sequence changes no rendered rank.
- Bad: points now affect scheduling, so the three-point cap also bounds the wait behind smaller work.

## Status

Accepted

Supersedes the ordering half of [ADR-the-plan-record-is-five-lane-files.md](./ADR-the-plan-record-is-five-lane-files.md), whose lane-membership choice survives. Supersedes [ADR-a-position-names-only-a-landed-entry.md](./ADR-a-position-names-only-a-landed-entry.md).
