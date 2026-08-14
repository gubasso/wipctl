# The method in one page

Work is agreed as stories: one vertical, demonstrable change each, in one document, estimated in points that count human judgment rather than effort. Stories live in five lanes — `backlog`, `todo`, `doing`, `review`, `closed` — where the file is the lane and the position is the ranking. The machine owns legality: dependencies closed, questions answered, order consistent with the graph. People own priority: what ranks above what, within legal bounds.

The record is plain files in version control, and it holds no derived state — no velocity, no blocked flags, no progress numbers. Every view is computed on every read, so the record can never disagree with itself about a number.

A lane change is a verb, never a file edit: `wipctl move` relocates the entry, appends a transition event to the entry's journal, and repairs the ranking, atomically. The journal is what makes age, dwell, and rework record facts instead of repository archaeology. `closed` is a lane, not a terminus — reopening is a recorded transition too, and even deletion leaves a journal tombstone that burns the id.

Epics name end states no single story delivers. Membership is a field on entries, pointing one way; completion is derived; an epic never sequences anything. `wipctl epic <id>` resolves one epic into its execution plan: a session takes any eligible entry, works it, closes it, and asks again.

New work is captured without waiting: `wipctl new` mints an id without asking anybody what the next one is (the title's slug plus four random hex; the rare collision surfaces loudly — a version-control conflict between clones, or the gate naming both claimants in one record — and one side is re-minted), writes the story document and a pending fragment, and touches no lane file. Two sessions capture in parallel; `wipctl land` later reconciles everything into one ranked record, in an order every clone derives identically, reporting every drift instead of resolving any conflict silently.

The gate holds it all together: `wipctl validate` proves the record coherent — shapes by schema, cross-file facts by the checker, lanes and journal in agreement — and a skipped check is always named. The gate never repairs; repair is a person's explicit act.

The pages alongside this one unpack each piece:

- [stories-and-estimation.md](./stories-and-estimation.md) — the unit and the scale.
- [lanes-and-ranking.md](./lanes-and-ranking.md) — eligibility and the two ranking rules.
- [epics.md](./epics.md) — what an epic is and is not.
- [transitions.md](./transitions.md) — why lane changes are recorded events.
- [concurrent-capture.md](./concurrent-capture.md) — capture and drain.
- [host-integration.md](./host-integration.md) — the contract with the adopting project.
