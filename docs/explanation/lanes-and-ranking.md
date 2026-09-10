# Lanes and ranking

## The record's shape

The lane files are the record. The file is the lane, so there is no `status` field. File sequence records membership alone, and the key chain computes rank from record facts on every read. A lane move is a reviewable membership diff, while a change to dependencies, class, points, or close date changes the computed order without relocating an entry by hand.

The lane entry carries what a scanner needs: type, points, class of service, summary, dependencies, epic, labels, and outcome. The story document carries what a session needs: goal, example, scope, references, acceptance, and tasks. The zone holds intent, so it is allowed to become false as work moves. That is why it must hold no durable fact. Durable facts live in the host project's documents, as [host-integration.md](./host-integration.md) states.

## Eligibility

An entry is eligible when every id in its `needs` is closed and no open question or blocking watch blocks it. Blocked is always derived from those three edge kinds, never a lane and never a field. As a result, a view and a gate can never disagree about it.

An id that carries an alias prefix is read in the peer that alias names. It gates this entry exactly as a local id does. [plans-and-peers.md](./plans-and-peers.md) says why the two forms are one edge.

## The key chain

The record uses one order computation. Close date sorts first, ascending, where an entry carries one. The plan configuration's required `[ranking]` table declares the remaining comparisons as one ordered list. The scaffold writes this chain:

1. eligible before ineligible
2. topological over same-lane dependencies
3. class of service: expedite, standard, intangible
4. points ascending
5. full id, compared lexically as one opaque string

Every valid chain starts with eligibility and same-lane dependencies because they are constraints rather than preferences. A chain that put class first could put blocked work at its head. The project chooses which stated keys follow and their order, and the full id always ends the list. The residual id resolves only otherwise indistinguishable entries and makes the order total. The closed lane uses the same procedure, with close date equal everywhere else, so the record needs no separate chain per lane.

The declaration lives in `.wipctl/plan.toml` because a commit and `sync` replicate it to every worker. The host repository would make it per checkout, the state directory would make it per machine, and the cache is disposable. An order that differs per machine is not an order.

An operator changes the chain by editing the plan configuration and committing it. The plan repository's hooks validate the change before it lands. No verb writes the chain because a configuration verb would need a grammar for every key, and no warning duplicates the hook's refusal.

Class affects ordering alone. It creates no dependency, changes no eligibility result, and enters no measure. More than one open expedite entry is legal and produces a warning at exit 0, because the person who stated the classes decides whether to change them.

The two work lanes admit only entries whose dependencies are closed and whose questions are answered. The chain still computes their order, and a gap left by a reopening is not a defect because file sequence carries no rank.

## Reading the order back

A computed order a reader cannot explain is an order they cannot argue with. `wipctl board` renders each lane in chain order and marks each row's class with a glyph: `^` for expedite, nothing for standard, and `.` for intangible. Two ASCII characters survive a pipe and a colour-free terminal, which is why they are the mark rather than a colour.

`wipctl board --why` and `wipctl next --why` add one column naming the key that decided each row against the row above it. One key, never the whole chain and never a score, because the reader wants to know why this row sits here rather than one line up.

Where the residual id decided a row, the board says so with no flag given. That mark is not optional, because a mark only a flag reveals is a mark nobody sees, and this is the one place the tool chose where a person did not.

The loop is read, edit the record, read again. Neither flag writes a byte, takes a lock, or exits non-zero.

## Computed order, human priority

The tool computes rank; it does not author priority. Class is the one field a person writes for ordering alone. Every comparison above the residual id reads a fact a person wrote or a condition derived from those facts. The residual id chooses only when those facts express no preference. There is no stored order to validate, repair, or offer as a set of legal positions.
