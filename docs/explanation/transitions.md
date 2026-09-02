# Transitions

## Why a lane change is recorded

Lane files say where an entry is; every flow question — how old is this work, how long did review hold it, how many times did it come back — is a question about when it arrived somewhere. So a lane change is an event the record itself states: `wipctl move` relocates the entry, appends the event, and repairs the ranking, in one atomic act. An edit records no when, which is why the gate fails a lane that disagrees with its journal.

## Why git is a witness, not a source

Deriving transitions from version-control history fails three independent ways: history can be rewritten (a rebase), collapsed (a squash merge folds a round trip into nothing), or absent (a shallow clone). The record states its own events; the plan trunk's commits merely carry the files between machines, and no verb reads that history back to answer a question the files answer. The host repository's history is further still: no verb touches it at all, so the record owes nothing to how the host rebases, squashes, or clones.

## The verb covers the whole space

Nobody is ever driven back to editing a lane file by hand:

- Forward moves and closes — `move`, with outcome, close date, and successor where applicable.
- Reopening — `move` out of `closed`, terminal fields stripped, the closing event preserved. Closing is not terminal; the append-only shape of `closed.yml` is a convention about arrival, not a property the record depends on.
- Deletion — `delete`, for an entry that should never have existed, leaving a tombstone journal and burning the id. Deletion is not `cut`: `cut` is a decision the record keeps.
- Taking work — `start`, the atomic read-and-take, so concurrent sessions fan out instead of racing for one head ([concurrency.md](./concurrency.md)).
- Two writers at once — the writer lock guarantees both transitions survive; the second writer waits, bounded, and fails naming the holder rather than losing an update.

## What the journal deliberately does not prove

Nothing proves the verb was used rather than the file edited — the lane-agreement gate catches the divergence after the fact, which is enough. Nothing records who moved an entry or why: three columns is the whole format, and the plan trunk commit carries the who for projects that read their plan's log.
