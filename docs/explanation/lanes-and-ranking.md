# Lanes and ranking

## The record's shape

The lane files are the record. The file is the lane, so there is no `status` field. File sequence records membership alone. The ranking procedure computes rank from record facts on every read. A lane move is a reviewable membership diff. A change to dependencies, delay cost, points, or close date changes the computed order without relocating an entry by hand.

The lane entry carries what a scanner needs: type, points, delay cost, summary, dependencies, epic, labels, and outcome. The story document carries what a session needs: goal, example, scope, references, acceptance, and tasks. The zone holds intent, so it can become false as work moves. It holds no durable fact. Durable facts live in the host project's documents, as [host-integration.md](./host-integration.md) states.

## Eligibility

An entry is eligible when every id in its `needs` is closed and no open question or blocking watch blocks it. Blocked is always derived from those three edge kinds, never a lane and never a field. A view and a gate therefore cannot disagree about it.

An id that carries an alias prefix is read in the peer that alias names. It gates this entry exactly as a local id does. [plans-and-peers.md](./plans-and-peers.md) explains why the two forms are one edge.

## The ranking procedure

The record uses one ranking procedure. It applies product constraints, then project preferences, then one internal tie-break. A constraint protects a valid plan. A preference expresses which valid entry a project wants first. A tie-break makes the result deterministic without expressing preference. The procedure applies these stages in order:

1. Close date places closed entries from oldest to newest.
2. Eligibility places startable work first.
3. Dependency order places each same-lane prerequisite before the entry that needs it.
4. The plan's declared preferences order entries that satisfy the same constraints.
5. The full id resolves any remaining tie.

Close date, eligibility, dependency order, and the final tie-break are fixed product behavior. They are not project preferences, so the configuration does not expose them. A project can choose only whether delay cost or points applies first, omit either preference, or state that it has none. One record-wide procedure avoids four more chains to configure. The closed lane is the only lane with a distinct key, and the close-date comparison is equal everywhere else.

Smaller work first shortens the average wait. The three-point cap bounds what a larger entry can wait behind because larger work splits along its judgments.

The scaffold writes both preferences, delay cost first, as [plan configuration](../reference/plan-configuration.md#ranking-preferences) shows.

The declaration lives in `.wipctl/plan.toml` because a commit and `sync` replicate it to every worker. Every worker then derives one order from one declaration. The host repository is per project checkout, the state directory is machine-local, and the cache is disposable, so none can carry a plan's order. An operator changes the preferences by editing that file and committing it. The plan repository's hooks validate the change before it lands.

The two work lanes admit only entries whose dependencies are closed and whose questions are answered. The procedure still computes their order. A gap left by a reopening is not a defect because file sequence carries no rank.

## Delay cost

Delay cost states when waiting harms an entry. `immediate` means waiting causes material harm now. `deferred` means the harm appears later, remains uncertain, or is negligible now. An absent field means the ordinary cost expected of planned work.

The field records only an exception to ordinary work. The reader places absence between `immediate` and `deferred` without adding a value to the entry. A comparison position for absence is not a read-time default.

Delay cost is the only field a person states for ordering alone. It creates no dependency edge, changes no eligibility result, and enters no measure because it is not an event.

Delay cost carries one ordering fact and no associated planning practice:

- Fixed date is absent because it needs a date on an open entry, which would be a second stored ordering input.
- Capacity allocation is absent because it is a planning practice, while the record states intent rather than policing it.
- No rule limits entries with immediate delay cost. Multiple entries remain legal, and a workflow verb warns at exit 0 and changes nothing.
- Delay cost marks a row. It creates no swimlane or sixth lane.

## A watch on work in flight

A watch is legal on work already in flight. A question means the project does not know what to build, so the entry leaves the work lane. A watch means the project knows what to build and the outside world moved after work started. Sending that entry back to the scheduled lane would falsely say it never started and its clock never ran. The watch keeps that blocked row visible where it carries the most signal.

## A watch and its reading

The `Read` line's sentence records a dated observation rather than a remote field. It can say that an item was open because the date keeps that statement true as an observation about that day.

The two project thresholds stay separate. `days` measures this project's own work going quiet, while `watch_days` measures an outside item going unread. A release cut monthly and a ticket triaged daily can also use different `Every` periods.

Clearing a watch creates no separate event store. The section enters and leaves `watches.md` through ordinary commits, so `git log` on that file records when the block appeared and lifted, with its author and date. A future blocked-duration measure can fold from that history without storing or computing the measure here.

## Reading the order back

A computed order a reader cannot explain is an order they cannot challenge. `wipctl board` marks immediate delay cost with `^`, ordinary work with no glyph, and deferred delay cost with `.`. The marks survive a pipe and a colour-free terminal.

`wipctl board --why` and `wipctl next --why` name the one constraint, preference, or tie-break that placed each row against the row above it. They show one reason, not the whole procedure or a score.

Where the final ID tie-break decides a row, the board says so without a flag. The tool chose where the record expressed no preference, so the output must expose that fact.

The loop is read, edit the record, read again. Neither flag writes a byte, takes a lock, or exits non-zero.

## Computed order, human preference

The tool computes rank. It does not author priority. Fixed constraints keep the plan valid. Declared preferences apply facts a person wrote. The final ID tie-break chooses only when those facts express no preference. There is no stored order to validate, repair, or offer as a set of legal positions.
