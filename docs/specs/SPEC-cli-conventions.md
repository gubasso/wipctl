# Command Surface Specification

<!--TOC-->

- [Purpose](#purpose)
- [Exit codes](#exit-codes)
- [Requirements](#requirements)
  - [`cli-conventions:the-exit-code-is-an-interface` — The exit code is an interface](#cli-conventionsthe-exit-code-is-an-interface--the-exit-code-is-an-interface)
  - [`cli-conventions:stdout-carries-data-alone` — Stdout carries data alone](#cli-conventionsstdout-carries-data-alone--stdout-carries-data-alone)
  - [`cli-conventions:a-diagnostic-carries-a-prefix` — A diagnostic carries a prefix](#cli-conventionsa-diagnostic-carries-a-prefix--a-diagnostic-carries-a-prefix)
  - [`cli-conventions:a-consumer-path-is-absolute` — A path printed for a consumer is absolute](#cli-conventionsa-consumer-path-is-absolute--a-path-printed-for-a-consumer-is-absolute)
  - [`cli-conventions:the-verb-list-is-derived` — The verb list is derived from the build](#cli-conventionsthe-verb-list-is-derived--the-verb-list-is-derived-from-the-build)
  - [`cli-conventions:global-flags-parse-before-the-verb` — Global flags parse before the verb](#cli-conventionsglobal-flags-parse-before-the-verb--global-flags-parse-before-the-verb)
  - [`cli-conventions:there-is-no-zone-argument` — There is no zone argument](#cli-conventionsthere-is-no-zone-argument--there-is-no-zone-argument)
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

One command with capabilities as verbs, and the rules that bind every one of them. The boundary runs at the surface: this domain covers exit codes, streams, dispatch, machine formats, and the two verbs that resolve nothing. What a message must say belongs to the messages domain, and what a writing verb must hold belongs to the transactions domain.

## Exit codes

```text
0  the command did what was asked
1  a check the command performs did not hold
2  the invocation itself was wrong
3  the writer lock could not be acquired within the bounded wait
```

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

### `cli-conventions:the-verb-list-is-derived` — The verb list is derived from the build

The implementation MUST derive the shipped verb list from its own dispatch units and MUST NOT hard-code it.

#### Scenario: A verb is added

- GIVEN a new dispatchable unit
- WHEN the listing renders
- THEN the verb appears by existing, and no build can claim a verb it lacks

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:global-flags-parse-before-the-verb` — Global flags parse before the verb

The implementation MUST parse a global flag only before the verb, and MUST treat any other leading option or unknown verb as a usage error.

#### Scenario: The command is run bare

- GIVEN an invocation with no verb
- WHEN it runs
- THEN help prints and the run succeeds, while an unknown leading option names the usage command and exits 2

Verify: `cargo nextest run --test verb_contracts`

### `cli-conventions:there-is-no-zone-argument` — There is no zone argument

The implementation MUST NOT offer a positional zone argument or a project flag.

#### Scenario: A caller wants another project

- GIVEN a caller outside the project they mean
- WHEN they want to act on it
- THEN they change directory, because resolution answers from the working directory and a second answer contradicts it

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

Completion MUST derive its candidates at completion time, from the usage output, the id verb, and the five lane names.

#### Scenario: The id source fails

- GIVEN a project that is not attached
- WHEN completion runs
- THEN it offers nothing, because falling back to filenames offers candidates that are not ids

Verify: `cargo nextest run --test completion`

## Unenforced rules

| Rule                                          | Why no command decides it                                                                                        |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `cli-conventions:a-consumer-path-is-absolute` | Whether a printed path is for a consumer to open or part of a mutation report is a reading of the verb's intent. |
