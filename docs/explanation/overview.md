# The method in one page

Work is agreed as stories: one vertical, demonstrable change each, in one document, estimated in points that count human judgment rather than effort. Stories live in five lanes — `backlog`, `todo`, `doing`, `review`, `closed` — where the file is the lane and the position is the ranking. The machine owns legality: dependencies closed, questions answered, order consistent with the graph. People own priority: what ranks above what, within legal bounds.

The record is plain files in a plain git repository of its own — one live plan per project, shared by every worktree and clone on the machine, hosted wherever its operator chooses — and it holds no derived state: no velocity, no blocked flags, no progress numbers. Every view is computed on every read, so the record can never disagree with itself about a number. The host project keeps one committed file naming its identity, and every checkout resolves the same plan through it.

A lane change is a verb, never a file edit: `wipctl move` relocates the entry, appends a transition event to the entry's journal, repairs the ranking, and commits the mutation to the plan trunk, atomically. The journal is what makes age, dwell, and rework record facts instead of repository archaeology. `closed` is a lane, not a terminus — reopening is a recorded transition too, and even deletion leaves a journal tombstone that burns the id.

Epics name end states no single story delivers. Membership is a field on entries, pointing one way; completion is derived; an epic never sequences anything. An epic may in turn name the initiative it serves — an end state no single epic delivers — and the ladder stops there, at three tiers, by rule. `wipctl epic <id>` resolves one epic into its execution plan: a session takes any eligible entry, works it, closes it, and asks again; `wipctl start` takes the head of `todo` atomically, so concurrent sessions fan out onto different entries instead of racing for one.

New work is captured without waiting: `wipctl new` slugs the title into an id, checks it against the live record and every tombstone, writes the story document and a pending fragment, and touches no lane file. A taken or burned id is refused on the spot with the rephrase suggested — nothing random is minted, so an id is its title's slug, carrying a qualifying postfix only where that slug was taken, and a filename is guessable from a title in the ordinary case. `wipctl land` later reconciles everything into one ranked record, in a derived order every machine agrees on, reporting every drift instead of resolving any conflict silently; `wipctl sync` replicates the trunk between machines the same way, reporting any genuine conflict for a person to decide.

The gate holds it all together: `wipctl validate` proves the record coherent — shapes by schema, cross-file facts by the checker, lanes and journal in agreement — and a skipped check is always named. The gate never repairs; repair is a person's explicit act.

The pages alongside this one unpack each piece:

- [stories-and-estimation.md](./stories-and-estimation.md) — the unit and the scale.
- [lanes-and-ranking.md](./lanes-and-ranking.md) — eligibility and the two ranking rules.
- [epics.md](./epics.md) — what an epic is and is not.
- [initiatives.md](./initiatives.md) — the tier above, and why the ladder stops.
- [transitions.md](./transitions.md) — why lane changes are recorded events.
- [concurrency.md](./concurrency.md) — one record, many agents, no queue.
- [host-integration.md](./host-integration.md) — the contract with the adopting project.
