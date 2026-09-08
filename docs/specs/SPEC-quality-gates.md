# Quality Gates Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`quality-gates:an-artifact-carries-a-hook-and-a-case` — A shipped artifact carries a hook and a case](#quality-gatesan-artifact-carries-a-hook-and-a-case--a-shipped-artifact-carries-a-hook-and-a-case)
  - [`quality-gates:every-artifact-carries-a-toolchain-pin` — A shipped artifact carries its toolchain](#quality-gatesevery-artifact-carries-a-toolchain-pin--a-shipped-artifact-carries-its-toolchain)
  - [`quality-gates:a-gate-fails-once-before-it-is-trusted` — A gate fails once before it is trusted](#quality-gatesa-gate-fails-once-before-it-is-trusted--a-gate-fails-once-before-it-is-trusted)
  - [`quality-gates:the-project-plans-with-its-own-method` — The project plans with the method it ships](#quality-gatesthe-project-plans-with-its-own-method--the-project-plans-with-the-method-it-ships)
  - [`quality-gates:the-worked-example-passes-validate` — The worked example passes validation](#quality-gatesthe-worked-example-passes-validate--the-worked-example-passes-validation)
  - [`quality-gates:every-schema-validates-against-the-metaschema` — Every schema validates against the metaschema](#quality-gatesevery-schema-validates-against-the-metaschema--every-schema-validates-against-the-metaschema)
  - [`quality-gates:every-verb-contract-carries-a-case` — Every verb contract carries a case](#quality-gatesevery-verb-contract-carries-a-case--every-verb-contract-carries-a-case)
  - [`quality-gates:every-refusal-names-its-resolution` — Every refusal case asserts its guidance](#quality-gatesevery-refusal-names-its-resolution--every-refusal-case-asserts-its-guidance)
  - [`quality-gates:the-writer-guarantees-are-gated` — The writer guarantees are gated](#quality-gatesthe-writer-guarantees-are-gated--the-writer-guarantees-are-gated)
  - [`quality-gates:the-journey-runs-against-an-installation` — The journey runs against an installation](#quality-gatesthe-journey-runs-against-an-installation--the-journey-runs-against-an-installation)
  - [`quality-gates:a-rendering-verb-is-gated-on-pipe-safety` — A rendering verb is gated on pipe safety](#quality-gatesa-rendering-verb-is-gated-on-pipe-safety--a-rendering-verb-is-gated-on-pipe-safety)
  - [`quality-gates:the-record-gates-run-at-both-stages` — The record gates run at both stages](#quality-gatesthe-record-gates-run-at-both-stages--the-record-gates-run-at-both-stages)
  - [`quality-gates:the-repository-is-self-contained` — The repository is self-contained](#quality-gatesthe-repository-is-self-contained--the-repository-is-self-contained)
  - [`quality-gates:an-implementation-conforms-by-every-gate` — An implementation conforms by every gate](#quality-gatesan-implementation-conforms-by-every-gate--an-implementation-conforms-by-every-gate)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What any implementation of this product gates, in its own toolchain, and what it means to conform. The boundary runs at the artifact: this domain says what every shipped thing owes, while each other domain says what its own artifact must do. An artifact asserted in prose and gated by nothing reads as verified and is not.

## Requirements

### `quality-gates:an-artifact-carries-a-hook-and-a-case` — A shipped artifact carries a hook and a case

When an implementation ships an artifact, it MUST land a commit-gate hook and a test case in the same change.

#### Scenario: A schema ships in a change that adds no case

- GIVEN a new schema added to the payload
- WHEN the change lands with no test and no hook
- THEN the artifact reads as verified while nothing checks it, and review rejects the change

Verify: `cargo nextest run`

### `quality-gates:every-artifact-carries-a-toolchain-pin` — A shipped artifact carries its toolchain

When an implementation ships an artifact whose gate needs a tool, the implementation MUST pin that tool in its development environment.

#### Scenario: A gate runs on a contributor's machine and not on a clean clone

- GIVEN a hook calling a formatter the environment does not pin
- WHEN a fresh clone runs the gate
- THEN the tool is missing and the gate is decorative, so the pin lands with the hook

Verify: `nix develop --command just check`

### `quality-gates:a-gate-fails-once-before-it-is-trusted` — A gate fails once before it is trusted

Before an implementation trusts a gate, the implementation MUST show that gate failing on a deliberate defect.

#### Scenario: A hook's file pattern selects nothing

- GIVEN a hook whose pattern matches no path in the tree
- WHEN the suite runs
- THEN the hook reports success without reading a file, which a deliberate defect is the only way to tell apart

Verify: `cargo nextest run --test docs_gates`

### `quality-gates:the-project-plans-with-its-own-method` — The project plans with the method it ships

The implementation project MUST plan with the method it ships, and MUST gate that its own plan record is attached and passes its own validation.

#### Scenario: A clone carries no record

- GIVEN a fresh clone of the implementation
- WHEN the dogfooding gate runs
- THEN the clone carries the identity and the record is one attachment away, so the gate asserts attachment rather than presence

Verify: `cargo nextest run --test dogfooding`

### `quality-gates:the-worked-example-passes-validate` — The worked example passes validation

The shipped example project MUST pass validation, MUST match the schemas for its lane files, fragments, and configuration, and MUST carry one permanently pending fragment.

#### Scenario: The pending gates select nothing

- GIVEN an example whose fragments are all drained
- WHEN a pending gate runs against it
- THEN the gate passes over an empty set, so the example keeps one fragment pending on purpose

Verify: `cargo nextest run --test worked_example`

### `quality-gates:every-schema-validates-against-the-metaschema` — Every schema validates against the metaschema

The implementation MUST validate every shipped schema, output schemas included, against the draft 2020-12 metaschema.

#### Scenario: An output schema drifts from the draft

- GIVEN a schema using a keyword the draft removed
- WHEN the metaschema check runs
- THEN the schema fails, because a schema that is not itself valid gates nothing

Verify: `cargo nextest run --test schemas`

### `quality-gates:every-verb-contract-carries-a-case` — Every verb contract carries a case

The implementation MUST carry one case per verb contract, and MUST validate every machine format against its schema with real output.

#### Scenario: A verb gains a machine format with no schema case

- GIVEN a verb emitting a new JSON shape
- WHEN no case validates real output against the schema
- THEN the shape is unchecked, and the contract is prose

Verify: `cargo nextest run --test verb_contracts`

### `quality-gates:every-refusal-names-its-resolution` — Every refusal case asserts its guidance

When the implementation gates a refusal path, the case MUST assert that the refusal message names the resolution.

#### Scenario: A refusal states only what went wrong

- GIVEN a refusal whose message names the fault and no next step
- WHEN the case asserts the exit code alone
- THEN the guidance obligation is asserted in prose and gated by nothing

Verify: `cargo nextest run --test verb_contracts`

### `quality-gates:the-writer-guarantees-are-gated` — The writer guarantees are gated

The implementation MUST gate determinism, identity on legal input, idempotence, non-canonicality, refusal on broken content, one commit per transaction, and lock behavior.

#### Scenario: Two writers run at once

- GIVEN two concurrent writers against one record
- WHEN both commit
- THEN both survive, two concurrent takes claim two different entries, a bounded wait fails naming the holder, and a dead holder releases

Verify: `cargo nextest run --test writer_guarantees`

### `quality-gates:the-journey-runs-against-an-installation` — The journey runs against an installation

The implementation MUST drive the full two-repository adoption sequence against a temporary installation rather than against its source tree.

#### Scenario: The program works from the source tree only

- GIVEN a build that resolves its payload by a path relative to the source
- WHEN the journey installs into a temporary prefix and runs the sequence
- THEN the payload is not found, which is the one case only an installed run catches

Verify: `cargo nextest run --test journey`

### `quality-gates:a-rendering-verb-is-gated-on-pipe-safety` — A rendering verb is gated on pipe safety

The implementation MUST gate every rendering verb on emitting no escape sequence when its output is not a terminal.

#### Scenario: A panel is piped into a file

- GIVEN a rendering verb whose output is redirected
- WHEN the case reads the file
- THEN the file holds no escape sequence, and the asserted content is what remains after stripping

Verify: `cargo nextest run --test rendering`

### `quality-gates:the-record-gates-run-at-both-stages` — The record gates run at both stages

The record gates MUST run at both the commit stage and the push stage of the plan repository.

#### Scenario: A commit skips the commit hooks

- GIVEN a commit made with the commit hooks skipped
- WHEN the branch is pushed
- THEN the push stage still meets the gate before anything is published

Verify: `cargo nextest run --test scaffold`

### `quality-gates:the-repository-is-self-contained` — The repository is self-contained

The implementation repository MUST hold everything needed to understand, build, or gate it, and MUST restate borrowed substance rather than link to it.

#### Scenario: A rule lives in another project's documentation

- GIVEN a convention this project is bound by, documented well elsewhere
- WHEN the page links there instead of stating it
- THEN the rule changes when that project edits it, and a reader without access cannot learn what binds them

Verify: reviewer clones the repository on a machine with nothing else and confirms that it explains itself, builds, and gates

### `quality-gates:an-implementation-conforms-by-every-gate` — An implementation conforms by every gate

An implementation MUST pass every requirement in this domain before it claims conformance.

#### Scenario: A build passes its own suite and skips one domain

- GIVEN an implementation with no case for the transition journal
- WHEN it claims conformance
- THEN the claim fails, because conformance is the whole set and not the passing subset

Verify: `cargo nextest run`

## Unenforced rules

| Rule                                             | Why no command decides it                                                                                                   |
| ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| `quality-gates:the-repository-is-self-contained` | The clone-on-a-bare-machine test judges whether a link carries a claim or supports one, which is a reading and not a match. |

The project's own plan record is outside the self-containment rule. It is coordination state: nothing in it is needed to understand, build, or gate the implementation, and `quality-gates:the-project-plans-with-its-own-method` is what binds it.
