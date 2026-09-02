# wipctl rename

Changes an entry's title, and with it the id, because the two can never disagree: `id == slugify(title)` is an invariant ([../../record/ids.md](../../record/ids.md), ADR-0052). Renaming is a supported operation with proper rules, not a drift a reader has to tolerate.

## Usage

```text
wipctl rename <id> "<new title>"
wipctl rename <id> --id <new-id>
```

The first form derives the new id by slugifying the new title. The second passes an explicit id — the qualifying-postfix case, where the title is right as written and only the id must differ; the document's title line keeps the title, and the explicit id MUST still satisfy every mint check. Both forms together, or neither, is exit 2. `--help` prints usage on stdout, exit 0.

## Contract

- `rename` is a writer, under the lock, in one transaction. Its write set is everything that names the id:

  ```text
  the lane entry's id
  stories/<old>.md          -> stories/<new>.md, and its title line rewritten
  stories/<old>/            -> stories/<new>/, when the sibling directory exists
  journal/<old>.tsv         -> journal/<new>.tsv, history travelling with the entry
  pending/<old>.yml         -> pending/<new>.yml, when the entry is still a fragment
  every needs entry naming the old id, in every lane
  every succeeded_by naming the old id
  every Blocks: line naming the old id, in open-questions.md
  every epic field naming the old id, when an epic is renamed
  every Initiative section naming the old id, when an initiative is renamed
  ```

- The new id is minted the same way a fresh one is, under the same lock, against the live record and every tombstone. A rename onto a taken or burned id is refused with the same message as at capture and the same suggestion to rephrase.
- The old id is burned: a fresh one-line tombstone is written at `journal/<old>.tsv` whose event's destination is `renamed` ([../../record/transition-journal.md](../../record/transition-journal.md)). A reference to the old id then fails loudly rather than resolving to nothing, and rename opens no second way for an id to become free.
- The rename is narrated in the document's own `Revisions` section, as a dated line naming the old id — the heading that already exists for changes to the agreement, so the journal's three-field format needs no fourth field to carry a pointer.
- Preflight before the first byte: a refused rename leaves every file byte-identical. One commit: `plan: rename <old> to <new>`.
- The verb emits one report line per rewritten path and offers no machine format, so under ADR-0013 it owes no schema.

## Example

```text
$ wipctl rename rate-limit-search "Announce the search rate limit"
lanes/todo.yml                                  id rewritten
stories/rate-limit-search.md                 -> stories/announce-the-search-rate-limit.md
journal/rate-limit-search.tsv                -> journal/announce-the-search-rate-limit.tsv
lanes/backlog.yml                               1 needs reference rewritten
open-questions.md                               1 Blocks: reference rewritten
journal/rate-limit-search.tsv                   tombstoned: renamed
plan: rename rate-limit-search to announce-the-search-rate-limit   committed
```

## Diagnostics

```text
the new id is taken                                                       exit 1
  wipctl: id announce-the-search-rate-limit is taken by an entry in backlog
  wipctl: rephrase the title, or pass --id announce-the-search-rate-limit-v2

the new id is burned                                                      exit 1
  wipctl: id fix-the-parser is held by a tombstone: deleted 2026-08-30
  wipctl: that id is burned for the life of the record; rephrase the title
```
