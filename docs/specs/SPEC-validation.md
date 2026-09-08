# Validation Specification

<!--TOC-->

- [Purpose](#purpose)
- [The three diagnostic classes](#the-three-diagnostic-classes)
- [Requirements](#requirements)
  - [`validation:every-check-has-one-owner` — Every check has one owner](#validationevery-check-has-one-owner--every-check-has-one-owner)
  - [`validation:the-checker-half-is-required` — The checker half is required](#validationthe-checker-half-is-required--the-checker-half-is-required)
  - [`validation:the-checker-is-internal` — The checker is internal](#validationthe-checker-is-internal--the-checker-is-internal)
  - [`validation:a-skipped-half-is-always-named` — A skipped half is always named](#validationa-skipped-half-is-always-named--a-skipped-half-is-always-named)
  - [`validation:a-warning-never-reaches-the-exit-code` — A warning never reaches the exit code](#validationa-warning-never-reaches-the-exit-code--a-warning-never-reaches-the-exit-code)
  - [`validation:a-diagnostic-is-one-location-line` — A diagnostic is one location line](#validationa-diagnostic-is-one-location-line--a-diagnostic-is-one-location-line)
  - [`validation:there-is-no-diagnostic-code` — There is no diagnostic code](#validationthere-is-no-diagnostic-code--there-is-no-diagnostic-code)
  - [`validation:the-gate-never-repairs` — The gate never repairs](#validationthe-gate-never-repairs--the-gate-never-repairs)
  - [`validation:the-verb-takes-no-argument` — The validation verb takes no argument](#validationthe-verb-takes-no-argument--the-validation-verb-takes-no-argument)
  - [`validation:success-prints-the-census` — Success prints the census](#validationsuccess-prints-the-census--success-prints-the-census)
  - [`validation:a-broken-install-is-not-a-skip` — A broken install is not a skip](#validationa-broken-install-is-not-a-skip--a-broken-install-is-not-a-skip)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Is the record coherent, and which half answers. The split is architectural: it decides where a rule is written, which tool reports it, and what a hook wires. The boundary runs at the file. A shape schema owns one file's own structure, and the cross-file checker owns every fact spanning two files. What each rule says belongs to the domain that owns the file.

## The three diagnostic classes

| Class           | Meaning                                                                | Exit     |
| --------------- | ---------------------------------------------------------------------- | -------- |
| failure         | the record states something incorrectly and no tool may decide the fix | non-zero |
| ranking failure | an order contradicting the dependency graph                            | non-zero |
| warning         | a fact the reader decides about                                        | zero     |

The ranking class is separate because it is the one class the repair can resolve.

## Requirements

### `validation:every-check-has-one-owner` — Every check has one owner

Every check MUST have exactly one owner: the shape schema for one file's own structure, and the cross-file checker for every fact spanning two files.

#### Scenario: A rule could be written in either half

- GIVEN a constraint expressible as a single-file conditional
- WHEN it is placed
- THEN the schema owns it, because a rule written twice is reported twice and drifts once

Verify: `cargo nextest run --test validation`

### `validation:the-checker-half-is-required` — The checker half is required

The cross-file checker MUST be present, and its absence MUST be a failure rather than a skip.

#### Scenario: An install lacks its checker

- GIVEN a build whose checker is missing
- WHEN validation runs
- THEN it fails saying the record was not validated, because the half holding every cross-file rule cannot be optional

Verify: `cargo nextest run --test validation`

### `validation:the-checker-is-internal` — The checker is internal

The cross-file checker MUST be reachable only through validation, the writers' preflights, reconciliation, and the scaffold's self-check, and MUST NOT be a separately supported command.

#### Scenario: A caller wants the checker alone

- GIVEN a request for a standalone checker command
- WHEN it is weighed
- THEN it is refused, because a second entry point is a second contract to keep true

Verify: `cargo nextest run --test verb_contracts`

### `validation:a-skipped-half-is-always-named` — A skipped half is always named

The implementation MUST name the schema half as run or skipped on every validation run.

#### Scenario: No instance checker is installed

- GIVEN a machine without the optional checker
- WHEN validation runs
- THEN the skip is named with its reason. A check that did not run, reported as green, is the one failure mode a gate must not have

Verify: `cargo nextest run --test validation`

### `validation:a-warning-never-reaches-the-exit-code` — A warning never reaches the exit code

A warning MUST reach the reader and MUST NOT reach the exit code.

#### Scenario: A close date is out of sequence

- GIVEN a record whose only fault is an out-of-order close
- WHEN validation runs
- THEN it succeeds and reports the warning, because the reader decides whether to run the repair

Verify: `cargo nextest run --test validation`

### `validation:a-diagnostic-is-one-location-line` — A diagnostic is one location line

Every diagnostic MUST be one location-and-message line on the diagnostic stream, using a real filesystem path and a line number where one is known.

#### Scenario: A reader jumps to a fault

- GIVEN a diagnostic naming a lane file and a line
- WHEN their editor opens it
- THEN the path resolves, because a synthetic location is a path nobody's tooling can follow

Verify: `cargo nextest run --test validation`

### `validation:there-is-no-diagnostic-code` — There is no diagnostic code

The implementation MUST NOT define a diagnostic code namespace.

#### Scenario: A caller wants to match on a code

- GIVEN a request for stable numeric codes
- WHEN it is weighed
- THEN it is refused, because the message is the diagnostic and a code is a second name for it that drifts

Verify: `cargo nextest run --test validation`

### `validation:the-gate-never-repairs` — The gate never repairs

Validation MUST stay read-only, and a hook MUST NOT invoke a writer.

#### Scenario: A hook is wired to the repair

- GIVEN a commit gate that repairs what it finds
- WHEN a broken record is committed
- THEN the gate cannot fail, and a gate that cannot fail proves nothing, so repair stays a person's explicit act

Verify: `cargo nextest run --test validation`

### `validation:the-verb-takes-no-argument` — The validation verb takes no argument

The validation verb MUST take no argument and no flag, and MUST treat any as a usage error.

#### Scenario: A caller passes a path

- GIVEN an invocation naming a lane file
- WHEN it parses
- THEN it is a usage error, because every rule is cross-file and a partial run answers a different question

Verify: `cargo nextest run --test verb_contracts`

### `validation:success-prints-the-census` — Success prints the census

On success the implementation MUST print the census and the schema report, with counts correct for number.

#### Scenario: A record holds exactly one epic

- GIVEN a census line naming the counts
- WHEN it prints
- THEN the singular form is used, because a count that reads wrong makes a reader doubt the number

Verify: `cargo nextest run --test verb_contracts`

### `validation:a-broken-install-is-not-a-skip` — A broken install is not a skip

Where the shipped schema directory is missing, the implementation MUST fail rather than skip.

#### Scenario: The schemas are absent from the install

- GIVEN a build with no schema directory
- WHEN validation runs
- THEN it fails as a broken install, because a missing optional tool and a missing shipped artifact are different faults

Verify: `cargo nextest run --test validation`

## Unenforced rules

| Rule                                   | Why no command decides it                                         |
| -------------------------------------- | ----------------------------------------------------------------- |
| `validation:every-check-has-one-owner` | Whether a proposed rule spans two files is a reading of the rule. |

Four instance schemas gate what a project writes, and seven output schemas gate what verbs emit. A verb relaying a record fact in its own voice carries the command prefix, which is the ordinary stream discipline.
