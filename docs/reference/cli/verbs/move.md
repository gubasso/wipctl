# wipctl move

The lane change as a recorded event. A lane change is this verb and never a file edit: an edit changes where an entry sits and records no when.

## Usage

```text
wipctl move <id> --to <lane> [--outcome <o>] [--succeeded-by <id>] [--closed <date>] [--dry-run]
```

- `<id>` — REQUIRED positional; missing is exit 2 `name the entry to move`.
- `--to <lane>` — REQUIRED; one of the five lanes.
- `--outcome <done|cut|reshaped>` — REQUIRED by and only by `--to closed`.
- `--succeeded-by <id>` — REQUIRED by and only by `--outcome reshaped`.
- `--closed <YYYY-MM-DD>` — only with `--to closed`; defaults to today in UTC. An explicit value MUST equal the UTC day of the once-read transition instant — the flag pins the date an invocation expects, it is deliberately not a backdater, because the closing journal event MUST enter `closed` on the entry's `closed:` date; a mismatch is exit 2.
- `--dry-run` — report the same transition, write nothing.
- `--help` usage on stdout exit 0; unknown option or a flag missing its value: exit 2.

## Preflight, entire and in order, before the first byte

1. The record MUST already validate — else exit 1 `the record does not validate as it stands; fix it before moving an entry`.
2. The destination MUST be a real lane — else exit 2.
3. The id MUST be in some lane — else exit 2.
4. Already in the destination — exit 2, deliberately not 1, so a retry loop cannot spin. Under concurrent agents this is the lost-race signal: another writer took the entry first, and a caller retrying the identical command would be retrying a mistake. The message says so and names the correction:

   ```text
   wipctl: rate-limit-the-search-endpoint is already in doing
   wipctl: another writer took it; run 'wipctl start' to take the
           next startable entry
   ```

5. Closing-flag consistency (exit 2 for a misuse; a `--succeeded-by` naming an entry in no lane is exit 1).
6. An entry moving to `doing` or `review` MUST have every need closed and no blocking open question — else exit 1 naming the need or the question, and the act that unblocks it.

A refused move MUST leave every file byte-identical.

## Write set and report

Three files change: the source lane file, the destination lane file, and `journal/<id>.tsv` (created on the entry's first transition). Then the rank repair runs, and the transaction commits — all inside the same lock holding. The report on stdout, paths relative to the plan repository:

```text
lanes/todo.yml                            -1 entry
lanes/doing.yml                           +1 entry
journal/rate-limit-the-search-endpoint.tsv  +1 event  todo -> doing
lanes                                     ranking repaired
plan: move rate-limit-the-search-endpoint from todo to doing   committed
```

The entry's original lines MUST relocate verbatim, appended at the bottom of the destination lane — arrival makes no rank claim, so the verb MUST NOT insert anywhere a person could have meant; the rank repair then restores legality, and any finer rank is a person's edit or `fix`. Appending is also what keeps `closed.yml` in ascending close-date order. Closing appends `outcome`, `closed`, and `succeeded_by` at the block's end. A lane emptied by the move MUST re-declare `stories: []`; a lane receiving its first entry MUST convert `stories: []` back to a block sequence.

`--dry-run` runs the entire preflight, prints the same report, then `wipctl: warning: dry run: nothing was written`, exit 0; every file MUST be left byte-identical, and the lock is not taken — a run that writes nothing is a read.

## Reopening

Moving out of `closed` into any lane is a transition this verb performs. The block MUST travel with `outcome`, `closed`, `succeeded_by`, and `delivered` stripped; a `closed -> <lane>` event MUST be appended; the original closing event stays in the journal. Reopening into `doing` or `review` MUST be gated like any other entry into a work lane. Re-closing later records a fresh outcome; the prior one is not recovered.

## The commit

The transition commits to the plan trunk inside the transaction, staging only the written paths, in the fixed grammar: `plan: move <id> from <from> to <to>`, or `plan: close <id> as <outcome>` when the destination is `closed`. There is no opt-out; the contract and the refused-commit state are in [../conventions.md](../conventions.md). The host repository is not touched.
