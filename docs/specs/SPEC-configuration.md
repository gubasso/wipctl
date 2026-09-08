# Configuration Specification

<!--TOC-->

- [Purpose](#purpose)
- [The two files](#the-two-files)
- [The two identities](#the-two-identities)
- [Requirements](#requirements)
  - [`configuration:no-key-has-a-read-time-default` — No key has a read-time default](#configurationno-key-has-a-read-time-default--no-key-has-a-read-time-default)
  - [`configuration:the-identity-file-carries-one-key` — The identity file carries one key](#configurationthe-identity-file-carries-one-key--the-identity-file-carries-one-key)
  - [`configuration:the-identity-file-marks-the-root` — The identity file marks the project root](#configurationthe-identity-file-marks-the-root--the-identity-file-marks-the-project-root)
  - [`configuration:the-plan-declares-a-global-identity` — The plan declares a global identity](#configurationthe-plan-declares-a-global-identity--the-plan-declares-a-global-identity)
  - [`configuration:the-two-identities-agree` — The two identities agree](#configurationthe-two-identities-agree--the-two-identities-agree)
  - [`configuration:an-unknown-key-is-rejected` — An unknown key is rejected](#configurationan-unknown-key-is-rejected--an-unknown-key-is-rejected)
  - [`configuration:the-iteration-window-is-required` — The iteration window is required](#configurationthe-iteration-window-is-required--the-iteration-window-is-required)
  - [`configuration:the-aging-threshold-is-optional-and-undefaulted` — The aging threshold is optional and undefaulted](#configurationthe-aging-threshold-is-optional-and-undefaulted--the-aging-threshold-is-optional-and-undefaulted)
  - [`configuration:the-identity-file-owes-no-schema` — The identity file owes no schema](#configurationthe-identity-file-owes-no-schema--the-identity-file-owes-no-schema)
  - [`configuration:there-is-no-commit-configuration` — There is no commit configuration](#configurationthere-is-no-commit-configuration--there-is-no-commit-configuration)
  - [`configuration:the-schema-owns-types-and-the-checker-owns-agreement` — The schema owns types and the checker owns agreement](#configurationthe-schema-owns-types-and-the-checker-owns-agreement--the-schema-owns-types-and-the-checker-owns-agreement)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

The two configuration files and what each one owns. The host file identifies, and the plan repository configures. The boundary runs at ownership. This domain says what each key means and what rejects a bad one. The attachment domain says how an invocation reaches the plan repository.

## The two files

`.wipctl.toml` lives at the project root of the host repository. It is hidden, committed, and travels with every clone.

```toml
project_id = "payments-acme"
```

`config.toml` lives at the zone root, inside the plan repository. It restates the identity, declares the plan's own global identity, and carries every fact about the plan itself, because the plan owns its own home.

```toml
project_id = "payments-acme"
plan_uid = "9f2c41a08b7d4e63a15c8f02d7e4b619"

[iteration]
start = 2026-07-06
length_days = 14

[aging]
threshold_days = 21
```

## The two identities

A plan carries two names, and they answer two questions.

```text
project_id    slug         resolution key and human name; unique on THIS
                           machine; renaming it is an explicit operation
plan_uid      32 hex       global identity; written down by other plans;
                           never renamed, never derived, never parsed
```

`project_id` answers which plan on this machine. `plan_uid` answers which plan anywhere. The first is a path segment under the data directory. The second is a value another repository commits. A machine-local slug cannot be the second one. Two operators mint the same slug without ever meeting, because uniqueness is decided against this machine's registry and not globally.

`plan_uid` has four properties and no more:

```text
grammar      exactly 32 lowercase hexadecimal characters
mint         128 bits from the machine's random source, at creation, once
immutable    no verb changes it, and there is no rename operation for it
opaque       nothing parses it; it carries no time, no machine, no order
```

Thirty-two hexadecimal characters sit inside the slug grammar as one segment, so no other grammar widens to hold the value. The mint size is stated as a fact. 128 bits of randomness makes a collision between two independently minted plans a risk no operator thinks about. No coordination between machines buys anything more.

Immutability binds verbs. Two replicas that each minted a uid before either replicated are a disagreement in one committed file. An operator settles it by editing that file and committing. There is no verb for it, and the reason is the same one that makes the value useful. Other plans write it down, and a verb that rewrites it rewrites one they already hold.

The uid never names a slot. A verb resolves a plan by the two steps the attachment domain states, then reads `plan_uid` inside the slot it reached. There is no tree keyed by the uid.

A peer is not configuration. `config.toml` carries facts about this plan, and a row naming another plan is a fact about that one. The peers domain owns a file of its own for them.

## Requirements

### `configuration:no-key-has-a-read-time-default` — No key has a read-time default

When a verb finds a required key absent, the verb MUST fail the check rather than assume a value.

#### Scenario: The iteration length is missing

- GIVEN a plan configuration without `iteration.length_days`
- WHEN a velocity window is computed
- THEN the verb fails naming the field, because a value guessed on a project's behalf is a value nobody wrote down

Verify: `cargo nextest run --test configuration`

### `configuration:the-identity-file-carries-one-key` — The identity file carries one key

The host identity file MUST carry `project_id` and no other key, in the slug grammar `[a-z0-9]+(-[a-z0-9]+)*`.

#### Scenario: A project adds a setting to the host file

- GIVEN a host file gaining an iteration length
- WHEN the file is read
- THEN the unknown key is rejected, because the host identifies and the plan repository configures

Verify: `cargo nextest run --test configuration`

### `configuration:the-identity-file-marks-the-root` — The identity file marks the project root

The directory holding the host identity file MUST be the project root, and nothing else MUST mark it.

#### Scenario: A verb runs from a nested directory

- GIVEN an invocation deep inside the source tree
- WHEN the verb resolves the project
- THEN the upward walk stops at the identity file, because a second root marker lets two directories both claim the root

Verify: `cargo nextest run --test attachment`

### `configuration:the-plan-declares-a-global-identity` — The plan declares a global identity

The plan configuration MUST carry `plan_uid` as exactly 32 lowercase hexadecimal characters, minted once when the plan is created.

#### Scenario: A plan another plan named moves to a new forge

- GIVEN a peer table in another plan repository holding this plan's uid
- WHEN this plan is renamed, rehosted, or moved between forges
- THEN the reference still resolves, because the identity is the value that repository committed and the location is a hint beside it

Verify: `cargo nextest run --test schemas`

### `configuration:the-two-identities-agree` — The two identities agree

The plan configuration's `project_id` MUST equal the host identity file's value, checked at every resolution.

#### Scenario: A slot holds another project's plan

- GIVEN a host naming `payments-acme` and a plan repository naming `billing-acme`
- WHEN any verb resolves
- THEN the check fails naming both sides and the choice, because acting on the wrong record is worse than refusing

Verify: `cargo nextest run --test attachment`

### `configuration:an-unknown-key-is-rejected` — An unknown key is rejected

The implementation MUST reject an unknown key at every level of either file.

#### Scenario: A key is misspelled

- GIVEN `threshold_dayz` under the aging table
- WHEN the configuration is read
- THEN the key is rejected, because silently ignoring it makes a typo look like a setting that had no effect

Verify: `cargo nextest run --test schemas`

### `configuration:the-iteration-window-is-required` — The iteration window is required

The plan configuration MUST carry `iteration.start` as a calendar date and `iteration.length_days` as an integer of at least 1.

#### Scenario: The cadence changes mid-project

- GIVEN a project changing either value
- WHEN velocity is next computed
- THEN the window series restarts, because the pair defines every window and no window survives a redefinition

Verify: `cargo nextest run --test configuration`

### `configuration:the-aging-threshold-is-optional-and-undefaulted` — The aging threshold is optional and undefaulted

Where the aging table is present, `threshold_days` MUST be an integer of at least 1, and its absence MUST leave every bar unmarked.

#### Scenario: A project never sets a threshold

- GIVEN a configuration with no aging table
- WHEN the aging view renders
- THEN no row is marked, because there is no default threshold to mark against

Verify: `cargo nextest run --test verb_contracts`

### `configuration:the-identity-file-owes-no-schema` — The identity file owes no schema

The host identity file MUST NOT carry a schema of its own, and the cross-file checker MUST own its presence, its grammar, and its agreement.

#### Scenario: Someone proposes a schema for one key

- GIVEN a file whose whole content is one required key
- WHEN a schema is proposed for it
- THEN the file is below a schema's floor, the same reasoning under which a one-line output owes no machine format

Verify: `test ! -e docs/specs/SPEC-configuration/wipctl.schema.json`

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

## Diagnostics

- No identity file at or above the working directory. This is a usage error, exit 2. The message names the scaffold as the resolution.
- `project_id` absent or outside the slug grammar, in either file. A failed check naming the file and the field.
- The two `project_id` values disagree. A failed check naming both files, both values, and the choice between re-attaching and correcting whichever file is wrong.
- A required key absent from the plan configuration. A failed check naming the field.
- `plan_uid` absent or outside its grammar. A failed check naming the file and the key. The message names the one verb form that gives an identity to a record that predates the key.
- A malformed value. The schema half of validation reports it.
