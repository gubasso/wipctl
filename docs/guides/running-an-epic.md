# Running an epic

Declare, drive, and retire a goal larger than one story. [explanation/epics.md](../explanation/epics.md) says why an epic is shaped this way. This page is the sequence.

## 1 — Open one

Inputs: none.

1. Apply the test first. Open an epic when a split produced pieces whose shared end state is invisible from any one of them. Two stories rarely qualify. Do not open any of these:
   - a renamed story with one member
   - a container before the split exists
   - an epic carrying acceptance criteria
   - an epic per release or per quarter
2. Mint an id from the shared namespace, which is the title's slug. Write `epics/<id>.md` in the epic shape. The document lists no members and carries no status.
3. Fill the `Initiative` section first. It carries the id of the initiative this epic serves, or the literal `None`. The section is never left empty, and a named id must resolve to a document under `initiatives/`. [running-an-initiative.md](./running-an-initiative.md) covers that tier.
4. Write the end state, not a summary. Make `Example` a simulation: what is true today, against what is true at the end. Make `Done when` observable, never "every member closed". Where the host keeps rule ids, cite the ids whose verification commands must pass.
5. Put the id in each serving entry's `epic` field.

Outputs of this phase:

```text
<EPIC_ID> — the epic's id; the filename stem under epics/, carried by each member's epic field
```

## 2 — Execute one

Inputs: `<EPIC_ID>` (§1).

1. Ask for the plan:

   ```text
   $ wipctl epic <EPIC_ID>
   ```

2. Take one entry from `eligible now`, work it, and close it:

   ```text
   $ wipctl move <id> --to closed --outcome done
   ```

3. Ask again. Asking again is not optional, because a close changes eligibility and the answer is derived on every call. A session that needs the plan as a file points a consumer at `wipctl epic <EPIC_ID> --write`. That cache is never authoritative and never read back.
4. Do not use the epic to sequence. The record's key chain orders entries from eligibility, same-lane dependencies, points, and the residual id; epic membership contributes nothing. A prerequisite from outside the epic appears in the plan, marked by what it serves. It is part of the work and not an intrusion.

Outputs: none.

## 3 — Retire one

Inputs: `<EPIC_ID>` (§1).

1. A fully closed epic needs no action.
2. An epic nobody is pursuing is retired by a person, in review, alongside whether the charter is still true. Nothing derives retirement from the numbers. Delivered against promised is visible, and a threshold nobody agreed to is not a decision.

Outputs: none.
