# Plan zone layout

The plan zone is one directory, placed anywhere inside the host project's documentation directory, at any depth. Its location is declared by `plan_dir` in `.wipctl.toml` at the project root and is never assumed. See [config.md](./config.md).

## Contents

```text
<zone>/
  README.md                     orientation for a reader arriving at the zone
  AGENTS.md                     the method, self-sufficient, written by the scaffold
  charter.md                    what the project is for, pillars, no-gos, cadence
  open-questions.md             questions that block named entries
  lanes/
    backlog.yml                 agreed, not scheduled
    todo.yml                    scheduled; the topmost entry is what to start
    doing.yml                   in flight
    review.yml                  waiting on a reader
    closed.yml                  finished, ordered by close date
  stories/
    <id>.md                     one document per story
    <id>/                       optional sibling directory for non-narrative artifacts
  epics/
    <id>.md                     one document per epic; the directory exists only when earned
  journal/
    <id>.tsv                    one transition stream per entry; created on first transition
  pending/
    <id>.yml                    one fragment per captured entry; created on demand
```

## Rules

- All five lane files MUST exist at all times, even when empty. A missing lane file is a validation failure, not an empty lane.
- `stories/` MUST exist from the scaffold onward and starts empty. A story document's filename stem is the entry's id; the two are checked against each other in both directions.
- `epics/` is not scaffolded. It appears when the first epic earns a document.
- `journal/<id>.tsv` appears on an entry's first transition. An entry that never moved has no journal file, not an empty one.
- `pending/` is created on demand by capture. Its absence is legal and means nothing is pending.
- Derived state MUST NOT be stored in the zone: no velocity, no board, no resolved epic plan, no blocked flags. Every view is computed from these files on every read.

## What is supplied by the product, not the zone

The schemas, the scaffold payload, and the worked example project are shipped by the installed product under a shared data directory (`schema/`, `init/`, `example/`). They MUST NOT be copied into a zone: a copy would be a second source of truth, drifting from the version the gate runs.
