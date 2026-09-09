# Configuration Specification

<!--TOC-->

- [Purpose](#purpose)
- [Configuration shape](#configuration-shape)
- [Open question](#open-question)
- [Requirements](#requirements)
  - [`configuration:no-key-has-a-read-time-default` — No key has a read-time default](#configurationno-key-has-a-read-time-default--no-key-has-a-read-time-default)
  - [`configuration:the-project-file-carries-one-key` — The project file carries one key](#configurationthe-project-file-carries-one-key--the-project-file-carries-one-key)
  - [`configuration:the-project-file-marks-the-root` — The project file marks the project root](#configurationthe-project-file-marks-the-root--the-project-file-marks-the-project-root)
  - [`configuration:the-plan-declares-a-global-identity` — The plan declares a global identity](#configurationthe-plan-declares-a-global-identity--the-plan-declares-a-global-identity)
  - [`configuration:the-two-identities-agree` — The two identities agree](#configurationthe-two-identities-agree--the-two-identities-agree)
  - [`configuration:an-unknown-key-is-rejected` — An unknown key is rejected](#configurationan-unknown-key-is-rejected--an-unknown-key-is-rejected)
  - [`configuration:an-optional-section-is-legally-absent` — An optional section is legally absent](#configurationan-optional-section-is-legally-absent--an-optional-section-is-legally-absent)
  - [`configuration:the-window-is-required` — The window is required](#configurationthe-window-is-required--the-window-is-required)
  - [`configuration:the-stale-threshold-is-optional-and-undefaulted` — The stale threshold is optional and undefaulted](#configurationthe-stale-threshold-is-optional-and-undefaulted--the-stale-threshold-is-optional-and-undefaulted)
  - [`configuration:the-project-file-owes-no-schema` — The project file owes no schema](#configurationthe-project-file-owes-no-schema--the-project-file-owes-no-schema)
  - [`configuration:the-configuration-directory-holds-one-file` — The configuration directory holds one file](#configurationthe-configuration-directory-holds-one-file--the-configuration-directory-holds-one-file)
  - [`configuration:there-is-no-commit-configuration` — There is no commit configuration](#configurationthere-is-no-commit-configuration--there-is-no-commit-configuration)
  - [`configuration:the-schema-owns-types-and-the-checker-owns-agreement` — The schema owns types and the checker owns agreement](#configurationthe-schema-owns-types-and-the-checker-owns-agreement--the-schema-owns-types-and-the-checker-owns-agreement)
  - [`configuration:a-superseded-plan-id-has-the-identity-grammar` — A superseded plan id has the identity grammar](#configurationa-superseded-plan-id-has-the-identity-grammar--a-superseded-plan-id-has-the-identity-grammar)
  - [`configuration:the-current-plan-id-is-not-superseded` — The current plan id is not superseded](#configurationthe-current-plan-id-is-not-superseded--the-current-plan-id-is-not-superseded)
  - [`configuration:a-superseded-plan-id-appears-once` — A superseded plan id appears once](#configurationa-superseded-plan-id-appears-once--a-superseded-plan-id-appears-once)
  - [`configuration:superseded-plan-ids-are-append-only` — Superseded plan ids are append-only](#configurationsuperseded-plan-ids-are-append-only--superseded-plan-ids-are-append-only)
  - [`configuration:only-mint-settlement-adds-a-superseded-id` — Only mint settlement adds a superseded id](#configurationonly-mint-settlement-adds-a-superseded-id--only-mint-settlement-adds-a-superseded-id)
  - [`configuration:plan-identity-claims-are-disjoint` — Plan identity claims are disjoint](#configurationplan-identity-claims-are-disjoint--plan-identity-claims-are-disjoint)
  - [`configuration:a-peer-row-resolves-to-one-canonical-plan` — A peer row resolves to one canonical plan](#configurationa-peer-row-resolves-to-one-canonical-plan--a-peer-row-resolves-to-one-canonical-plan)

<!--TOC-->

## Purpose

The project file identifies the project, and the plan file configures its plan. The attachment domain owns how an invocation reaches the plan.

## Configuration shape

```toml
# .wipctl/project.toml
project_id = "a1b2c3d4e5f60718293a4b5c6d7e8f90"

# .wipctl/plan.toml
project_id = "a1b2c3d4e5f60718293a4b5c6d7e8f90"
plan_id = "9f2c41a08b7d4e63a15c8f02d7e4b619"
superseded_plan_ids = ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
```

Both identities are 128 random bits written as exactly 32 lowercase hexadecimal characters. Each is minted once, immutable, opaque, and parsed by nothing.

The plan file also holds its window and optional tables. The superseded list keeps discarded plan identities resolvable.

## Open question

A plan cloned outside a host or peer table can derive a name only from its directory. Whether a plan repository carries a human name of its own remains open. No optional name key is added.

## Requirements

### `configuration:no-key-has-a-read-time-default` — No key has a read-time default

When a verb finds a required key absent, the verb MUST fail the check rather than assume a value.

#### Scenario: The window length is missing

- GIVEN a plan configuration without `window.length_days`
- WHEN a velocity window is computed
- THEN the verb fails naming the field, because a value guessed on a project's behalf is a value nobody wrote down

Verify: `cargo nextest run --test configuration`

### `configuration:the-project-file-carries-one-key` — The project file carries one key

The host repository's `.wipctl/project.toml` MUST carry only `project_id`, written as exactly 32 lowercase hexadecimal characters.

#### Scenario: A project adds a setting to the project file

- GIVEN a project file gaining a window length
- WHEN the file is read
- THEN the unknown key is rejected, because the host identifies and the plan repository configures

Verify: `cargo nextest run --test configuration`

### `configuration:the-project-file-marks-the-root` — The project file marks the project root

The directory holding `.wipctl/project.toml` MUST be the project root, and nothing else MUST mark it.

#### Scenario: A verb runs from a nested directory

- GIVEN an invocation deep inside the source tree
- WHEN the verb resolves the project
- THEN the upward walk stops at the first directory holding `.wipctl/project.toml`, because a second root marker lets two directories both claim the root

Verify: `cargo nextest run --test attachment`

### `configuration:the-plan-declares-a-global-identity` — The plan declares a global identity

The plan configuration MUST carry `plan_id` as exactly 32 lowercase hexadecimal characters, minted once when the plan is created.

#### Scenario: A plan another plan named moves to a new forge

- GIVEN a peer table in another plan repository holding this plan's uid
- WHEN this plan is renamed, rehosted, or moved between forges
- THEN the reference still resolves, because the identity is the value that repository committed and the location is a hint beside it

Verify: `cargo nextest run --test schemas`

### `configuration:the-two-identities-agree` — The two identities agree

The plan configuration's `project_id` MUST equal the project file's value, checked at every resolution.

#### Scenario: A slot holds another project's plan

- GIVEN a host naming `payments-acme` and a plan repository naming `billing-acme`
- WHEN any verb resolves
- THEN the check fails naming both sides and the choice, because acting on the wrong record is worse than refusing

Verify: `cargo nextest run --test attachment`

### `configuration:an-unknown-key-is-rejected` — An unknown key is rejected

The implementation MUST reject an unknown key at every level of either file.

#### Scenario: A key is misspelled

- GIVEN `[peer.payments]` written in the singular
- WHEN the configuration is read
- THEN the key is rejected, because silently ignoring it makes a typo look like a section that had no effect

Verify: `cargo nextest run --test schemas`

### `configuration:an-optional-section-is-legally-absent` — An optional section is legally absent

The `peers` and `sources` sections MUST be optional, each absence MUST mean what its own domain states, and a present section MUST be complete.

#### Scenario: A plan names no peer and declares no source

- GIVEN a scaffolded plan repository carrying the core alone
- WHEN the configuration is read
- THEN it is valid and each absence carries its stated meaning, because an empty section is a statement nobody made

Verify: `cargo nextest run --test schemas`

### `configuration:the-window-is-required` — The window is required

The plan configuration MUST carry `window.start` as a calendar date and `window.length_days` as an integer of at least 1.

#### Scenario: The cadence changes mid-project

- GIVEN a project changing either value
- WHEN velocity is next computed
- THEN the window series restarts, because the pair defines every window and no window survives a redefinition

Verify: `cargo nextest run --test configuration`

### `configuration:the-stale-threshold-is-optional-and-undefaulted` — The stale threshold is optional and undefaulted

Where the `stale_after` table is present, `days` MUST be an integer of at least 1, and its absence MUST leave every bar unmarked.

#### Scenario: A project never sets a threshold

- GIVEN a configuration with no `stale_after` table
- WHEN the stale view renders
- THEN no row is marked, because there is no default threshold to mark against

Verify: `cargo nextest run --test verb_contracts`

### `configuration:the-project-file-owes-no-schema` — The project file owes no schema

The project file MUST NOT carry a schema of its own, and the cross-file checker MUST own its presence, its grammar, and its agreement.

#### Scenario: Someone proposes a schema for one key

- GIVEN a file whose whole content is one required key
- WHEN a schema is proposed for it
- THEN the file is below a schema's floor, the same reasoning under which a one-line output owes no machine format

Verify: `test ! -e docs/specs/SPEC-configuration/project.schema.json`

### `configuration:the-configuration-directory-holds-one-file` — The configuration directory holds one file

A `.wipctl` directory MUST hold the one configuration file its place names, and MUST NOT hold a second file.

#### Scenario: A project wants a second file beside the plan configuration

- GIVEN a request to put another wipctl file inside the zone's `.wipctl` directory
- WHEN the request is read
- THEN it is refused, because a directory invites a second file, and that is how a split configuration returns one addition at a time

Verify: `cargo nextest run --test validation`

### `configuration:a-superseded-plan-id-has-the-identity-grammar` — A superseded plan id has the identity grammar

Every `superseded_plan_ids` entry MUST contain exactly 32 lowercase hexadecimal characters.

#### Scenario: A discarded identity is malformed

- GIVEN an array entry with 31 characters
- WHEN the schema reads the plan configuration
- THEN the value fails the same grammar as the canonical plan identity

Verify: `cargo nextest run --test schemas`

### `configuration:the-current-plan-id-is-not-superseded` — The current plan id is not superseded

The `superseded_plan_ids` array MUST exclude the current `plan_id`.

#### Scenario: The canonical identity appears in both places

- GIVEN one value used as `plan_id` and as an array entry
- WHEN the checker reads the configuration
- THEN it fails because one identity cannot be both current and discarded

Verify: `cargo nextest run --test configuration`

### `configuration:a-superseded-plan-id-appears-once` — A superseded plan id appears once

Every value in `superseded_plan_ids` MUST appear once.

#### Scenario: Settlement appends an existing value

- GIVEN an array that already records the discarded identity
- WHEN the schema reads a repeated entry
- THEN it rejects the duplicate because repetition states no new equivalence

Verify: `cargo nextest run --test schemas`

### `configuration:superseded-plan-ids-are-append-only` — Superseded plan ids are append-only

After an identity enters `superseded_plan_ids`, every verb MUST retain it.

#### Scenario: No current peer row uses an old identity

- GIVEN a superseded value with no visible consumer
- WHEN a writer edits the plan configuration
- THEN the value remains because an unknown plan can still have written it down

Verify: `cargo nextest run --test sync`

### `configuration:only-mint-settlement-adds-a-superseded-id` — Only mint settlement adds a superseded id

The implementation MUST add a superseded identity only when it settles two plan identities minted for one plan.

#### Scenario: An operator tries to claim another plan's identity

- GIVEN an identity with no two-mint conflict
- WHEN a verb is asked to append it
- THEN the verb refuses because unrestricted equivalence can claim another plan

Verify: `cargo nextest run --test sync`

### `configuration:plan-identity-claims-are-disjoint` — Plan identity claims are disjoint

Across attached slots, the checker MUST let each canonical or superseded plan identity belong to one plan only.

#### Scenario: Two plans claim one discarded identity

- GIVEN two attached plans whose identity sets overlap
- WHEN validation reads the registry
- THEN it fails naming both slots and the shared value

Verify: `cargo nextest run --test validation`

### `configuration:a-peer-row-resolves-to-one-canonical-plan` — A peer row resolves to one canonical plan

When a peer row carries a superseded value, the implementation MUST resolve it and report the plan's canonical `plan_id`.

#### Scenario: A peer row predates identity settlement

- GIVEN a row carrying the discarded identity
- WHEN the peer is resolved
- THEN it reaches the plan and reports the canonical identity without changing the row

Verify: `cargo nextest run --test validation`

### `configuration:there-is-no-commit-configuration` — There is no commit configuration

The configuration MUST NOT carry a table that turns the plan trunk's commit off.

#### Scenario: A project wants to batch its plan commits

- GIVEN a request for a commit setting
- WHEN the configuration is read
- THEN no such table exists, because every plan mutation is committed by the tool and the host's history gains nothing

Verify: `cargo nextest run --test configuration`

### `configuration:the-schema-owns-types-and-the-checker-owns-agreement` — The schema owns types and the checker owns agreement

The plan configuration's schema MUST own its types and constraints, and the cross-file checker MUST own presence and agreement.

#### Scenario: A malformed value and a missing file arrive together

- GIVEN a start date written as a string and an absent host file
- WHEN validation runs
- THEN the schema half reports the malformed value and the cross-file half reports the missing identity, each in its own class

Verify: `cargo nextest run --test validation`
