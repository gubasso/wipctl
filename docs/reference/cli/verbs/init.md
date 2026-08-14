# wipctl init

The scaffold: the only verb that writes into a project it did not previously touch. Create-only —
a path is written when absent, otherwise reported and left alone. There is no `--force` and no
overwrite. `init` MUST NOT walk upward: scaffolding into a parent project by surprise is exactly
the failure the walk would create.

## Usage

```text
wipctl init [--root DIR] [--plan-dir PATH] [--iteration-start YYYY-MM-DD]
            [--headings-gate] [--print-hooks] [--dry-run]
```

- `--root DIR` — the project root to scaffold into; defaults to the working directory.
- `--plan-dir PATH` — where the zone lives, relative to the root. No default ever. Absent with a
  terminal on stdin, the verb MUST prompt (on stderr) until answered; absent without a terminal,
  exit 2. The path MUST be relative, carry no `.` or `..` segment, and match the config path
  grammar — each violation exit 2. A trailing slash is stripped.
- `--iteration-start YYYY-MM-DD` — the anchor date; MUST be a real calendar date (exit 2).
  Defaults to today, which makes it the one non-deterministic output — a gated invocation MUST
  always pass the flag.
- `--headings-gate` — also write the two heading-shape gate configurations and print the hook
  entries. The flag MUST be refused when a discovered general lint configuration already sets a
  required-headings rule at any value, because it would merge over the dedicated files and
  silently disable the gate.
- `--print-hooks` — print the hook fragment on stdout with the zone path substituted, write
  nothing else, exit 0. Re-runnable against an existing zone.
- `--dry-run` — report what would be written, write nothing.

## What is written

The root config `.wipctl.toml` (generated with the resolved `plan_dir` and start date;
`length_days = 14` is the payload's one baked value) and the zone: `README.md`, `AGENTS.md` (the
method, self-sufficient, with gate names substituted from what the build actually carries),
`charter.md` (placeholders), `open-questions.md`, the five lane files (each `stories: []`), and an
empty `stories/`. Not written: `epics/` (earned, not seeded), a first story (it would have to be
deleted before it could be believed), the templates (their shipped location is printed for a
person to copy). The full payload contract is
[../../scaffold-payload.md](../../scaffold-payload.md).

## Report

One line per path, a two-word vocabulary in a fixed column:

```text
wrote   .wipctl.toml
wrote   docs/plan/AGENTS.md
exists  docs/plan/charter.md
```

Under `--dry-run`, `wrote` becomes `would`. Trailing stderr lines name the shipped schema and
template directories.

## Preflight and self-check

All refusals MUST be collected, then exit 1: an existing `.wipctl.toml` declaring a different
`plan_dir`; a destination that is a symlink or an existing non-directory where a directory
belongs; the headings-gate conflict above; a scaffold payload missing from the data directory
(broken install). After emitting, the zone MUST be validated by the shipped checker:
`self-check: ok`, or a named skip under `--dry-run`, or exit 1
`the emitted zone does not pass validation`. A second run over a complete zone MUST write nothing
and exit 0.
