# Slot Naming Specification

<!--TOC-->

- [Purpose](#purpose)
- [Deriving the project slug](#deriving-the-project-slug)
- [Slot states](#slot-states)
- [Open question](#open-question)
- [Requirements](#requirements)
  - [`slot-naming:a-slot-name-is-minted-once` — A slot name is minted once](#slot-naminga-slot-name-is-minted-once--a-slot-name-is-minted-once)
  - [`slot-naming:a-project-slug-follows-one-ladder` — A project slug follows one ladder](#slot-naminga-project-slug-follows-one-ladder--a-project-slug-follows-one-ladder)
  - [`slot-naming:a-derived-slug-decides-only-at-mint-time` — A derived slug decides only at mint time](#slot-naminga-derived-slug-decides-only-at-mint-time--a-derived-slug-decides-only-at-mint-time)
  - [`slot-naming:drift-is-a-healthy-state` — Drift is a healthy state](#slot-namingdrift-is-a-healthy-state--drift-is-a-healthy-state)
  - [`slot-naming:a-slot-name-is-recorded-only-as-the-directory` — A slot name is recorded only as the directory](#slot-naminga-slot-name-is-recorded-only-as-the-directory--a-slot-name-is-recorded-only-as-the-directory)
  - [`slot-naming:a-slot-name-never-replicates` — A slot name never replicates](#slot-naminga-slot-name-never-replicates--a-slot-name-never-replicates)
  - [`slot-naming:a-taken-name-escalates` — A taken name escalates](#slot-naminga-taken-name-escalates--a-taken-name-escalates)
  - [`slot-naming:an-empty-or-repeated-segment-is-skipped` — An empty or repeated segment is skipped](#slot-namingan-empty-or-repeated-segment-is-skipped--an-empty-or-repeated-segment-is-skipped)
  - [`slot-naming:a-minted-name-is-stable` — A minted name is stable](#slot-naminga-minted-name-is-stable--a-minted-name-is-stable)
  - [`slot-naming:a-slot-name-is-one-opaque-token` — A slot name is one opaque token](#slot-naminga-slot-name-is-one-opaque-token--a-slot-name-is-one-opaque-token)

<!--TOC-->

## Purpose

A slot name is a slug that a machine gives one plan. It is minted once from the project slug, escalated until free, and stored only in the directory name.

A slot name does not replicate. No other machine reads it, and no file records it.

A slot name is not an alias. A peer alias and a source alias are declared in committed files, but a slot name is declared nowhere.

The name has three properties:

```text
local        no other machine reads it, and no file records it
minted once  it tracks no value, so no change makes it stale
free-able    the escalation ladder ends with a step that cannot fail
```

## Deriving the project slug

A project slug is the readable name derived for the project. The first step that answers supplies it:

```text
1. the main worktree's forge project name
2. the main worktree's directory name
3. the directory holding .wipctl/project.toml
```

Git identifies the main worktree. The working directory does not supply the slug because several worktrees for one project can have different directory names.

A host project does not need to be a git repository. The project file marks its root by its presence, so the third step supplies a slug without git.

A peer has no host worktree. Its first url supplies the forge project name and namespace before the same escalation applies.

The slot name becomes free by postfix escalation:

```text
1. <slug>
2. + <parent-scope>
3. + <username>
4. + <forge>
5. + 01, 02, and the first free sequential number
```

At mint time, the implementation derives the project slug, escalates it, and freezes the result as the slot name. At every read, it derives the slug again for display and drift detection.

A derived slug never selects a path, resolves a record, or compares with a committed value. It decides only the first slot name candidate at mint time.

## Slot states

```text
matching  the slot name and the derived slug agree
drifted   they disagree; the record is intact and every verb works
absent    the project file names a project that no slot holds
```

Drift is legal. It does not fail a check or a verb.

Detection does not repair drift. A rename needs both slot-name locks, changes the registry, and moves machine-local state. An ordinary reader must not become that writer.

The explicit repair is `wipctl fix --registry [<name>]`, as the repair domain specifies.

The scaffold stays create-only. Every other write starts with a verb that a person ran.

## Open question

Which declared remote supplies the forge name and namespace remains open. The design does not select `origin` by default. It also does not choose between an explicit configuration key and skipping the first step where several remotes answer.

## Requirements

### `slot-naming:a-slot-name-is-minted-once` — A slot name is minted once

When a plan first enters a machine's registry, the implementation MUST mint its slot name once and keep that name until an explicit repair.

#### Scenario: A project changes its name

- GIVEN a plan in a slot whose project later has another name
- WHEN a verb resolves the plan
- THEN the slot keeps its minted name, because an ordinary read does not rename a directory

Verify: `cargo nextest run --test attachment`

### `slot-naming:an-empty-or-repeated-segment-is-skipped` — An empty or repeated segment is skipped

Where an escalation segment is absent or already present, the implementation MUST skip that segment and try the next step.

#### Scenario: A personal repository repeats its account name

- GIVEN a slug that already carries the forge account name
- WHEN the account step runs
- THEN the step adds nothing and escalation continues

Verify: `cargo nextest run --test attachment`

### `slot-naming:a-minted-name-is-stable` — A minted name is stable

Once a slot name is minted, the implementation MUST keep it stable until the slot repair runs.

#### Scenario: A forge project is renamed

- GIVEN a slot minted from the former project slug
- WHEN a reader derives the new slug
- THEN the slot name stays stable and the reader reports drift

Verify: `cargo nextest run --test attachment`

### `slot-naming:a-slot-name-is-one-opaque-token` — A slot name is one opaque token

The implementation MUST treat a slot name as one opaque token and read forge facts from their source.

#### Scenario: A report needs the forge namespace

- GIVEN a slot name that includes a namespace postfix
- WHEN the report renders the namespace
- THEN it reads the configured source instead of parsing the slot name

Verify: reviewer confirms no code splits a slot name to recover forge facts

### `slot-naming:a-project-slug-follows-one-ladder` — A project slug follows one ladder

When a project slug is needed, the implementation MUST use the first available source in the stated derivation ladder.

#### Scenario: A project has several worktrees

- GIVEN two linked worktrees with different directory names
- WHEN each derives the project slug
- THEN both use the main worktree and produce one answer

Verify: `cargo nextest run --test attachment`

### `slot-naming:a-derived-slug-decides-only-at-mint-time` — A derived slug decides only at mint time

When a slot is minted, the implementation MUST use the derived slug as its candidate and use later derivations only for display and drift detection.

#### Scenario: A forge project is renamed

- GIVEN a slot whose name matches the old project slug
- WHEN a later read derives the new slug
- THEN the existing slot still resolves and the reader reports the drift

Verify: `cargo nextest run --test attachment`

### `slot-naming:drift-is-a-healthy-state` — Drift is a healthy state

Where a slot name differs from its derived project slug, the implementation MUST report the difference and let every verb continue.

#### Scenario: A directory supplying the project slug is renamed

- GIVEN an attached project whose directory has another name
- WHEN the health report reads it
- THEN the report names both slugs, exits zero, and leaves the slot unchanged

Verify: `cargo nextest run --test verb_contracts`

### `slot-naming:a-slot-name-is-recorded-only-as-the-directory` — A slot name is recorded only as the directory

The implementation MUST record a slot name as the directory under the projects tree and in no file.

#### Scenario: A reader opens a slot

- GIVEN a directory holding a plan repository
- WHEN the reader looks for the slot name
- THEN the directory basename is the complete record of that local name

Verify: `cargo nextest run --test zone_layout`

### `slot-naming:a-slot-name-never-replicates` — A slot name never replicates

The implementation MUST keep each slot name local to the machine that minted it.

#### Scenario: A plan is attached on two machines

- GIVEN two machines with different occupied names
- WHEN each machine attaches the same plan
- THEN each machine can mint a different slot name without changing the plan repository

Verify: `cargo nextest run --test attachment`

### `slot-naming:a-taken-name-escalates` — A taken name escalates

When a candidate slot name is taken, the implementation MUST apply the postfix escalation until it finds a free name.

#### Scenario: A peer derives a name already in use

- GIVEN a machine whose `payments` slot holds another plan
- WHEN the peer attachment walk needs a slot for a second payments plan
- THEN the walk tries the next candidate instead of refusing the plan

Verify: `cargo nextest run --test attachment`
