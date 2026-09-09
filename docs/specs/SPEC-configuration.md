# Configuration Specification

<!--TOC-->

- [Purpose](#purpose)
- [The two files](#the-two-files)
- [The two identities](#the-two-identities)
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
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

The two configuration files and what each one owns. The project file identifies, and the plan file configures. The boundary runs at ownership. This domain says what each key means and what rejects a bad one. The attachment domain says how an invocation reaches the plan repository.

## The two files

wipctl reads its configuration from a `.wipctl` directory, in both places. The directory carries the product's name, so the file inside it is free to carry the name of the domain it serves. The product has two domains that own committed state, and they are the project and the plan.

```text
host repo/
  .wipctl/
    project.toml       the project's configuration

plan zone/
  .wipctl/
    plan.toml          the plan's configuration
```

`.wipctl/project.toml` lives at the project root of the host repository. It is committed and travels with every clone.

```toml
project_id = "payments-acme"
```

`.wipctl/plan.toml` lives at the zone root, inside the plan repository. It restates the identity, declares the plan's own global identity, and carries every fact about the plan itself, because the plan owns its own home.

```toml
project_id = "payments-acme"
plan_uid = "9f2c41a08b7d4e63a15c8f02d7e4b619"

[window]
start = 2026-07-06
length_days = 14

[stale_after]
days = 21

[peers.payments]
uid = "9f2c41a08b7d4e63a15c8f02d7e4b619"
urls = ["https://git.example.org/acme/payments-plan.git"]

[sources.issues]
url_template = "https://tracker.example/issues/{key}"
list_command = ["<the command that lists open items>", "--format", "json"]
```

The file has a required core and two optional sections. The core is every key above `[peers]`, and the two sections are `peers` and `sources`. Each section name states what the operator decided. `window.length_days = 14` says the counting window is fourteen days long, and `stale_after.days = 21` says work goes stale after twenty-one days.

```text
[peers.<alias>]     the other plans this one names. The peers domain owns
                    every rule about an alias, a uid, and a url.

[sources.<alias>]   the outside systems this plan's artifacts reference.
                    The external sources domain owns every rule about a
                    reference and a command.
```

This domain owns the file: where it lives, which sections it holds, and what an absent section means. It gains no semantics. A reader who wants to know what a url can contain reads the peers page.

Each directory holds one file and nothing else. The cost is stated rather than hidden: a genuine later need for a second file is a change to this specification, not a quiet addition. That price buys the property the whole shape rests on, which is that a plan declares itself in one place.

A `.wipctl` directory holding no configuration file is not a root and is not silence. It arrives from a partial copy, a failed scaffold, or a person who made the directory by hand. The walk reports it rather than passing over it.

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

A peer row names another plan, and the peers domain owns what such a row means. This file is where the row is written, and the section reference above is all this domain says about it.

## Requirements

### `configuration:no-key-has-a-read-time-default` — No key has a read-time default

When a verb finds a required key absent, the verb MUST fail the check rather than assume a value.

#### Scenario: The window length is missing

- GIVEN a plan configuration without `window.length_days`
- WHEN a velocity window is computed
- THEN the verb fails naming the field, because a value guessed on a project's behalf is a value nobody wrote down

Verify: `cargo nextest run --test configuration`

### `configuration:the-project-file-carries-one-key` — The project file carries one key

The host repository's `.wipctl/project.toml` MUST carry `project_id` and no other key, in the slug grammar `[a-z0-9]+(-[a-z0-9]+)*`.

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

The plan configuration MUST carry `plan_uid` as exactly 32 lowercase hexadecimal characters, minted once when the plan is created.

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

- No `.wipctl/project.toml` at or above the working directory. This is a usage error, exit 2. The message names the scaffold as the resolution.
- A `.wipctl` directory with no `project.toml` inside it. This is a usage error, exit 2, naming the directory. The message says that the project root is the directory holding `.wipctl/project.toml`, and it offers two resolutions: run the scaffold here, or remove the empty directory and let the walk continue upward.
- A second file inside a `.wipctl` directory. A failed check naming the file and the rule that binds it.
- `project_id` absent or outside the slug grammar, in either file. A failed check naming the file and the field.
- The two `project_id` values disagree. A failed check naming both files, both values, and the choice between re-attaching and correcting whichever file is wrong.
- A required key absent from the plan configuration, including a required key of a present optional section. A failed check naming the field.
- `plan_uid` absent or outside its grammar. A failed check naming the file and the key. The message names the one verb form that gives an identity to a record that predates the key.
- A malformed value. The schema half of validation reports it.
