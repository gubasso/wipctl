# Transitions

## Why a lane change is recorded

Lane files say where an entry is; every flow question — how old is this work, how long did review
hold it, how many times did it come back — is a question about when it arrived somewhere. So a
lane change is an event the record itself states: `wipctl move` relocates the entry, appends the
event, and repairs the ranking, in one atomic act. An edit records no when, which is why the gate
fails a lane that disagrees with its journal.

## Why git is a witness, not a source

Deriving transitions from version-control history fails three independent ways: history can be
rewritten (a rebase), collapsed (a squash merge folds a round trip into nothing), or absent (a
shallow clone). And requiring a git practice would exceed the single host assumption. The record
states its own events; version control merely carries the files. The one place the program
touches git is the opt-in transition commit — a convenience the config enables, never a source
anything reads back.

## The verb covers the whole space

Nobody is ever driven back to editing a lane file by hand:

- Forward moves and closes — `move`, with outcome, close date, and successor where applicable.
- Reopening — `move` out of `closed`, terminal fields stripped, the closing event preserved.
  Closing is not terminal; the append-only shape of `closed.yml` is a convention about arrival,
  not a property the record depends on.
- Deletion — `delete`, for an entry that should never have existed, leaving a tombstone journal
  and burning the id. Deletion is not `cut`: `cut` is a decision the record keeps.
- Two writers at once — the zone lock guarantees both transitions survive; the second writer
  waits, bounded, and fails naming the holder rather than losing an update.

## What the journal deliberately does not prove

Nothing proves the verb was used rather than the file edited — the lane-agreement gate catches
the divergence after the fact, which is enough. Nothing records who moved an entry or why: three
columns is the whole format, and the commit, when enabled, carries the who for projects that want
it.
