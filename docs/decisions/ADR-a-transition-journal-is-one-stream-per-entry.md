# A transition journal is one stream per entry

## Context and Problem Statement

Events could embed in the lane entry, join one global log, or split one file per entry. The choice decides merge behaviour and what a rank repair can put at risk.

## Considered Options

- One append-only TSV stream per entry
- Events embedded in the lane entry — the rank repair relocates entry blocks; history riding in a block being moved is history at risk in every repair
- One global log — every pair of branches that moved any two entries conflicts in the same file; per-entry files conflict only when two branches moved the same entry, which is the conflict a human should see
- A timestamped filename per event — directory churn without a benefit; the stream's order is the fact

## Decision Outcome

Chosen option: one append-only stream per entry, `journal/<id>.tsv` — conflicts appear exactly where a human should see them. The instant is written one way — RFC 3339 UTC with `Z`, second precision — so lexical order is chronological order and a future-dated event is detectable arithmetic, not a parsing question. Append-only is what makes the last line the entry's current lane.

## Consequences

- Good: merge arithmetic matches the method's concurrency story, and the format needs no schema and no parser beyond splitting on tabs.
- Good: the checker's journal rules stay line-local plus one agreement check.
- Bad: the journal directory grows one file for every entry that ever moved.

## Status

Accepted
