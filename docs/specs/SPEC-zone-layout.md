# Zone Layout Specification

<!--TOC-->

- [Purpose](#purpose)
- [The four locations](#the-four-locations)
- [The zone](#the-zone)
- [Requirements](#requirements)
  - [`zone-layout:the-record-lives-under-the-data-directory` — The record lives under the data directory](#zone-layoutthe-record-lives-under-the-data-directory--the-record-lives-under-the-data-directory)
  - [`zone-layout:a-base-directory-default-is-honoured` — A base-directory default is honoured](#zone-layouta-base-directory-default-is-honoured--a-base-directory-default-is-honoured)
  - [`zone-layout:the-lock-directory-has-a-stated-fallback` — The lock directory has a stated fallback](#zone-layoutthe-lock-directory-has-a-stated-fallback--the-lock-directory-has-a-stated-fallback)
  - [`zone-layout:every-lane-file-exists` — Every lane file exists](#zone-layoutevery-lane-file-exists--every-lane-file-exists)
  - [`zone-layout:derived-state-is-never-stored` — Derived state is never stored](#zone-layoutderived-state-is-never-stored--derived-state-is-never-stored)
  - [`zone-layout:an-earned-directory-appears-when-earned` — An earned directory appears when earned](#zone-layoutan-earned-directory-appears-when-earned--an-earned-directory-appears-when-earned)
  - [`zone-layout:a-story-filename-is-its-id` — A story filename is its id](#zone-layouta-story-filename-is-its-id--a-story-filename-is-its-id)
  - [`zone-layout:the-plan-trunk-is-one-branch` — The plan trunk is one branch](#zone-layoutthe-plan-trunk-is-one-branch--the-plan-trunk-is-one-branch)
  - [`zone-layout:plan-history-is-append-only` — Plan history is append-only](#zone-layoutplan-history-is-append-only--plan-history-is-append-only)
  - [`zone-layout:the-shipped-payload-is-never-copied-in` — The shipped payload is never copied into a zone](#zone-layoutthe-shipped-payload-is-never-copied-in--the-shipped-payload-is-never-copied-into-a-zone)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Where a project's plan record lives on a machine, and what the zone contains. The record lives in its own repository, at a machine-level location shared by every worktree, branch, and clone of that project. The host repository keeps only the identity, in a `.wipctl` directory the tool owns. The boundary runs at the tree. This domain says what exists and where, and each file's own domain says what is inside it.

## The four locations

```text
host repo:
  .wipctl/project.toml          identity only; committed; travels with every clone

$XDG_DATA_HOME/wipctl/
  projects/<project_id>/plan-repo/   the plan zone, its own repository, trunk only

$XDG_STATE_HOME/wipctl/
  projects/<project_id>/             replication bookkeeping, diagnostics

$XDG_CACHE_HOME/wipctl/
  projects/<project_id>/             derived views; disposable; no verb reads it back

$XDG_RUNTIME_DIR/wipctl/
  <project_id>.lock                  writer lock; transaction-scoped; never in the record

fallback, where XDG_RUNTIME_DIR is unset or empty:
  /tmp/wipctl-<uid>/                 created mode 0700
```

The base-directory specification gives no default for the runtime directory, and unset is ordinary in cron jobs, containers, and remote sessions with no session manager. Those are exactly where agents run, so the fallback is stated rather than left open.

```text
XDG_DATA_HOME    $HOME/.local/share
XDG_STATE_HOME   $HOME/.local/state
XDG_CACHE_HOME   $HOME/.cache
```

## The zone

```text
plan-repo/
  .wipctl/plan.toml             the plan's whole configuration: both
                                identities, the cadence, the peers it
                                names, and the sources it references
  README.md                     orientation for a reader arriving at the zone
  AGENTS.md                     the method, self-sufficient, written by the scaffold
  charter.md                    what the project is for, pillars, no-gos, cadence
  open-questions.md             questions that block named entries
  lanes/                        backlog, todo, doing, review, closed
  stories/<id>.md               one document per story, optional sibling directory
  epics/<id>.md                 one document per epic; earned, not scaffolded
  initiatives/<id>.md           one document per initiative; earned the same way
  journal/<id>.tsv              one transition stream per entry; created on first move
  pending/<id>.yml              one fragment per captured entry; created on demand
```

## Requirements

### `zone-layout:the-record-lives-under-the-data-directory` — The record lives under the data directory

The plan record MUST live under the base data directory, and the implementation MUST NOT place it under state or cache.

#### Scenario: A backup runs over a machine

- GIVEN an operator backing up their data directory
- WHEN the plan record is included
- THEN the record survives, because it is user data, while the cache stays disposable and authoritative for nothing

Verify: `cargo nextest run --test zone_layout`

### `zone-layout:a-base-directory-default-is-honoured` — A base-directory default is honoured

Where a base-directory variable is unset, empty, or relative, the implementation MUST use the specification's stated default.

#### Scenario: A container sets an empty data directory

- GIVEN an environment with the variable set to an empty string
- WHEN a path is resolved
- THEN the default under the home directory is used, because an empty value states no location

Verify: `cargo nextest run --test zone_layout`

### `zone-layout:the-lock-directory-has-a-stated-fallback` — The lock directory has a stated fallback

Where the runtime directory is unset or empty, the implementation MUST use the stated fallback path, create it mode 0700, and warn naming the replacement.

#### Scenario: An agent runs from a cron job

- GIVEN a session with no session manager and no runtime directory
- WHEN a writer takes the lock
- THEN the fallback path is used and named, because an unstated fallback means no lock at all in exactly the environments agents run in

Verify: `cargo nextest run --test zone_layout`

### `zone-layout:every-lane-file-exists` — Every lane file exists

All five lane files MUST exist at all times, including when empty.

#### Scenario: A lane is emptied

- GIVEN a lane whose last entry moves away
- WHEN the record is validated
- THEN the file remains and declares an empty sequence, because a missing file is a failure and not an empty lane

Verify: `cargo nextest run --test validation`

### `zone-layout:derived-state-is-never-stored` — Derived state is never stored

The implementation MUST NOT store derived state in the zone.

#### Scenario: A blocked flag is proposed

- GIVEN a request to record which entries are blocked
- WHEN the record already holds the dependencies and the questions
- THEN the flag is refused, because every view is computed on every read and a stored copy drifts

Verify: `cargo nextest run --test validation`

### `zone-layout:an-earned-directory-appears-when-earned` — An earned directory appears when earned

An earned directory or file MUST appear when its first member is created, and its absence MUST be legal.

#### Scenario: A project uses two tiers and names no peer

- GIVEN a project that never declares an initiative and never names another plan
- WHEN the record is validated
- THEN both absences are legal, and each means what it says: two tiers, and no peer. An entry that never moved has no journal file rather than an empty one

Verify: `cargo nextest run --test validation`

### `zone-layout:a-story-filename-is-its-id` — A story filename is its id

A story document's filename stem MUST equal the entry's id, checked in both directions.

#### Scenario: A document is renamed by hand

- GIVEN a story file renamed without the record
- WHEN validation runs
- THEN both directions fail, because an id with no document and a document with no id are two different faults

Verify: `cargo nextest run --test validation`

### `zone-layout:the-plan-trunk-is-one-branch` — The plan trunk is one branch

The plan repository MUST carry exactly one published branch, created by the tool.

#### Scenario: Someone branches the plan

- GIVEN a second branch pushed to the plan repository
- WHEN the guard runs
- THEN it is refused, because the record's whole conflict story assumes one line of history

Verify: `cargo nextest run --test journey`

### `zone-layout:plan-history-is-append-only` — Plan history is append-only

The implementation MUST NOT force-push the plan repository.

#### Scenario: A bad commit lands

- GIVEN a mutation the operator regrets
- WHEN they want it gone
- THEN a new commit reverses it, because rewriting the line other machines already replicated loses their work

Verify: `cargo nextest run --test journey`

### `zone-layout:the-shipped-payload-is-never-copied-in` — The shipped payload is never copied into a zone

The schemas, the scaffold payload, and the worked example MUST stay under the installed product's data directory and MUST NOT be copied into a zone.

#### Scenario: A project pins its own copy of a schema

- GIVEN a schema copied into the zone
- WHEN the product is upgraded
- THEN the copy is a second source of truth drifting from the version the gate runs

Verify: `cargo nextest run --test journey`

## Unenforced rules

| Rule                                        | Why no command decides it                                                           |
| ------------------------------------------- | ----------------------------------------------------------------------------------- |
| `zone-layout:derived-state-is-never-stored` | Whether a proposed field is derived or stated is a reading of what the field means. |

Hooks are early gates and are bypassable by construction, so correctness lives in the tool's transactions. Hosted branch protection backstops publication where the operator hosts a remote.
