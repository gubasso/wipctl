# Initiatives

An initiative is an end state no single epic delivers. It is one document under `initiatives/`, plus an `Initiative` section on the epic documents that serve it. A story is the unit of work. An epic is an end state no single story delivers. An initiative is an end state no single epic delivers. Membership points upward at every step and stops there.

## Five properties

The first four are the epic's four, unchanged in kind. The fifth is what this tier owes the reader.

1. One-way membership. Epics point at the initiative, and the document lists no members. A member list is a second store of the same fact, and it drifts the moment an epic changes.
2. Shared id space. Initiative ids, epic ids, entry ids, and question ids draw from one namespace, and one id names one thing.
3. Gated reference. An epic's `Initiative` section must name an existing document, or carry the literal `None`.
4. Never a dependency. An initiative never appears in a dependency list, never sequences anything, and never enters ranking. Membership is not sequencing, at any tier.
5. Bounded depth. The ladder is story, epic, initiative, and it stops there by rule. An initiative has no parent, and no artifact above it exists. A fourth tier is a new decision record, never a configuration value. Every tradition that let containers nest without a stated ceiling produced a filing system. A bounded ladder is a fixed traversal every consumer can write straight-line. That is what answers the nesting objection of ADR-an-epic-is-a-field-and-a-document.

## One path upward

Membership above the story flows only through the epic. A story names its epic, and the epic names its initiative. A story with no epic is outside every initiative. There is one path and no conflict rule to write. Every entry is counted at most once at every tier, with no deduplication step to get wrong.

The cost is honest. A story serving a strategic end state on its own must earn an epic to say so. An epic opened only to say so is the one-member anti-pattern below.

## Completion is derived

An initiative is closed out when every epic carrying its id is closed out, by the same rule one tier down. Closed out is not done, because a cut member closes without delivering. That is why the document carries `Done when`. The rollup shows delivered against promised. Nothing derives a retirement decision. That is a review question at every tier, asked alongside whether the charter is still true.

## Reported, never executed

The initiative verb decomposes an initiative into one row per member epic. It never resolves entries into eligible and blocked groups. A session works one end state at a time. An eligible set spanning four epics invites a session to take work from an end state nobody is currently pursuing. The entry-level execution plan is the epic verb's answer and has exactly one home, per ADR-an-initiative-is-reported-never-executed. The initiative verb's output is the list to run it against.

## When to open one

Open an initiative when several epics turn out to serve one end state that is invisible from any of them. This is the same test as an epic, one tier up. Two epics rarely qualify. The anti-patterns are the siblings of the epic page's:

- an initiative with one epic, which is an epic wearing a container
- an initiative per quarter or per release, which is a calendar and not an end state

Most projects will never need the tier, and an absent `initiatives/` directory is exactly that statement.
