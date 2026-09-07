# Epics

An epic is an end state no single story delivers. It is one document under `epics/`, plus an `epic` field on the entries that serve it.

## Four properties

1. One-way membership. Entries point at the epic, and the document lists no members. A member list is a second store of the same fact, and it drifts the moment an entry changes.
2. Shared id space. Epic ids and entry ids draw from one namespace, and one id names one thing.
3. Gated reference. An entry's `epic` field must name an existing document. A `tag`, by contrast, is gated by nothing.
4. Never a dependency, and one optional parent. An epic never appears in a dependency list and never sequences members. It can name the one initiative it serves, in its own document and nothing deeper, as [initiatives.md](./initiatives.md) states. The ladder stops there, because unbounded nesting is how a plan becomes a filing system.

## Completion is derived

An epic is closed out when every entry carrying its id is closed. Closed out is not done, because a cut member closes without delivering. That is why the document carries `Done when`, which states an observable end state and never "every member closed".

Where the host keeps rule ids, the sharpest end state is a set of rule ids whose verification commands all pass. The epic then never restates a specification's rules, it cites them. Doneness is decided by running the commands rather than by reading prose.

The rollup shows delivered against promised, and a cut member stays in the denominator. Nothing derives a retirement decision. That is a review question, asked alongside whether the charter is still true. An empty epic, meaning a document no entry joined, is a legal first draft.

## When to open one

Open an epic when a split produced pieces whose shared end state is invisible from any one of them. Two stories rarely qualify. There are four anti-patterns:

- a renamed story with one member
- a container opened before the split
- an epic that carries acceptance criteria
- an epic per release or per quarter

## Where the approach goes

An epic states an end state, not a design. The approach that reaches it covers the seams chosen, the reason for the order, and the rejected alternatives. It belongs in `Core` and `Example` when it is short. When it is long it belongs in a host document the members name under `Reads`, as [host-integration.md](./host-integration.md) states. An epic has no section of its own for it. A design an epic holds is a state nobody transfers forward when the strategy changes.

## Executing one

The epic verb resolves the epic into eligible and blocked groups in graph order, with outside prerequisites included and marked. The loop is simple: take one entry from the eligible group, work it, close it, and ask again. Asking again is not optional, because a close changes eligibility and can change the ranking. One entry serves one end state. A story that seems to serve two was split along the wrong seam.
