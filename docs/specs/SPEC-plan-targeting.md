# Plan Targeting Specification

<!--TOC-->

- [Purpose](#purpose)
- [The flag](#the-flag)
- [The third resolution path](#the-third-resolution-path)
- [The dispatch matrix](#the-dispatch-matrix)
- [Parity, and what it costs](#parity-and-what-it-costs)
- [What it does not change](#what-it-does-not-change)
- [Requirements](#requirements)
  - [`plan-targeting:the-flag-selects-from-this-plans-table` — The flag selects from this plan's table](#plan-targetingthe-flag-selects-from-this-plans-table--the-flag-selects-from-this-plans-table)
  - [`plan-targeting:a-verb-that-resolves-nothing-refuses-it` — A verb that resolves nothing refuses the flag](#plan-targetinga-verb-that-resolves-nothing-refuses-it--a-verb-that-resolves-nothing-refuses-the-flag)
  - [`plan-targeting:the-dispatch-matrix-is-derived` — The dispatch matrix is derived](#plan-targetingthe-dispatch-matrix-is-derived--the-dispatch-matrix-is-derived)
  - [`plan-targeting:the-target-owns-the-lock-and-the-commit` — The target owns the lock and the commit](#plan-targetingthe-target-owns-the-lock-and-the-commit--the-target-owns-the-lock-and-the-commit)
  - [`plan-targeting:the-flag-names-one-plan` — The flag names one plan](#plan-targetingthe-flag-names-one-plan--the-flag-names-one-plan)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

Which plan a verb acts on, when it is not the one underfoot. The boundary runs at the selection. This domain owns the flag, the verbs it binds, and the resolution path it adds. The attachment domain owns the ordinary two-step resolution, and the peers domain owns the table the flag reads.

## The flag

```text
wipctl [--plan <alias>] <verb> [...]
```

`--plan` is a global flag, parsed before the verb, in the group the command surface already defines. It takes an alias from this plan's own peer table. Given one, the verb acts on the plan that alias names, and every other rule of that verb is unchanged.

It reaches one hop, and the page says so plainly. An alias is local to the plan that declares it, so an alias means nothing outside the file that holds it. If `alpha` declares `beta` and `beta` declares `gamma`, then `gamma` has no name in `alpha`. Running `wipctl --plan gamma <verb>` from `alpha` is a usage error, because the table has no such row.

The closure is what the checker proves over. The flag selects from one table. A verb that must act on `gamma` is run from a shell inside it. The other route is for `alpha` to declare it and give it a name of its own.

## The third resolution path

```text
no --plan     walk upward to .wipctl/project.toml, read project_id,
              and find the slot whose plan file declares it

--plan <a>    resolve this project's plan as above, read its peers
              section, resolve <a> to plan_id, and find the slot whose
              plan file declares that identity
```

The second path still starts with the first, because the table that binds the alias lives in this plan. A caller outside any project has no table to read, so the flag there is the ordinary no-configuration usage error.

## The dispatch matrix

`init` never walks, and the manual, the usage output, and the version resolve nothing. A flag that redirects resolution has nothing to say to such a verb, so the page states the matrix rather than the word every.

```text
takes --plan     every verb that resolves a record: the readers, the ten
                 writers, peer, peers, doctor, and validate

refuses --plan   init, man, help, version. Passing it is a usage error,
                 naming the verb and the reason, because the invocation
                 asks for a redirection the verb has nowhere to apply
```

`init` is the one worth a sentence. It creates a plan for the project the caller is inside, so `wipctl --plan payments init` reads as an instruction to create a plan inside a plan. That is not a thing, and refusing it is better than choosing a meaning for it.

The matrix is derived the same way the verb list is, from the build. A verb added later joins one side of it by existing rather than by being remembered.

## Parity, and what it costs

A peer is a plan, so every verb that resolves a record accepts the flag, writers included. `wipctl --plan payments board` reads that plan's board. `wipctl --plan payments move <id> doing` moves an entry in that plan, holds that plan's writer lock, and commits to that plan's trunk.

Write access is the operator's business. The tool refuses nothing on the basis of who owns the peer, because it has no permission model and no pillar allows one. A push the forge rejects is reported by replication in the relayed-transport class, which is where a permission failure surfaces.

The cost is worth stating. A verb that writes can now write into another team's record from a shell that happens to be in this project. That is what parity means. The mitigation is that the alias is already in a committed table, not that the tool second-guesses the operator.

## What it does not change

- Resolution for a bare invocation. Without the flag, everything resolves as it did.
- Whether a verb is multi-plan. The flag selects one plan, so the batch mode stays the one the replication domain defines.
- How far a name reaches. The flag resolves through this plan's own table, so it can only name a plan this record declared itself.
- Whether a plan exists. An alias whose slot is empty is the ordinary unattached-peer failure, naming the walk that fills it.

## Requirements

### `plan-targeting:the-flag-selects-from-this-plans-table` — The flag selects from this plan's table

The flag MUST resolve its alias through the invoking plan's own peer table, and MUST reject an alias that table does not declare.

#### Scenario: An alias only a peer declares

- GIVEN this plan declaring `beta`, and `beta` declaring `gamma`
- WHEN the flag names `gamma`
- THEN it is a usage error naming the plan that does declare it, because an alias is local and the flag is not a path

Verify: `cargo nextest run --test verb_contracts`

### `plan-targeting:a-verb-that-resolves-nothing-refuses-it` — A verb that resolves nothing refuses the flag

Where a verb resolves no record, the flag MUST be a usage error naming the verb and the reason.

#### Scenario: The flag is passed to the scaffold

- GIVEN an invocation naming a peer and the scaffold verb
- WHEN it parses
- THEN it is refused, because creating a plan inside a plan is not a thing, and refusing beats choosing a meaning for it

Verify: `cargo nextest run --test verb_contracts`

### `plan-targeting:the-dispatch-matrix-is-derived` — The dispatch matrix is derived

The implementation MUST derive which verbs take the flag from its own dispatch units, and MUST NOT hard-code the matrix.

#### Scenario: A verb is added

- GIVEN a new dispatchable unit that resolves a record
- WHEN the flag is passed to it
- THEN it is accepted by existing, because a remembered list is the copy that goes stale

Verify: `cargo nextest run --test verb_contracts`

### `plan-targeting:the-target-owns-the-lock-and-the-commit` — The target owns the lock and the commit

A verb under the flag MUST take the target plan's writer lock and MUST commit to the target plan's trunk.

#### Scenario: A writer is pointed at a peer

- GIVEN an entry moved in a peer from a shell inside this project
- WHEN the transaction runs
- THEN nothing about it changes except which plan it runs against, because a peer is a plan and not a lesser thing

Verify: `cargo nextest run --test writer_guarantees`

### `plan-targeting:the-flag-names-one-plan` — The flag names one plan

The flag MUST name one plan, and combining it with a batch flag MUST be a usage error.

#### Scenario: Both flags are given

- GIVEN one invocation carrying both
- WHEN it parses
- THEN it is refused, because one flag names a target and the other names a batch, so together they say nothing

Verify: `cargo nextest run --test verb_contracts`

## Diagnostics

```text
an alias the table does not declare                                       exit 2
  wipctl: unknown plan 'payments'
  wipctl: --plan selects a peer this plan declares; run 'wipctl peers'
          to see the aliases, or add it with 'wipctl peer add <url>'

an alias a peer declares, but this plan does not                          exit 2
  wipctl: unknown plan 'gamma'
  wipctl: 'gamma' is declared by peer 'beta', not by this plan. An alias
          is local to the plan that declares it, so --plan reads this
          plan's peers section only. Declare it here with 'wipctl peer
          add <url>', or run the verb from inside that plan.

--plan on a verb that resolves nothing                                    exit 2
  wipctl: 'init' does not take --plan
  wipctl: init creates the plan for the project you are in, so there is
          nothing for --plan to redirect; run it from that project

--plan with --all                                                         exit 2
  wipctl: --plan and --all cannot be combined
  wipctl: --plan names one plan and --all names every plan in the
          closure; choose one

the named plan is not attached                                            exit 1
  wipctl: plan 'payments' is declared but not attached on this machine
  wipctl: run 'wipctl attach --peers' to fill its slot
```
