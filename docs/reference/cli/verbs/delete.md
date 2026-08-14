# wipctl delete

Removes an entry that should never have existed, leaving nothing but its journal. Deletion is not
`cut`: a `cut` entry was decided against and stays in `closed.yml` with its points in the epic
denominator; a deleted entry keeps no lane presence at all.

## Usage

```text
wipctl delete <id> [--dry-run] [<plan-dir>]
```

## Contract

- Same preflight discipline as `move`: every check MUST run before the first byte; a refused
  deletion writes nothing. The verb MUST hold the zone lock.
- The verb MUST refuse while anything still names the id: an entry whose `needs` lists it, a
  `succeeded_by` naming it, or a `Blocks:` line listing it — exit 1 naming the referrer.
- Writes: removes the lane entry block, removes the one-to-one story document, and appends a
  `<lane> -> deleted` event to `journal/<id>.tsv` (creating it if the entry never moved). The
  journal is now a tombstone and the id is burned; see
  [../../record/transition-journal.md](../../record/transition-journal.md).
- The verb MUST NOT delete epic documents, sibling artifact directories, or any document the
  story referenced.
- `--dry-run` — the entire preflight runs, the same report prints, then
  `wipctl: warning: dry run: nothing was written`, exit 0; every file MUST be left
  byte-identical, and the zone lock is not taken — a run that writes nothing is a read.
- No undelete verb exists; the journal stays three columns, so no reason is recorded.

## Example

```text
$ wipctl delete a-mistyped-story-08b1
docs/plan/lanes/backlog.yml               -1 entry
docs/plan/stories/a-mistyped-story-08b1.md  removed
docs/plan/journal/a-mistyped-story-08b1.tsv  +1 event  backlog -> deleted
```
