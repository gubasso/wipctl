# Stories and estimation

## The unit

The unit of work is a story: one vertical, demonstrable change, in one document. Three types:

- `story` — a change a reader can observe from their side; must show the outcome under `Example`.
- `spike` — an investigation whose outcome is a decision or measurement; must show its outcome under `Example`, and exits through a decision record, a story revision, or a closed measurement.
- `chore` — upkeep with no reader-facing outcome; needs no worked example.

Tasks live inside the story as a checklist and never become records of their own: a unit that may be any node of a tree is not a unit.

## Points count judgment

A point is one unit of irreducible human judgment — a decision a test cannot settle and a person must accept by eye. Points estimate reviewer attention, not time, effort, or diff size, because when agents implement, reviewer attention is the scarce resource.

- `1` — confirmation only. Named tests settle acceptance; at most one unchanged contract is involved.
- `2` — one judgment. An interface, a name, a message, or an existing contract needs human reasoning.
- `3` — two judgments, or one that is hard to reverse — a schema, a security control.
- `4` is not a value. Work above three splits along the judgments its acceptance section already names.

## Core and cuts

`Core` is never cut. `In scope` is the ordered negotiable remainder, cut from the bottom when the work runs long. Correctness, tests, review, and security never appear in the negotiable list, because they are never negotiable. Cutting planned remainder needs no `Revisions` entry; changing the agreement does.

## Velocity is a budget, not a score

Velocity is the sum of points of entries closed `done` inside an iteration window, derived on every read. `cut` counts nothing; `reshaped` counts through its successor. The number budgets how much judgment one iteration can absorb; it measures nobody. The method deliberately does not specify planning against velocity or calibrating the scale between teams.
