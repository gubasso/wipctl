# Lanes and ranking

## The record's shape

The lane files are the record. The file is the lane, so there is no `status` field. File sequence records membership alone. The ranking procedure computes rank from record facts on every read. A lane move is a reviewable membership diff. A change to dependencies, delay cost, points, or close date changes the computed order without relocating an entry by hand.

The lane entry carries what a scanner needs: type, points, delay cost, summary, dependencies, epic, labels, and outcome. The story document carries what a session needs: goal, example, scope, references, acceptance, and tasks. The zone holds intent, so it can become false as work moves. It holds no durable fact. Durable facts live in the host project's documents, as [host-integration.md](./host-integration.md) states.

## Eligibility

An entry is eligible when every id in its `needs` is closed and no open question or blocking watch blocks it. Blocked is always derived from those three edge kinds, never a lane and never a field. A view and a gate therefore cannot disagree about it.

An id that carries an alias prefix is read in the peer that alias names. It gates this entry exactly as a local id does. [plans-and-peers.md](./plans-and-peers.md) explains why the two forms are one edge.

## The ranking procedure

The record uses one ranking procedure:

1. Close date places closed entries from oldest to newest.
2. Eligibility places startable work first.
3. Dependency order places each same-lane prerequisite before the entry that needs it.
4. The plan's declared preferences order entries that satisfy the same constraints.
5. The full id resolves any remaining tie.

Close date, eligibility, dependency order, and the final tie-break are fixed product behavior. They are not project preferences, so the configuration does not expose them. A project can choose only whether delay cost or points applies first, omit either preference, or state that it has none.

The scaffold writes:

```toml
[ranking]
preferences = ["delay-cost", "points-ascending"]
```

The declaration lives in `.wipctl/plan.toml` because a commit and `sync` replicate it to every worker. An operator changes the preferences by editing that file and committing it. The plan repository's hooks validate the change before it lands.

Delay cost states when waiting harms an entry. `immediate` sorts before ordinary work. `deferred` sorts after ordinary work. Ordinary work omits the field. Delay cost creates no dependency, changes no eligibility result, and enters no measure.

The two work lanes admit only entries whose dependencies are closed and whose questions are answered. The procedure still computes their order. A gap left by a reopening is not a defect because file sequence carries no rank.

## Reading the order back

A computed order a reader cannot explain is an order they cannot challenge. `wipctl board` marks immediate delay cost with `^`, ordinary work with no glyph, and deferred delay cost with `.`. The marks survive a pipe and a colour-free terminal.

`wipctl board --why` and `wipctl next --why` name the one constraint, preference, or tie-break that placed each row against the row above it. They show one reason, not the whole procedure or a score.

Where the final ID tie-break decides a row, the board says so without a flag. The tool chose where the record expressed no preference, so the output must expose that fact.

The loop is read, edit the record, read again. Neither flag writes a byte, takes a lock, or exits non-zero.

## Computed order, human preference

The tool computes rank. It does not author priority. Fixed constraints keep the plan valid. Declared preferences apply facts a person wrote. The final ID tie-break chooses only when those facts express no preference. There is no stored order to validate, repair, or offer as a set of legal positions.
