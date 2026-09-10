# Lanes and ranking

## The record's shape

The lane files are the record. The file is the lane, so there is no `status` field. File sequence records membership alone, and the key chain computes rank from record facts on every read. A lane move is a reviewable membership diff, while a change to dependencies, class, points, or close date changes the computed order without relocating an entry by hand.

The lane entry carries what a scanner needs: type, points, class of service, summary, dependencies, epic, labels, and outcome. The story document carries what a session needs: goal, example, scope, references, acceptance, and tasks. The zone holds intent, so it is allowed to become false as work moves. That is why it must hold no durable fact. Durable facts live in the host project's documents, as [host-integration.md](./host-integration.md) states.

## Eligibility

An entry is eligible when every id in its `needs` is closed and no open question or blocking watch blocks it. Blocked is always derived from those three edge kinds, never a lane and never a field. As a result, a view and a gate can never disagree about it.

An id that carries an alias prefix is read in the peer that alias names. It gates this entry exactly as a local id does. [plans-and-peers.md](./plans-and-peers.md) says why the two forms are one edge.

## The key chain

The record uses one order computation. Close date sorts first, ascending, where an entry carries one. The remaining comparisons run in this order:

1. eligible before ineligible
2. topological over same-lane dependencies
3. class of service: expedite, standard, intangible
4. points ascending
5. full id, compared lexically as one opaque string

The first two comparisons apply the graph constraints by construction. Class follows, with an absent class at the `standard` position. It states whether the cost of delay is immediate, ordinary, or late. Smaller work follows, which shortens the average wait, and the three-point cap bounds what larger work can wait behind. The full id resolves only otherwise indistinguishable entries and makes the order total. The closed lane uses the same procedure, with close date equal everywhere else, so the record needs no separate chain per lane.

Class affects ordering alone. It creates no dependency, changes no eligibility result, and enters no measure. More than one open expedite entry is legal and produces a warning at exit 0, because the person who stated the classes decides whether to change them.

The two work lanes admit only entries whose dependencies are closed and whose questions are answered. The chain still computes their order, and a gap left by a reopening is not a defect because file sequence carries no rank.

## Computed order, human priority

The tool computes rank; it does not author priority. Class is the one field a person writes for ordering alone. Every comparison above the residual id reads a fact a person wrote or a condition derived from those facts. The residual id chooses only when those facts express no preference. There is no stored order to validate, repair, or offer as a set of legal positions.
