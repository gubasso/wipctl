# Stories and estimation

## The unit

The unit of work is a story: one vertical, demonstrable change, in one document. There are three types:

- `story`: a change a reader observes from their side. It must show the outcome under `Example`.
- `spike`: an investigation whose outcome is a decision or a measurement. It must show its outcome under `Example`. It exits through a recorded decision, a story revision, or a closed measurement.
- `chore`: upkeep with no reader-facing outcome. It needs no worked example.

What a recorded decision looks like is the host's business, not this method's. A project keeping decision records writes one. A project keeping none records the decision in whatever durable document owns the area. What the exit requires is that the finding left the spike. A spike whose outcome stays in the spike did not end.

Tasks live inside the story as a checklist and never become records of their own. A unit that can be any node of a tree is not a unit.

## Points count judgment

A point is one unit of irreducible human judgment: a decision a test cannot settle and a person must accept by eye. Points estimate reviewer attention, not time, effort, or diff size. When agents implement, reviewer attention is the scarce resource.

- `1`: confirmation only. Named tests settle acceptance, and at most one unchanged contract is involved.
- `2`: one judgment. An interface, a name, a message, or an existing contract needs human reasoning.
- `3`: two judgments, or one that is hard to reverse, such as a schema or a security control.
- `4` is not a value. Work above three splits along the judgments its acceptance section already names.

## Core and cuts

`Core` is never cut. `In scope` is the ordered negotiable remainder, cut from the bottom when the work runs long. Correctness, tests, review, and security never appear in the negotiable list, because they are never negotiable. Cutting planned remainder needs no revision entry. Changing the agreement does.

## Velocity is a budget, not a score

Velocity is the sum of points of entries closed as done inside one window, derived on every read. A cut entry counts nothing. A reshaped entry counts through its successor. The number budgets how much judgment one window can absorb. It measures nobody. The method deliberately does not specify planning against velocity, and it does not specify calibrating the scale between teams.
