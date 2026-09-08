# Transitions

## Why a lane change is recorded

Lane files say where an entry is. Every flow question is a question about when it arrived somewhere: how old is this work, how long did review hold it, and how many times did it come back. So a lane change is an event the record itself states. The move verb relocates the entry, appends the event, and repairs the ranking, in one transaction. An edit records no when, which is why the gate fails a lane that disagrees with its journal.

## Why git is a witness, not a source

Deriving transitions from version-control history fails three independent ways. History can be rewritten by a rebase. It can be collapsed, because a squash merge folds a round trip into nothing. It can be absent, as in a shallow clone.

The record states its own events. The plan trunk's commits merely carry the files between machines, and no verb reads that history back to answer a question the files answer. The host repository's history is further still. No verb touches it at all, so the record owes nothing to how the host rebases, squashes, or clones.

## The verb covers the whole space

Nobody is ever driven back to editing a lane file by hand:

- Forward moves and closes. The move verb takes an outcome, a close date, and a successor where applicable.
- Reopening. The move verb takes an entry out of the closed lane, with the terminal fields stripped and the closing event preserved. Closing is not terminal. The append-only shape of the closed lane is a convention about arrival, not a property the record depends on.
- Deletion. The delete verb removes an entry that never belonged in the record. It leaves a tombstone journal and burns the id. Deletion is not a cut, because a cut is a decision the record keeps.
- Taking work. The take verb is the atomic read-and-take, so concurrent sessions fan out instead of racing for one head, as [concurrency.md](./concurrency.md) explains.
- Two writers at once. The writer lock guarantees that both transitions survive. The second writer waits, bounded, and fails naming the holder rather than losing an update.

## What the journal deliberately does not prove

Nothing proves the verb was used rather than the file edited. The lane-agreement gate catches the divergence after the fact, and that is enough. Nothing records who moved an entry or why. Three columns is the whole format, and the plan trunk commit carries the who for projects that read their plan's log.
