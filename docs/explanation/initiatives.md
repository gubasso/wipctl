# Initiatives

An initiative is an end state no single epic delivers: one document under `initiatives/`, plus an `Initiative` section on the epic documents that serve it. A story is the unit of work; an epic is an end state no single story delivers; an initiative is an end state no single epic delivers. Membership points upward at every step and stops there.

## Five properties

The first four are the epic's four, unchanged in kind; the fifth is what this tier owes the reader.

1. One-way membership. Epics point at the initiative; the document lists no members. A member list would be a second store of the same fact, drifting the moment an epic changes.
2. Shared id space. Initiative ids, epic ids, entry ids, and question ids draw from one namespace; one id names one thing.
3. Gated reference. An epic's `Initiative` section must name an existing document, or carry the literal `None`.
4. Never a dependency. An initiative never appears in `needs`, never sequences anything, and never enters ranking. Membership is not sequencing, at any tier.
5. Bounded depth. The ladder is story, epic, initiative — and stops, by rule. An initiative has no parent, and no artifact above it exists; a fourth tier is a new decision record, never a configuration value. Every tradition that let containers nest without a stated ceiling produced a filing system; a bounded ladder is a fixed traversal every consumer can write straight-line, which is what answers the nesting objection of ADR-0004.

## One path upward

Membership above the story flows only through the epic. A story names its epic; the epic names its initiative; a story with no epic is outside every initiative. There is one path and no conflict rule to write — and every entry is counted at most once at every tier, with no deduplication step to get wrong. The cost is honest: a story serving a strategic end state on its own must earn an epic to say so, and an epic opened only to say so is the one-member anti-pattern below.

## Completion is derived

An initiative is closed out when every epic carrying its id is closed out, by the same rule one tier down — and closed out is not done, which is why the document carries `Done when`: a cut member closes without delivering. The rollup shows delivered against promised; nothing derives a retirement decision, which is a review question at every tier, asked alongside "is the charter still true".

## Reported, never executed

`wipctl initiative <id>` decomposes an initiative into one row per member epic; it never resolves entries into eligible and blocked groups. A session works one end state at a time, and an eligible set spanning four epics would invite a session to take work from an end state nobody is currently pursuing. The entry-level execution plan is `wipctl epic <id>`'s answer and has exactly one home (ADR-0056); the initiative verb's output is the list to run it against.

## When to open one

Open an initiative when several epics turn out to serve one end state that is invisible from any of them — the same test as an epic, one tier up. Two epics rarely qualify. Anti-patterns, the siblings of the epic page's: an initiative with one epic, which is an epic wearing a container; and an initiative per quarter or per release, which is a calendar, not an end state. Most projects will never need the tier, and an absent `initiatives/` directory is exactly that statement.
