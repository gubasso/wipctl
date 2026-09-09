# Command Surface Specification

<!--TOC-->

- [Purpose](#purpose)
- [Exit codes](#exit-codes)
- [The verb surface](#the-verb-surface)
- [Requirements](#requirements)
  - [`cli-conventions:the-exit-code-is-an-interface` — The exit code is an interface](#cli-conventionsthe-exit-code-is-an-interface--the-exit-code-is-an-interface)
  - [`cli-conventions:stdout-carries-data-alone` — Stdout carries data alone](#cli-conventionsstdout-carries-data-alone--stdout-carries-data-alone)
  - [`cli-conventions:a-diagnostic-carries-a-prefix` — A diagnostic carries a prefix](#cli-conventionsa-diagnostic-carries-a-prefix--a-diagnostic-carries-a-prefix)
  - [`cli-conventions:a-consumer-path-is-absolute` — A path printed for a consumer is absolute](#cli-conventionsa-consumer-path-is-absolute--a-path-printed-for-a-consumer-is-absolute)
  - [`cli-conventions:the-verb-names-are-fixed` — The verb names are fixed](#cli-conventionsthe-verb-names-are-fixed--the-verb-names-are-fixed)
  - [`cli-conventions:the-verb-list-is-derived` — The verb list is derived from the build](#cli-conventionsthe-verb-list-is-derived--the-verb-list-is-derived-from-the-build)
  - [`cli-conventions:global-flags-parse-before-the-verb` — Global flags parse before the verb](#cli-conventionsglobal-flags-parse-before-the-verb--global-flags-parse-before-the-verb)
  - [`cli-conventions:there-is-no-zone-argument` — There is no zone argument](#cli-conventionsthere-is-no-zone-argument--there-is-no-zone-argument)
  - [`cli-conventions:a-verb-acts-on-one-plan` — A verb acts on one plan](#cli-conventionsa-verb-acts-on-one-plan--a-verb-acts-on-one-plan)
  - [`cli-conventions:a-machine-format-is-verb-local` — A machine format is verb-local and owes a schema](#cli-conventionsa-machine-format-is-verb-local--a-machine-format-is-verb-local-and-owes-a-schema)
  - [`cli-conventions:an-optional-dependency-never-gates-output` — An optional dependency never gates output](#cli-conventionsan-optional-dependency-never-gates-output--an-optional-dependency-never-gates-output)
  - [`cli-conventions:a-skipped-check-is-named` — A skipped check is named](#cli-conventionsa-skipped-check-is-named--a-skipped-check-is-named)
  - [`cli-conventions:help-is-the-authority-on-the-build` — Help is the authority on the build](#cli-conventionshelp-is-the-authority-on-the-build--help-is-the-authority-on-the-build)
  - [`cli-conventions:an-absent-verb-is-declared` — An absent verb is declared](#cli-conventionsan-absent-verb-is-declared--an-absent-verb-is-declared)
  - [`cli-conventions:the-version-is-one-bare-string` — The version is one bare string](#cli-conventionsthe-version-is-one-bare-string--the-version-is-one-bare-string)
  - [`cli-conventions:completion-derives-its-candidates` — Completion derives its candidates](#cli-conventionscompletion-derives-its-candidates--completion-derives-its-candidates)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

One command with capabilities as verbs, and the rules that bind every one of them. The boundary runs at the surface: this domain covers exit codes, streams, dispatch, machine formats, and the three verbs that resolve nothing. What a message must say belongs to the messages domain, and what a writing verb must hold belongs to the transactions domain. What the manual holds and where it comes from belong to `SPEC-manual.md`.

## Exit codes

```text
0  the command did what was asked
1  a check the command performs did not hold
2  the invocation itself was wrong
3  the writer lock could not be acquired within the bounded wait
```

## The verb surface

The command is `wipctl`. These are its verbs and their usage. A verb's own domain states what it does. This table states what it is called and what it accepts.

```text
wipctl [--plan <alias>] <verb> [...]

wipctl init [--root DIR] [--window-start YYYY-MM-DD] [--print-hooks] [--dry-run]
wipctl attach <plan-repo-url>
wipctl attach --create
wipctl attach --mint-uid
wipctl attach --peers
wipctl new <type> "<title>" [--id <id>] [--epic <id>] [--points <n>] [--lane <backlog|todo>]
wipctl land [--report]
wipctl move <id> --to <lane> [--outcome <o>] [--succeeded-by <id>] [--closed <date>] [--dry-run]
wipctl start
wipctl delete <id> [--dry-run]
wipctl rename <id> "<new title>" [--id <new-id>]
wipctl rename <id> --id <new-id>
wipctl fix [--slots]
wipctl sync [--all] [--json]
wipctl resolve <id> --keep <here|remote>
wipctl validate
wipctl doctor
wipctl next
wipctl ids
wipctl board
wipctl dashboard
wipctl graph [--unblocks <id>]
wipctl flow [--lane <lane>]
wipctl stale
wipctl velocity
wipctl epic [--json] [--write] [<epic-id>]
wipctl epics [--json]
wipctl initiative [--json] [<initiative-id>]
wipctl initiatives [--json]
wipctl peer add <url> [--as <alias>]
wipctl peer alias <alias> <new-alias>
wipctl peer url <alias> (--add <url> | --remove <url> | --set <url>...)
wipctl peer remove <alias>
wipctl peers [--json]
wipctl man [<verb>]
wipctl help
wipctl version
```

Ten verbs write the record: `new`, `land`, `move`, `start`, `delete`, `rename`, `fix`, `sync`, `resolve`, and `peer`. Closing is not a verb of its own. It is `move --to closed` with an outcome.

`attach` takes exactly one of four forms: a plan location, `--create`, `--mint-uid`, or `--peers`. `peer` and `peers` are a writer and a reader one letter apart, as `epic` and `epics` already are. The operation of `peer` is its first positional argument, so the verb stays one dispatchable unit and the surface gains no subcommands.

## Requirements

### `cli-conventions:the-exit-code-is-an-interface` — The exit code is an interface

Every verb MUST use the four exit codes as stated, and a warning MUST NOT reach an exit code.

#### Scenario: A caller retries on a failed check

- GIVEN a hook that retries on exit 1
- WHEN the invocation is malformed instead
- THEN it exits 2 and the caller stops, because a retry loop on a typo never ends

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:stdout-carries-data-alone` — Stdout carries data alone

Every verb MUST send data to stdout and every diagnostic, log line, prompt, and progress message to stderr.

#### Scenario: A verb reports progress while its output is piped

- GIVEN a verb writing a report into a pipe
- WHEN it also reports progress
- THEN the progress goes to stderr, because the data stream stays parseable

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:a-diagnostic-carries-a-prefix` — A diagnostic carries a prefix

When a verb speaks about its own invocation or progress, the message MUST carry the command prefix, in its plain, warning, or error form.

#### Scenario: A validation diagnostic is emitted

- GIVEN a check reporting a fault in a record file
- WHEN the line is written
- THEN it is one unprefixed location-and-message line, which is the stated exception, and a single-value output prints bare with no banner

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:a-consumer-path-is-absolute` — A path printed for a consumer is absolute

A path printed for a consumer MUST be absolute, and a path in a mutation report MUST be relative to the plan repository root.

#### Scenario: A story path is printed for an editor to open

- GIVEN a verb naming a document the reader will open
- WHEN the path is printed
- THEN it is absolute, because the record no longer lives under the working directory

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:the-verb-names-are-fixed` — The verb names are fixed

The implementation MUST use the verb names and the usage grammar this specification states, for every capability it ships.

#### Scenario: An implementation renames a verb it finds clearer

- GIVEN an implementation that exposes capture as `capture` rather than `new`
- WHEN a guide, a script, or another implementation's user invokes it
- THEN the command is unknown, because the verb name is the contract and not an implementation choice

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:the-verb-list-is-derived` — The verb list is derived from the build

The implementation MUST derive the shipped verb list from its own dispatch units and MUST NOT hard-code it.

#### Scenario: A verb is added

- GIVEN a new dispatchable unit
- WHEN the listing renders
- THEN the verb appears by existing, and no build can claim a verb it lacks

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:global-flags-parse-before-the-verb` — Global flags parse before the verb

The implementation MUST answer `-h` and `--help` with help, `-V` and `--version` with the version, and MUST end option parsing at `--`.

#### Scenario: The command is run bare

- GIVEN an invocation with no verb
- WHEN it runs
- THEN help prints and the run succeeds

The implementation parses these tokens only before the verb. An unknown leading option and an unknown verb are both usage errors, and each names the usage command. `--plan` sits in the same group, and the plan targeting domain owns what it selects and which verbs refuse it.

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:there-is-no-zone-argument` — There is no zone argument

The implementation MUST NOT offer a positional zone argument, and a caller who wants another plan MUST name it with the plan flag.

#### Scenario: A caller wants a plan this one does not declare

- GIVEN an alias no row of this plan's peer table holds
- WHEN the flag names it
- THEN it is a usage error, because the flag selects from the closure this record already committed to and is not a path

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:a-verb-acts-on-one-plan` — A verb acts on one plan

A verb MUST act on exactly one plan, and MUST reach more than one only under a batch flag.

#### Scenario: A check reads two attached peers to answer about this plan

- GIVEN a record whose dependencies reach two peers
- WHEN validation runs
- THEN it is still single-plan, because it answers about this plan. That one plan is the one the working directory resolves to, unless the invocation names another through the plan targeting domain's flag

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:a-machine-format-is-verb-local` — A machine format is verb-local and owes a schema

Where a verb offers a machine format, the implementation MUST ship a schema for that shape and a case validating real output against it.

#### Scenario: A verb's output is one token per line

- GIVEN a verb whose output is already parseable
- WHEN a machine format is proposed
- THEN it offers none, as drawings do, because there is no structure to lose

Verify: `cargo nextest run --test schemas`

### `cli-conventions:an-optional-dependency-never-gates-output` — An optional dependency never gates output

An optional tool MUST improve output, MUST NOT gate it, and MUST carry a manifest entry and a probe at verb entry.

#### Scenario: An optional tool is absent mid-render

- GIVEN a view that draws on an optional tool
- WHEN the tool is missing
- THEN the probe at entry chose the fallback already, because probing mid-render splits one view across two shapes

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:a-skipped-check-is-named` — A skipped check is named

When a check is skipped, the implementation MUST name the skip and its reason on stderr.

#### Scenario: No instance checker is on the path

- GIVEN a validation run without the optional checker
- WHEN the schema class is skipped
- THEN the skip is named, because a silently green class is indistinguishable from a passing one

Verify: `cargo nextest run --test validation`

### `cli-conventions:help-is-the-authority-on-the-build` — Help is the authority on the build

The usage output MUST list every shipped verb, one indented verb per line, sorted, and MUST close with the exit-code table and the documentation location.

#### Scenario: Completion parses the command block

- GIVEN shell completion reading the usage output
- WHEN the block's shape changes
- THEN completion breaks, so the one-verb-per-line shape is part of the contract

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:an-absent-verb-is-declared` — An absent verb is declared

Where a build lacks a verb the method defines, the usage output MUST list it under its own heading and name it as planned work.

#### Scenario: A partial build is used mid-project

- GIVEN a build missing three verbs
- WHEN a reader runs the usage output
- THEN the three are named as planned work in the project's own record, not as missing features

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:the-version-is-one-bare-string` — The version is one bare string

The version verb MUST print the version string alone, MUST reject any argument, and MUST fail when its single source of truth is missing.

#### Scenario: A script captures the version

- GIVEN a script reading the version output
- WHEN it captures the line
- THEN no parsing is needed, and a missing source is reported as a broken install rather than as an empty string

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:completion-derives-its-candidates` — Completion derives its candidates

Completion MUST derive its candidates at completion time, from the usage output, the id verb, the peer view, and the five lane names.

#### Scenario: The id source fails

- GIVEN a project that is not attached
- WHEN completion runs
- THEN it offers nothing, because falling back to filenames offers candidates that are not ids

Verify: `cargo nextest run --test completion`

## Unenforced rules

| Rule                                          | Why no command decides it                                                                                        |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `cli-conventions:a-consumer-path-is-absolute` | Whether a printed path is for a consumer to open or part of a mutation report is a reading of the verb's intent. |
