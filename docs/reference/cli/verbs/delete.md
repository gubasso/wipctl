# wipctl delete

Removes an entry that should never have existed, leaving nothing but its journal. Deletion is not `cut`: a `cut` entry was decided against and stays in `closed.yml` with its points in the epic denominator; a deleted entry keeps no lane presence at all.

## Usage

```text
wipctl delete <id> [--dry-run]
```

## Contract

- Same preflight discipline as `move`: every check MUST run before the first byte; a refused deletion writes nothing. The verb MUST hold the lock for the whole transaction, the commit included.
- The verb MUST refuse while anything still names the id: an entry whose `needs` lists it, a `succeeded_by` naming it, or a `Blocks:` line listing it — exit 1 naming the referrer and the edit or verb that clears it.
- Writes: removes the lane entry block, removes the one-to-one story document, appends a `<lane> -> deleted` event to `journal/<id>.tsv` (creating it if the entry never moved), and commits `plan: delete <id>`. The journal is now a tombstone and the id is burned for the life of the record — the mint refuses it thereafter, naming the tombstone ([../../record/ids.md](../../record/ids.md)).
- The verb MUST NOT delete epic documents, initiative documents, sibling artifact directories, or any document the story referenced.
- `--dry-run` — the entire preflight runs, the same report prints, then `wipctl: warning: dry run: nothing was written`, exit 0; every file MUST be left byte-identical, and the lock is not taken — a run that writes nothing is a read.
- No undelete verb exists; the journal stays three columns, so no reason is recorded.

## Example

```text
$ wipctl delete a-mistyped-story
lanes/backlog.yml                -1 entry
stories/a-mistyped-story.md      removed
journal/a-mistyped-story.tsv     +1 event  backlog -> deleted
plan: delete a-mistyped-story    committed
```

## Diagnostics

```text
something still names the id                                              exit 1
  wipctl: profile-composition needs a-mistyped-story
  wipctl: remove the reference — edit the needs list, or delete the
          referring entry first — then re-run the delete
```
