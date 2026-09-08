# Validation Specification

<!--TOC-->

- [Purpose](#purpose)
- [The three diagnostic classes](#the-three-diagnostic-classes)
- [The closure the checker reads](#the-closure-the-checker-reads)
- [Requirements](#requirements)
  - [`validation:every-check-has-one-owner` — Every check has one owner](#validationevery-check-has-one-owner--every-check-has-one-owner)
  - [`validation:the-read-set-is-attached-or-the-run-fails` — The read set is attached, or the run fails](#validationthe-read-set-is-attached-or-the-run-fails--the-read-set-is-attached-or-the-run-fails)
  - [`validation:every-plan-read-is-checked-in-full` — Every plan read is checked in full](#validationevery-plan-read-is-checked-in-full--every-plan-read-is-checked-in-full)
  - [`validation:a-peer-is-read-at-its-current-commit` — A peer is read at its current commit](#validationa-peer-is-read-at-its-current-commit--a-peer-is-read-at-its-current-commit)
  - [`validation:a-prefixed-dependency-resolves-or-fails` — A prefixed dependency resolves, or the run fails](#validationa-prefixed-dependency-resolves-or-fails--a-prefixed-dependency-resolves-or-the-run-fails)
  - [`validation:an-unreferenced-peer-row-is-stale` — An unreferenced peer row is stale](#validationan-unreferenced-peer-row-is-stale--an-unreferenced-peer-row-is-stale)
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
- [Diagnostics](#diagnostics)

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

## The closure the checker reads

Every check in every domain applies to every plan the invocation reads: the plan the working directory resolves to, and every attached peer in its closure. A failure names the plan it was found in.

One precondition holds before any of that runs. Every peer in the read set is attached on this machine. The read set is the peers a prefixed dependency reaches, transitively, and the peers domain names it beside the declared set. A peer in the read set with no slot is a failed check naming the alias and the attach form that fills it.

Two consequences follow, and each is a thing a reader otherwise finds the hard way. Adding one prefixed dependency can make a run start failing on a machine where it passed a minute ago. The new id moved a peer into the read set. Attachment is also transitive through checking, not only through the walk. Reading a peer means checking it, so that peer's read set joins this one.

The checker reads the files of this plan repository and the committed trees of every peer attached on this machine. It reads nothing else, and it opens no network connection. Its answer is a function of those trees at the moment of the read, so the census names the revision it read each peer at. The acyclicity proof is complete over that closure, which is why the closure is complete before the proof runs.

The refusal is the whole design. A check that runs on one machine and quietly does not run on another is worse than one that fails loudly. The first teaches a reader that a green result means something it does not.

## Requirements

### `validation:every-check-has-one-owner` — Every check has one owner

Every check MUST have exactly one owner: the shape schema for one file's own structure, and the cross-file checker for every fact spanning two files.

#### Scenario: A rule could be written in either half

- GIVEN a constraint expressible as a single-file conditional
- WHEN it is placed
- THEN the schema owns it, because a rule written twice is reported twice and drifts once

Verify: `cargo nextest run --test validation`

### `validation:the-read-set-is-attached-or-the-run-fails` — The read set is attached, or the run fails

Where a peer in the read set has no slot, or its slot declares another uid, validation MUST fail, naming the alias.

#### Scenario: A machine has attached less than the closure

- GIVEN a record whose dependencies reach two peers, one of them unattached
- WHEN validation runs
- THEN it fails and names that one peer. A check demands what it reads and no more, and a skipped peer reads as a passing one

Verify: `cargo nextest run --test validation`

### `validation:every-plan-read-is-checked-in-full` — Every plan read is checked in full

Every check MUST apply to every plan the invocation reads, and a failure MUST name the plan it was found in.

#### Scenario: Another team's record is broken

- GIVEN a peer whose own record fails a check
- WHEN this project validates
- THEN it fails, naming the peer, the slot, and the failing check. There is one idea of a plan and a peer is an instance of it, so no peer gets a reduced mode

Verify: `cargo nextest run --test validation`

### `validation:a-peer-is-read-at-its-current-commit` — A peer is read at its current commit

A peer MUST be read at the tree of its current commit, never at its working tree.

#### Scenario: The peer's own writer is mid-transaction

- GIVEN a peer whose slot is being written while this project validates
- WHEN the read happens
- THEN it sees committed state alone, so no reader blocks that writer and no reader sees half a transaction

Verify: `cargo nextest run --test validation`

### `validation:a-prefixed-dependency-resolves-or-fails` — A prefixed dependency resolves, or the run fails

A prefixed dependency MUST name a declared alias, and MUST resolve to a live entry or to a tombstone in that peer.

#### Scenario: The target was renamed in the peer

- GIVEN a dependency naming an entry that peer renamed last month
- WHEN validation runs
- THEN the tombstone is read and the successor is named, because the answer is in the peer's own record and a generic guess is not

Verify: `cargo nextest run --test validation`

### `validation:an-unreferenced-peer-row-is-stale` — An unreferenced peer row is stale

Where no dependency uses a declared alias, validation MUST report that row as stale and MUST NOT fail on it.

#### Scenario: A peer is declared before the work that needs it

- GIVEN a row nothing references, and no slot for it
- WHEN validation runs
- THEN it passes and reports the row, because an unused row breaks nothing and misleads a reader, which is what stale means here

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

The validation verb MUST take no argument and no verb-local flag, and MUST treat any as a usage error.

#### Scenario: A caller passes a path

- GIVEN an invocation naming a lane file
- WHEN it parses
- THEN it is a usage error, because every rule is cross-file and a partial run answers a different question. A global flag parsed before the verb is not a verb-local flag, so naming a plan to run against is not one either

Verify: `cargo nextest run --test verb_contracts`

### `validation:success-prints-the-census` — Success prints the census

On success the implementation MUST print the census, the schema report, and each peer's revision, with counts correct for number.

#### Scenario: Two machines disagree about one record

- GIVEN two runs whose answers differ
- WHEN each census names the revision it read each peer at
- THEN the disagreement is one line to diagnose. A record naming no peer reports zero peers rather than dropping the counter. A counter that disappears reads as a check that did not run

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

Five instance schemas gate what a project writes, and ten output schemas gate what verbs emit. A verb relaying a record fact in its own voice carries the command prefix, which is the ordinary stream discipline.

## Diagnostics

One resolver turns a prefixed dependency into an entry, and these are its messages. No verb writes a second, softer message for one of these conditions. Two messages for one condition teach a reader that the tool has two opinions.

```text
the target was deleted in the peer                                        exit 1
  wipctl: needs payments#secure-session-storage: that entry was deleted
          in the payments plan on 2026-08-30
  wipctl: the id is burned there for the life of that record; drop the
          dependency, or ask its owners what replaced the work

the target was renamed in the peer                                        exit 1
  wipctl: needs payments#secure-session-storage: that entry was renamed
          in the payments plan on 2026-08-30
  wipctl:   it is now  payments#session-identity-storage
  wipctl: update the reference on profile-composition

the target was never there                                                exit 1
  wipctl: needs payments#secure-session-storage: no such entry and no
          tombstone in the payments plan at 91a47c0d
  wipctl: check the id, or the slot may be behind; run
          'wipctl sync --all' and try again

a peer in the read set is not attached                                    exit 1
  wipctl: peer 'payments' is not attached on this machine
  wipctl: run 'wipctl attach --peers' to clone every plan this one names

a declared row nothing references                                         exit 0
  wipctl: stale: no entry needs anything from peer 'platform'
  wipctl: drop the row with 'wipctl peer remove platform', or leave it
          for the work that will use it
```

Three answers replace one guess. The third names the revision, because at that point the slot being behind is the likeliest explanation left.
