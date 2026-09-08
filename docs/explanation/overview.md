# The method in one page

Work is agreed as stories. A story is one vertical, demonstrable change, in one document, estimated in points that count human judgment rather than effort. Stories live in five lanes: `backlog`, `todo`, `doing`, `review`, and `closed`. The file is the lane, and the position is the ranking. The machine owns legality: dependencies closed, questions answered, and an order consistent with the graph. People own priority: what ranks above what, within legal bounds.

The record is plain files in a plain git repository of its own. There is one live plan per project, shared by every worktree and clone on the machine, hosted wherever its operator chooses. It holds no derived state: no velocity, no blocked flags, and no progress numbers. Every view is computed on every read, so the record can never disagree with itself about a number. The host project keeps one committed file naming its identity, and every checkout resolves the same plan through it.

A lane change is a verb, never a file edit. The move verb relocates the entry and appends a transition event to its journal. It then repairs the ranking and commits the mutation to the plan trunk, all in one transaction. The journal is what makes age, dwell, and rework record facts instead of repository archaeology. The closed lane is a lane and not a terminus. Reopening is a recorded transition too, and even deletion leaves a journal tombstone that burns the id.

Epics name end states no single story delivers. Membership is a field on entries, pointing one way. Completion is derived, and an epic never sequences anything. An epic can in turn name the initiative it serves, which is an end state no single epic delivers. The ladder stops there, at three tiers, by rule.

The epic verb resolves one epic into its execution plan. A session takes any eligible entry, works it, closes it, and asks again. The take verb takes the head of the scheduled lane in one transaction. Concurrent sessions therefore fan out onto different entries instead of racing for one.

New work is captured without waiting. The capture verb slugs the title into an id and checks it against the live record and every tombstone. It then writes the story document and a pending fragment, and touches no lane file. A taken or burned id is refused on the spot, with the rephrase suggested. Nothing random is minted, so an id is its title's slug, carrying a qualifying postfix only where that slug was taken. A filename is therefore guessable from a title in the ordinary case.

The drain later reconciles everything into one ranked record, in a derived order every machine agrees on. It reports every drift instead of resolving any conflict silently. The replication verb moves the trunk between machines the same way, and reports any genuine conflict for a person to decide.

The gate holds it all together. Validation proves the record coherent: shapes by schema, cross-file facts by the checker, and lanes and journal in agreement. A skipped check is always named. The gate never repairs, because repair is a person's explicit act.

The pages alongside this one unpack each piece:

- [stories-and-estimation.md](./stories-and-estimation.md): the unit and the scale.
- [lanes-and-ranking.md](./lanes-and-ranking.md): eligibility and the two ranking rules.
- [epics.md](./epics.md): what an epic is and is not.
- [initiatives.md](./initiatives.md): the tier above, and why the ladder stops.
- [transitions.md](./transitions.md): why lane changes are recorded events.
- [concurrency.md](./concurrency.md): one record, many agents, no queue.
- [replication.md](./replication.md): the same record on a second machine.
- [host-integration.md](./host-integration.md): the contract with the adopting project.
