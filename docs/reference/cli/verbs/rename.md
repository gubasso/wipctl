# wipctl rename

Changes an artifact's title, its id, or both, holding the two in agreement: an id is its title's slug plus at most a qualifying postfix, and that is an invariant ([../../record/ids.md](../../record/ids.md), ADR-0052 and ADR-0058). Renaming is a supported operation with proper rules, not a drift a reader has to tolerate. It binds every artifact the invariant binds — a story, an epic, an initiative, and a question — and the write set below covers each; a question has no lane entry, no journal, and no fragment, so its rename touches its heading and its inbound references alone.

## Usage

```text
wipctl rename <id> "<new title>" [--id <new-id>]
wipctl rename <id> --id <new-id>
```

A new title, an explicit id, or both; neither is exit 2. `--help` prints usage on stdout, exit 0.

- A new title alone derives the new id by slugifying it.
- An explicit id alone keeps the current title — the qualifying-postfix case, where the title is right as written and only the id must differ.
- Both together set the title and take the given id, which is what a rename onto a taken slug needs: the new title's own slug is unavailable, so the operator supplies the postfixed id alongside it. The same pairing `new` offers at capture ([new.md](./new.md)).

In every form the given id MUST agree with the resulting title under the invariant — the title's slug, plus at most a qualifying postfix ([../../record/ids.md](../../record/ids.md)) — and an id that agrees with it in no other way is exit 2, because the invocation itself is wrong.

Every other mint check is a check against the record, so its refusal is exit 1: a taken or burned id, and a postfix whose base slug is free. The last names the free base and the choice:

```text
the postfix qualifies nothing                                             exit 1
  wipctl: id rate-limit-v2 carries a postfix, but rate-limit is free
  wipctl: a postfix disambiguates an id the record holds; rename to
          rate-limit, or choose a title of its own
```

## Contract

- `rename` is a writer, under the lock, in one transaction. Its write set is everything that names the id:

  ```text
  the lane entry's id, when an entry is renamed
  stories/<old>.md          -> stories/<new>.md, and its title line rewritten
  stories/<old>/            -> stories/<new>/, when the sibling directory exists
  epics/<old>.md            -> epics/<new>.md, when an epic is renamed
  initiatives/<old>.md      -> initiatives/<new>.md, when an initiative is renamed
  the question's heading in open-questions.md, when a question is renamed
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
- One case leaves no tombstone, and it is the only one: a rename resolving an id collision `sync` reported ([sync.md](./sync.md)), where the same slug was minted on two machines. The id is not freed there — the other capture keeps it — so burning it would refuse the surviving claimant its own id. The verb MUST detect that case from the reported collision rather than infer it, and MUST say in its report that the id was not burned and why.
- That recovery MUST rephrase: the new id carries no qualifying postfix, and a postfixed one is exit 1. The losing capture holds the base slug at the moment of the rename, so the ordinary qualification check would pass — but this rename frees the base without burning it, and the surviving capture that keeps it has not reconciled in, so the committed result would carry a postfix qualifying nothing and fail the gate the rename's own commit must pass. Rephrasing is what the collision asks for anyway: two captures that slug identically wanted different names.
- The rename is narrated in the document's own `Revisions` section, as a dated line naming the old id — the heading that already exists for changes to the agreement, so the journal's three-field format needs no fourth field to carry a pointer. A question has no `Revisions` section and owes no line: its shape is the heading, `Raised:`, `Blocks:`, and `Exit:` ([../../record/open-questions.md](../../record/open-questions.md)), and its rename is carried by the heading itself and by the plan trunk commit.
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

the new id is taken, resolving a collision                                exit 1
  wipctl: id announce-the-search-rate-limit is taken by an entry in backlog
  wipctl: this rename resolves an id collision, so a postfix is not
          available; rephrase to a title whose slug is free

a postfix was passed, resolving a collision                               exit 1
  wipctl: --id rate-limit-the-search-endpoint-v2 carries a postfix
  wipctl: this rename resolves an id collision, and the base slug leaves
          the record with it, so the postfix would qualify nothing;
          rephrase to a title whose slug is free

the new id is burned                                                      exit 1
  wipctl: id fix-the-parser is held by a tombstone: deleted 2026-08-30
  wipctl: that id is burned for the life of the record; rephrase the title
```
