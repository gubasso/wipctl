# Configuration

`.wipctl.toml` lives at the project root. The directory holding it is, by definition, the project root: every verb discovers the zone by walking upward to this file, so nothing else marks the root. The file is hidden and carries the product's name, not the command's verbs.

No key has a read-time default: a verb finding a required key absent MUST fail the check rather than assume a value, because a value guessed on a project's behalf is a value nobody wrote down. The scaffold generates the file complete — the resolved `plan_dir`, the given or current start date, `length_days = 14` — and from that moment its operator owns every value in it (see [../cli/verbs/init.md](../cli/verbs/init.md)).

## Keys

```toml
plan_dir = "docs/plan"

[iteration]
start = 2026-07-06
length_days = 14

[commit]
type = "docs"
scope = "plan"

[aging]
threshold_days = 21
```

- `plan_dir` — REQUIRED. Root-relative path to the plan zone. Grammar: one or more segments of `A-Z a-z 0-9 _ . -` joined by single slashes; absolute paths, trailing slashes, empty segments, and `.` or `..` segments MUST be rejected. `docs/plan` is only the most common answer, never a default.
- `iteration.start` — REQUIRED. Calendar date anchoring the first iteration window.
- `iteration.length_days` — REQUIRED. MUST be an integer of at least 1. Together with `start` it defines every velocity window; changing either restarts the series.
- `commit` — OPTIONAL table. Its presence makes `move` commit each transition. When present, both keys are REQUIRED:
  - `commit.type` — MUST be one of `feat fix docs style refactor perf test build ci chore revert`.
  - `commit.scope` — MUST be lowercase kebab, at most two slash-separated levels: `^[a-z0-9]+(-[a-z0-9]+)*(/[a-z0-9]+(-[a-z0-9]+)*)?$`.
- `aging` — OPTIONAL table.
  - `aging.threshold_days` — MUST be an integer of at least 1. The age past which `wipctl aging` marks a row. Absent, bars render with no marks — there is no default threshold.

Unknown keys MUST be rejected at every level. `config.schema.json` owns the types and constraints; the cross-file checker owns presence and agreement.

## Diagnostics

- No `.wipctl.toml` at or above the zone — failure: the record cannot state where its project root is.
- A required key absent — failure naming the field.
- The declared `plan_dir` naming a different directory than the one being checked — failure naming both, compared after path resolution so symlinks and trailing slashes do not read as disagreement.
- A malformed value — the schema's job, reported by the schema half of `validate`.

## Discovery

A verb given no zone starts its upward walk at the working directory; a verb given a directory starts there. The first `.wipctl.toml` found upward marks the root; `plan_dir` joined to the root is the zone. Finding no config is a usage error (exit 2) — the invocation named no project. A config found but incomplete, or disagreeing with the directory given, is a failed check (exit 1) — the stale declaration is caught where it happened. `init` alone MUST NOT walk: scaffolding into a parent project by surprise is the failure mode the walk would create.
