# Lane files

One YAML file per lane under `lanes/`. The file is the lane: there is no `status` field. The sequence position is the ranking: there is no `priority` field. `lane.schema.json` owns each file's shape; the cross-file checker owns everything spanning two files.

## File shape

```yaml
lane: todo
stories:
  - id: rate-limit-the-search-endpoint
    type: story
    points: 2
    summary: "Callers of the search endpoint are limited per token, and the limit is announced in the response headers rather than discovered by being cut off."
    needs: [secure-session-storage]
    epic: session-hardening
```

`lane` MUST equal the file's basename. `stories` is a sequence ordered highest priority first; an empty lane MUST declare `stories: []`.

## Entry fields

- `id` — REQUIRED. The title's slug per [ids.md](./ids.md). The id is the story document's filename stem; the agreement is checked in both directions. MUST be unique across every lane, every epic, every initiative, and every tombstone.
- `type` — REQUIRED. `story`, `spike`, or `chore`. Decides the worked-example gate: a `story` or `spike` MUST carry a fenced block under its `Example` heading; a `chore` need not.
- `points` — REQUIRED. `1`, `2`, or `3`. `1`: confirmation only — named tests settle acceptance, at most one unchanged contract involved. `2`: one judgment — an interface, name, message, or existing contract needs human reasoning. `3`: two judgments, or one hard to reverse, such as a schema or a security control. `4` is not a value; work above three splits along the judgments its acceptance already names.
- `summary` — REQUIRED. MUST be one double-quoted line of 60 to 400 characters with no leading or trailing whitespace. A restatement of the story for a reader scanning the lane, never a second specification. The summary says what the work is; `note` says how the entry is being handled.
- `needs` — OPTIONAL. One-line flow sequence of unique ids this entry depends on. The only sequencing fact the record stores.
- `epic` — OPTIONAL. One id naming an existing document under `epics/`. A gated reference and a membership claim; never a dependency, never a sequencing fact. Membership above the epic is the epic document's own claim, never the entry's: an entry names its epic and nothing higher ([documents.md](./documents.md)).
- `tags` — OPTIONAL. One-line flow sequence of unique slugs. Nothing gates on a tag.
- `branch` — OPTIONAL. The code branch carrying the entry's work, in the host repository or wherever the work lives. A code reference the record states because plan and code no longer share commits; opaque to every check beyond shape, because the record cannot verify another repository.
- `delivered` — OPTIONAL, `closed.yml` only. The commit that delivered the work, as its identifier in the host's version control. The same standing as `branch`: stated, searchable, and never verified.
- `outcome` — REQUIRED in `closed.yml`, and MUST NOT appear elsewhere. `done`, `cut`, or `reshaped`.
- `closed` — REQUIRED in `closed.yml`, and MUST NOT appear elsewhere. ISO date (`YYYY-MM-DD`); the velocity source.
- `succeeded_by` — REQUIRED exactly when `outcome` is `reshaped`, and MUST NOT appear otherwise. One id.
- `note` — OPTIONAL free prose about handling (why parked, who is waited on). Nothing parses it.

## Lane semantics

- `backlog` — agreed, not scheduled.
- `todo` — scheduled. The topmost entry is what to start: ranking rule R1 keeps every eligible entry above the ineligible, so whenever the lane holds a startable entry, the head is one.
- `doing` — in flight. Entry requires every need closed and no blocking question.
- `review` — waiting on a reader. Same entry gate as `doing`. The points an iteration counts are the ones a review accepted.
- `closed` — finished, append-ordered by close date ascending, stable within a day. A close date out of sequence is a warning the repair resolves. `closed` is a lane, not a terminus: an entry MAY leave it again (see [transition-journal.md](./transition-journal.md) and the `move` contract), and a gap left by a departure is not a defect.

## Canonical YAML subset

The record accepts a deliberately narrow YAML: flat mappings, exactly one level of entry nesting, one-line flow sequences for `needs` and `tags`. Anchors, aliases, block scalars, multi-line strings, and multi-document files MUST be rejected. Summaries MUST be double-quoted; ids are written bare — the id grammar holds nothing YAML could misread. The restriction keeps the record human-scannable, lets any implementation parse it without a full YAML processor, and lets the repair relocate entry blocks byte-for-byte — comments and blank lines survive because nothing is ever re-serialised.
