# Plan repository and zone layout

A project's plan record lives in its own git repository — the plan repository — at a machine-level location, shared by every worktree, branch, and clone of that project on the machine, hosted wherever its operator chooses. The host repository keeps only the project's identity. How an invocation finds the plan repository is [resolution.md](./resolution.md).

## The four locations

```text
host repo:
  .wipctl.toml                  identity only; committed; travels with every clone

$XDG_DATA_HOME/wipctl/
  projects/
    <project_id>/
      plan-repo/                the plan zone, its own git repository, trunk only

$XDG_STATE_HOME/wipctl/
  projects/
    <project_id>/               sync bookkeeping, diagnostics

$XDG_CACHE_HOME/wipctl/
  projects/
    <project_id>/               derived views; disposable; no verb reads it back

$XDG_RUNTIME_DIR/wipctl/
  <project_id>.lock             writer lock; transaction-scoped; never in the record
```

The plan record is user data, so it lives under the data directory — back it up — not state and not cache. The cache keeps exactly the role it already has: disposable, rebuildable, authoritative for nothing. The checkout directory inside the slot is named `plan-repo`, explicitness over brevity.

Each of `XDG_DATA_HOME`, `XDG_STATE_HOME`, and `XDG_CACHE_HOME` has a default the base-directory specification gives, and each MUST be honoured when the variable is unset or empty:

```text
XDG_DATA_HOME    $HOME/.local/share
XDG_STATE_HOME   $HOME/.local/state
XDG_CACHE_HOME   $HOME/.cache
```

A variable holding a relative path is invalid and MUST be treated as unset. It gives `XDG_RUNTIME_DIR` no default path at all, and unset is ordinary in cron jobs, containers, and remote sessions with no session manager — exactly where agents run. Since the lock is the whole same-machine guarantee, an unstated fallback would be no lock at all in those environments. When `XDG_RUNTIME_DIR` is unset or empty, the lock directory MUST be `/tmp/wipctl-<uid>`, created mode `0700`, a warning MUST be printed naming the replacement, and `doctor` MUST report which of the two directories is in use.

## The zone

The plan repository's working tree is the zone:

```text
plan-repo/
  config.toml                   identity and configuration; see config.md
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
  initiatives/
    <id>.md                     one document per initiative; earned the same way
  journal/
    <id>.tsv                    one transition stream per entry; created on first transition
  pending/
    <id>.yml                    one fragment per captured entry; created on demand
```

## Rules

- All five lane files MUST exist at all times, even when empty. A missing lane file is a validation failure, not an empty lane.
- `config.toml` MUST exist and MUST state the `project_id` the repository serves, so an attach to the wrong repository surfaces as a disagreement between two committed facts (see [resolution.md](./resolution.md)).
- `stories/` MUST exist from the scaffold onward and starts empty. A story document's filename stem is the entry's id; the two are checked against each other in both directions.
- `epics/` is not scaffolded. It appears when the first epic earns a document. `initiatives/` is earned the same way, and its absence is legal and means the project uses two tiers.
- `journal/<id>.tsv` appears on an entry's first transition. An entry that never moved has no journal file, not an empty one.
- `pending/` is created on demand by capture. Its absence is legal and means nothing is pending.
- Derived state MUST NOT be stored in the zone: no velocity, no board, no resolved epic plan, no blocked flags. Every view is computed from these files on every read.
- The lock MUST NOT live in the zone. It is never committed, never cloned, and answers no question about the plan.

## The plan trunk

- The plan repository has exactly one published branch, created by the tool, and no others.
- Every semantic mutation is one commit in the fixed grammar `plan: <act>` — the full grammar and the transaction rules are in [../cli/conventions.md](../cli/conventions.md). One mutation is one commit: a `move` touching two lanes and a journal is one transaction and one commit, never one commit per keystroke. The tool stages only paths it owns.
- Push is backup and replication. It MUST NOT be part of a write transaction; [../cli/verbs/sync.md](../cli/verbs/sync.md) owns replication.
- Plan history is append-only: the tool MUST NOT force-push, ever.
- The plan repository carries its own hook set — validate at commit and at push, a single-branch guard, a no-force guard — installed by the tool. The plan repository is created by the tool and owned by the tool, so installing hooks into it is the scaffold writing what it created, which ADR-0011's create-only rule permits; the host repository still receives exactly one file it did not have, and no merged content. Hooks are early gates, bypassable by construction, so correctness lives in the tool's transactions, and hosted branch protection backstops publication where the operator hosts one. A missing hook installation is a `doctor` failure, never a silent state.

## What is supplied by the product, not the zone

The schemas, the scaffold payload, and the worked example project are shipped by the installed product under a shared data directory (`schema/`, `init/`, `example/`). They MUST NOT be copied into a zone: a copy would be a second source of truth, drifting from the version the gate runs.
