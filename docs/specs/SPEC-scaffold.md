# Scaffold Specification

<!--TOC-->

- [Purpose](#purpose)
- [Payload inventory](#payload-inventory)
- [Requirements](#requirements)
  - [`scaffold:the-host-receives-one-file` — The host receives one file](#scaffoldthe-host-receives-one-file--the-host-receives-one-file)
  - [`scaffold:the-payload-names-no-documentation-method` — The payload names no documentation method](#scaffoldthe-payload-names-no-documentation-method--the-payload-names-no-documentation-method)
  - [`scaffold:the-scaffold-writes-only-what-it-creates` — The scaffold writes only what it creates](#scaffoldthe-scaffold-writes-only-what-it-creates--the-scaffold-writes-only-what-it-creates)
  - [`scaffold:the-payload-is-discovered` — The payload is discovered, never enumerated](#scaffoldthe-payload-is-discovered--the-payload-is-discovered-never-enumerated)
  - [`scaffold:an-earned-directory-is-not-seeded` — An earned directory is not seeded](#scaffoldan-earned-directory-is-not-seeded--an-earned-directory-is-not-seeded)
  - [`scaffold:the-travelling-document-is-self-sufficient` — The travelling document is self-sufficient](#scaffoldthe-travelling-document-is-self-sufficient--the-travelling-document-is-self-sufficient)
  - [`scaffold:a-gate-name-is-substituted-or-absent` — A gate name is substituted or declared absent](#scaffolda-gate-name-is-substituted-or-absent--a-gate-name-is-substituted-or-declared-absent)
  - [`scaffold:the-travelling-document-carries-no-link` — The travelling document carries no relative link](#scaffoldthe-travelling-document-carries-no-link--the-travelling-document-carries-no-relative-link)
  - [`scaffold:the-hook-set-installs-into-the-plan-repository` — The hook set installs into the plan repository](#scaffoldthe-hook-set-installs-into-the-plan-repository--the-hook-set-installs-into-the-plan-repository)
  - [`scaffold:a-hook-never-invokes-a-writer` — A hook never invokes a writer](#scaffolda-hook-never-invokes-a-writer--a-hook-never-invokes-a-writer)
  - [`scaffold:the-hook-set-is-printable-and-checked` — The hook set is printable and checked](#scaffoldthe-hook-set-is-printable-and-checked--the-hook-set-is-printable-and-checked)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What the scaffold lands and the contract each landed file carries. The scaffold writes into two places: one project file, inside a `.wipctl` directory it creates in the host repository, and the plan repository into this machine's attachment registry. The boundary runs at creation. This domain says what the scaffold produces, while the attachment domain says how a later run finds it again.

## Payload inventory

| Payload member      | Lands at                      | Notes                                                                                                                                                 |
| ------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| host project file   | `<root>/.wipctl/project.toml` | generated, not copied: the minted `project_id` and nothing else. The scaffold creates the directory and the file together                             |
| plan config         | `<zone>/.wipctl/plan.toml`    | generated: the minted id restated, the minted `plan_uid`, the given or today's start date under `[window]`, `length_days = 14` as the one baked value |
| zone `README.md`    | `<zone>/README.md`            | orientation: where to start, what bounds it, what stops it, what already happened                                                                     |
| `AGENTS.md`         | `<zone>/AGENTS.md`            | the travelling method; gate placeholders substituted at emission                                                                                      |
| `charter.md`        | `<zone>/charter.md`           | angle-bracketed placeholders for a person to fill                                                                                                     |
| `open-questions.md` | `<zone>/open-questions.md`    | heading plus the `Blocks:` grammar; ships without a sample question                                                                                   |
| lane files          | `<zone>/lanes/*.yml`          | each exactly `lane: <name>` plus `stories: []`                                                                                                        |
| stories directory   | `<zone>/stories/`             | created empty                                                                                                                                         |
| hook set            | the plan repository           | the record gate at both stages, the schema checks, the three heading-shape checks, the single-branch guard, the no-force guard                        |
| templates           | not copied                    | shipped location printed for a person to copy                                                                                                         |

The zone is the plan repository's working tree, which the scaffold creates whole: the repository, the trunk, the hook set, and the initial commit.

The scaffold mints both identities and reports both. `project_id` is minted against this machine's registry, and `plan_uid` is minted from the machine's random source. The uid is reported because another plan repository writes it down. An operator who cannot read it in the report reads it out of a file instead.

## Requirements

### `scaffold:the-host-receives-one-file` — The host receives one file

When the scaffold runs, the host repository MUST gain exactly one file it did not have, and MUST receive no merged content.

#### Scenario: A host already carries a documentation tree

- GIVEN a host repository with its own documents and configuration
- WHEN the scaffold runs
- THEN the host gains `.wipctl/project.toml` alone, because merging into a file the host owns makes the tool a co-author of that file

Verify: `cargo nextest run --test journey`

### `scaffold:the-payload-names-no-documentation-method` — The payload names no documentation method

A payload member MUST NOT name a documentation method or the tooling that installs one.

#### Scenario: A landed document illustrates a rule delta

- GIVEN a payload page showing an `Amends` item against a host that keeps requirement-level specifications
- WHEN the check runs
- THEN the generic convention passes and a product name fails. A payload naming one teaches every scaffolded project to adopt it

Verify: `cargo nextest run --test agnosticism`

### `scaffold:the-scaffold-writes-only-what-it-creates` — The scaffold writes only what it creates

The scaffold MUST write into a file or a repository only where the scaffold itself created it.

#### Scenario: Hooks are installed into the plan repository

- GIVEN a plan repository the scaffold created whole
- WHEN the hook set installs into it
- THEN the write is legal, because the scaffold owns what it made, and the host's own history still receives nothing

Verify: `cargo nextest run --test journey`

### `scaffold:the-payload-is-discovered` — The payload is discovered, never enumerated

The scaffold MUST discover its payload members from the shipped data directory rather than from a list in the code.

#### Scenario: The payload gains a member

- GIVEN a new top-level document added to the shipped payload
- WHEN the scaffold runs without any code change
- THEN the new member is emitted, because a hard-coded list is the copy that goes stale

Verify: `cargo nextest run --test scaffold`

### `scaffold:an-earned-directory-is-not-seeded` — An earned directory is not seeded

The scaffold MUST NOT create the epic directory, the initiative directory, a first story, the capture directory, or the external sources declaration.

#### Scenario: A fresh record is read for the first time

- GIVEN a scaffolded plan repository
- WHEN a reader opens it
- THEN nothing must be deleted before the record can be believed, and the capture directory appears when a capture creates it

Verify: `cargo nextest run --test scaffold`

### `scaffold:the-travelling-document-is-self-sufficient` — The travelling document is self-sufficient

The landed method document MUST teach every subject the list below names.

#### Scenario: A reader has only ever seen a plan beside the code

- GIVEN an adopting project whose only channel to the method is this document
- WHEN the reader opens it
- THEN the two-location model is taught from zero, because it is the one channel through which the method reaches them

The subjects, each taught from zero:

- the two-location model: one project file in the host, one plan repository at the machine-level slot every checkout shares
- all three heading sequences, with a one-line purpose per heading, the initiative section rule, and the depth rule
- that membership above the story flows through the epic
- the lane semantics, the entry field list, the points scale, and the canonical subset
- that a dependency id can carry an alias prefix naming an entry in a peer, and that it then means what a bare id means
- the peer table, and the one command that fills every peer's slot on a machine
- the id grammar, the slugification function, the mint's check against the live record and every tombstone, the rephrase resolution, and the hand-mint procedure
- eligibility, the two ranking rules, and the blocking grammar
- the move-and-close procedure, and why an edit is not a move
- the take verb beside the preview verb, and that an agent holds nothing while it works
- capture and drain, which lanes a fragment claims, and what a drift report asks of a reader
- that every plan mutation is committed to the plan trunk by the tool, with no opt-out, and that the host's history receives nothing
- replication, and when to run it
- the reference split: inbound, outbound, the promise marker, the optional typed rule delta, and the acceptance transfer as a same-unit-of-review obligation
- the gate commands, their exit codes, and the explicit list of what no gate catches

Verify: reviewer confirms the landed document covers each subject the list names

### `scaffold:a-gate-name-is-substituted-or-absent` — A gate name is substituted or declared absent

When the scaffold emits the method document, it MUST substitute each gate name from the build and MUST name an absent verb as absent.

#### Scenario: A build lacks a verb the document mentions

- GIVEN an emitting build without one gate verb
- WHEN the document is written
- THEN the verb is named absent rather than promised, because a promised command is a defect a reader finds at the worst moment

Verify: `cargo nextest run --test scaffold`

### `scaffold:the-travelling-document-carries-no-link` — The travelling document carries no relative link

The landed method document MUST NOT carry a relative link into any repository.

#### Scenario: The document is read from a different checkout

- GIVEN a landed document linking to a sibling page by relative path
- WHEN a reader opens it from another location
- THEN the link resolves to nothing, so the document states what it needs instead

Verify: `cargo nextest run --test scaffold`

### `scaffold:the-hook-set-installs-into-the-plan-repository` — The hook set installs into the plan repository

The scaffold MUST install the hook set into the plan repository and MUST NOT install it into the host.

#### Scenario: A host already runs its own commit gates

- GIVEN a host with a commit gate of its own
- WHEN the scaffold runs
- THEN the record's gates live with the record, because the plan repository is tool-owned and the host is not

Verify: `cargo nextest run --test journey`

### `scaffold:a-hook-never-invokes-a-writer` — A hook never invokes a writer

An installed hook MUST NOT invoke a verb that writes the record.

#### Scenario: A gate repairs what it finds

- GIVEN a hook calling the repair verb on a failing record
- WHEN a commit runs it
- THEN the gate rewrites the change under the author, so a hook reads and refuses and never writes

Verify: `cargo nextest run --test scaffold`

### `scaffold:the-hook-set-is-printable-and-checked` — The hook set is printable and checked

The implementation MUST print the hook set for inspection on request, and MUST report a missing installation as a diagnostic failure.

#### Scenario: A clone has the record and no hooks

- GIVEN a plan repository cloned onto a second machine
- WHEN the diagnostic runs
- THEN the missing hook installation is reported, because an ungated record drifts silently

Verify: `cargo nextest run --test verb_contracts`

## Unenforced rules

| Rule                                                  | Why no command decides it                                                                |
| ----------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `scaffold:the-travelling-document-is-self-sufficient` | Whether a subject is taught from zero is a reading of the prose, not a match against it. |

Naming a document class the host keeps or does not keep is the same fault as naming a method, at a smaller scale. A specification and a decision record are two such classes. This project's own specification states such a class only as one option among alternatives.
