# wipctl init

The scaffold: mints the project's identity, writes one file into the host repository, and creates the plan repository in this machine's attachment registry. Create-only — a path is written when absent, otherwise reported and left alone. There is no `--force` and no overwrite. `init` MUST NOT walk upward: scaffolding a parent project by surprise is exactly the failure the walk would create.

## Usage

```text
wipctl init [--root DIR] [--iteration-start YYYY-MM-DD] [--print-hooks] [--dry-run]
```

- `--root DIR` — the project root to receive `.wipctl.toml`; defaults to the working directory.
- `--iteration-start YYYY-MM-DD` — the anchor date; MUST be a real calendar date (exit 2). Defaults to today, which makes it the one non-deterministic input — a gated invocation MUST always pass the flag.
- `--print-hooks` — print the plan repository's hook set on stdout, write nothing else, exit 0. Re-runnable against an existing plan repository, which is how a hook set is inspected or reinstalled after `doctor` reports it missing.
- `--dry-run` — report what would be written, write nothing.

## What is written, in two places

Into the host repository, exactly one file: `.wipctl.toml`, carrying the minted `project_id` and nothing else ([../../record/config.md](../../record/config.md)). The id derives from the project's name and is made unique against this machine's attachment registry by the postfix escalation in [../../record/resolution.md](../../record/resolution.md). The host receives no other file, no directory, and no commit.

Into the attachment registry slot, the plan repository: a git repository at `$XDG_DATA_HOME/wipctl/projects/<project_id>/plan-repo/` holding the zone — `config.toml` (generated with the minted id, the given or current start date, and `length_days = 14`, the payload's one baked value), `README.md`, `AGENTS.md` (the method, self-sufficient, with gate names substituted from what the build actually carries), `charter.md` (placeholders), `open-questions.md`, the five lane files (each `stories: []`), and an empty `stories/` — plus the hook set (validate at commit and push, the single-branch guard, the no-force guard) and the initial commit on the plan trunk. The plan repository is created by the tool and owned by the tool, so installing hooks into it is the scaffold writing what it created (ADR-0011). No remote is configured; the operator adds one whenever hosting is wanted.

Not written: `epics/` and `initiatives/` (earned, not seeded), a first story (it would have to be deleted before it could be believed), the templates (their shipped location is printed for a person to copy). The full payload contract is [../../scaffold-payload.md](../../scaffold-payload.md).

A project cloned from elsewhere already carries `.wipctl.toml`; the verb for that machine is `attach`, not `init`, and an `init` finding an existing config says so.

## Report

One line per path, a two-word vocabulary in a fixed column; host paths relative to the root, plan paths relative to the plan repository:

```text
wrote   .wipctl.toml
wrote   ~/.local/share/wipctl/projects/payments-acme/plan-repo
wrote   config.toml
wrote   AGENTS.md
exists  charter.md
```

Under `--dry-run`, `wrote` becomes `would`. Trailing stderr lines name the shipped schema and template directories and, when the runtime directory fallback is in use, the lock-directory warning.

## Preflight and self-check

All refusals MUST be collected, then exit 1: an existing `.wipctl.toml` (the project has an identity; the message names `attach` for a machine that lacks the plan and reports the id it found); a registry slot already holding a different project's plan repository; a destination that is a symlink or an existing non-directory where a directory belongs; a scaffold payload missing from the data directory (broken install). After emitting, the zone MUST be validated by the shipped checker: `self-check: ok`, or a named skip under `--dry-run`, or exit 1 `the emitted zone does not pass validation`. A second run over a complete project MUST write nothing and exit 0.

## Diagnostics

```text
the project already has an identity                                       exit 1
  wipctl: .wipctl.toml exists and names payments-acme
  wipctl: this project is already a wipctl project; to bring its plan onto
          this machine, run 'wipctl attach <plan-repo-url>' or
          'wipctl attach --create'
```
