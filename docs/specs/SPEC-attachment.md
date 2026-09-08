# Attachment Specification

<!--TOC-->

- [Purpose](#purpose)
- [The postfix escalation](#the-postfix-escalation)
- [Where each identity enters resolution](#where-each-identity-enters-resolution)
- [Requirements](#requirements)
  - [`attachment:resolution-is-two-steps` — Resolution is two steps](#attachmentresolution-is-two-steps--resolution-is-two-steps)
  - [`attachment:a-plan-uid-is-minted-only-at-creation` — A global identity is minted only at creation](#attachmenta-plan-uid-is-minted-only-at-creation--a-global-identity-is-minted-only-at-creation)
  - [`attachment:one-plan-occupies-one-slot` — One plan occupies one slot](#attachmentone-plan-occupies-one-slot--one-plan-occupies-one-slot)
  - [`attachment:git-is-the-only-version-control` — Git is the only version control](#attachmentgit-is-the-only-version-control--git-is-the-only-version-control)
  - [`attachment:the-registry-is-the-filesystem` — The registry is the filesystem](#attachmentthe-registry-is-the-filesystem--the-registry-is-the-filesystem)
  - [`attachment:the-scaffold-does-not-walk-upward` — The scaffold does not walk upward](#attachmentthe-scaffold-does-not-walk-upward--the-scaffold-does-not-walk-upward)
  - [`attachment:an-id-is-minted-by-postfix-escalation` — An id is minted by postfix escalation](#attachmentan-id-is-minted-by-postfix-escalation--an-id-is-minted-by-postfix-escalation)
  - [`attachment:an-empty-or-repeated-segment-is-skipped` — An empty or repeated segment is skipped](#attachmentan-empty-or-repeated-segment-is-skipped--an-empty-or-repeated-segment-is-skipped)
  - [`attachment:a-minted-id-is-stable` — A minted id is stable](#attachmenta-minted-id-is-stable--a-minted-id-is-stable)
  - [`attachment:an-id-is-one-opaque-token` — An id is one opaque token](#attachmentan-id-is-one-opaque-token--an-id-is-one-opaque-token)
  - [`attachment:the-scaffold-is-create-only` — The scaffold is create-only](#attachmentthe-scaffold-is-create-only--the-scaffold-is-create-only)
  - [`attachment:refusals-are-collected-before-exit` — Refusals are collected before exit](#attachmentrefusals-are-collected-before-exit--refusals-are-collected-before-exit)
  - [`attachment:the-scaffold-self-checks-its-emission` — The scaffold self-checks its emission](#attachmentthe-scaffold-self-checks-its-emission--the-scaffold-self-checks-its-emission)
  - [`attachment:attach-takes-exactly-one-form` — Attach takes exactly one form](#attachmentattach-takes-exactly-one-form--attach-takes-exactly-one-form)
  - [`attachment:a-clone-must-agree-on-identity` — A clone must agree on identity](#attachmenta-clone-must-agree-on-identity--a-clone-must-agree-on-identity)
  - [`attachment:attaching-is-idempotent` — Attaching is idempotent](#attachmentattaching-is-idempotent--attaching-is-idempotent)
  - [`attachment:attach-touches-neither-host-nor-remote` — Attach touches neither host nor remote](#attachmentattach-touches-neither-host-nor-remote--attach-touches-neither-host-nor-remote)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

How an invocation finds the plan repository it acts on, and how that repository comes to exist on a machine. Resolution replaces discovery: no verb takes a zone argument, because there is no zone on the invocation's path to point at. The boundary runs at the identity. This domain mints it and resolves through it. The configuration domain says what the two files holding it contain.

## The postfix escalation

An id is derived from the project's name and made unique against this machine's registry by appending postfixes, one at a time, in this order:

```text
1. <slug>                    the project name alone
2. + <parent-scope>          the forge namespace: the user or organisation, or the
                             group and subgroups, slugged
3. + <username>              the operator's forge account name
4. + <forge>                 the forge's own name
5. + NN                      01, 02, ... — last resort only
```

```text
scope != username (acme/payments)      scope == username (gbasso/wipctl)

1. payments                            1. wipctl
2. payments-acme                       2. wipctl-gbasso
3. payments-acme-gbasso                3. skipped — gbasso is already there
4. payments-acme-gbasso-github         4. wipctl-gbasso-github
5. payments-acme-gbasso-github-01      5. wipctl-gbasso-github-01
```

## Where each identity enters resolution

```text
the ordinary path   walk upward to .wipctl.toml for project_id, then find
                    that slug in the attachment registry

where a uid is      resolve as above, then read plan_uid from config.toml
needed              inside each slot and pick the slot that matches
```

`project_id` resolves and `plan_uid` travels. The uid is looked up inside a slot and never names one. There is no tree keyed by it, so a reader never goes looking for one. The configuration domain owns what each value is. This domain owns where each one enters an invocation.

A slot holds one plan, whether it arrived as this machine's own project or as a peer another plan named. Once filled, nothing distinguishes the two. The peer attachment domain owns the walk that fills a peer's slot. The plan targeting domain owns the third resolution path, where an invocation names one of this plan's peers.

## Requirements

### `attachment:resolution-is-two-steps` — Resolution is two steps

When a verb touches a record, the verb MUST resolve the project id by an upward walk, then find it in the attachment registry.

#### Scenario: A verb runs outside any project

- GIVEN a working directory with no identity file at or above it
- WHEN the verb resolves
- THEN it is a usage error, because the invocation named no project. An unattached id is a failed check instead, naming the attachment that fixes it

Verify: `cargo nextest run --test attachment`

### `attachment:a-plan-uid-is-minted-only-at-creation` — A global identity is minted only at creation

When a verb creates a plan, it MUST mint a fresh `plan_uid`, and a verb that adopts one MUST read the uid it carries.

#### Scenario: A record that predates the key is upgraded

- GIVEN a plan repository whose configuration carries no `plan_uid`
- WHEN the operator runs the one explicit upgrade form
- THEN a uid is minted once, committed to the plan trunk, and reported. A second run is refused, because a value other plans can carry is never replaced by a verb

Verify: `cargo nextest run --test attachment`

### `attachment:one-plan-occupies-one-slot` — One plan occupies one slot

When a plan repository is offered whose uid already sits in a slot, the implementation MUST refuse and MUST name both sides.

#### Scenario: A copy is attached beside the plan it was copied from

- GIVEN a copy of a plan repository, carrying its origin's uid
- WHEN it is attached on the machine that already holds the origin
- THEN the refusal names the slot and the location offered, and every slot is left as it was. A copy carries the identity it was copied from, and no verb mints a new one

Verify: `cargo nextest run --test attachment`

### `attachment:git-is-the-only-version-control` — Git is the only version control

The plan repository MUST be a git repository, and the implementation MUST specify replication and the transition commit against git alone.

#### Scenario: A team asks for another version control system

- GIVEN a project whose source lives outside git
- WHEN the plan repository is created
- THEN it is still git, because the record's replication and its commit grammar are specified against git and nothing else

Verify: `cargo nextest run --test journey`

### `attachment:the-registry-is-the-filesystem` — The registry is the filesystem

The attachment registry MUST be the projects tree itself, and the implementation MUST NOT keep a registry file.

#### Scenario: Someone proposes a listing of attached projects

- GIVEN a request for an index of every attachment
- WHEN the tree already answers it
- THEN the index is refused, because it is a second store of state the filesystem owns

Verify: `cargo nextest run --test attachment`

### `attachment:the-scaffold-does-not-walk-upward` — The scaffold does not walk upward

The scaffold MUST NOT walk upward from the working directory.

#### Scenario: The scaffold runs inside a subdirectory of a project

- GIVEN an invocation two directories below a project root
- WHEN the scaffold runs
- THEN it scaffolds where it stands, because a walk scaffolds a parent project by surprise

Verify: `cargo nextest run --test scaffold`

### `attachment:an-id-is-minted-by-postfix-escalation` — An id is minted by postfix escalation

When an id collides at mint time, the implementation MUST append postfixes in priority order until the id is unique on this machine.

#### Scenario: Two projects share a repository name

- GIVEN a second project also named `payments`
- WHEN its id is minted
- THEN the forge namespace is appended first, because uniqueness is decided against this machine and not globally

Verify: `cargo nextest run --test attachment`

### `attachment:an-empty-or-repeated-segment-is-skipped` — An empty or repeated segment is skipped

Where an escalation step's segment is already in the id or does not exist, the implementation MUST skip that step and move to the next.

#### Scenario: A personal repository whose namespace is the account name

- GIVEN `gbasso/wipctl`, minted to `wipctl-gbasso`
- WHEN the next step appends the username a second time
- THEN the step is skipped, because a repeated segment adds no disambiguation, and a project with no forge remote falls through the same way

Verify: `cargo nextest run --test attachment`

### `attachment:a-minted-id-is-stable` — A minted id is stable

Once minted, the id MUST NOT change when the repository, the owner, or the directory is renamed.

#### Scenario: A repository is renamed at the forge

- GIVEN a project whose forge name changes
- WHEN any verb resolves afterwards
- THEN the id is unchanged, because the derivation is a naming convenience at mint time and not a live binding. Renaming the id is its own recorded operation

Verify: `cargo nextest run --test attachment`

### `attachment:an-id-is-one-opaque-token` — An id is one opaque token

A consumer MUST treat a project id as one opaque token and MUST NOT parse it into parts.

#### Scenario: A report wants the forge name

- GIVEN an id minted with a forge postfix
- WHEN a consumer wants that part back
- THEN it reads the forge from the remote rather than from the id, because the parts are known at mint time only

Verify: reviewer confirms no code splits a project id on its separator

### `attachment:the-scaffold-is-create-only` — The scaffold is create-only

The scaffold MUST write a path only where it is absent, MUST report an existing path and leave it alone, and MUST offer no overwrite.

#### Scenario: The scaffold runs twice over a complete project

- GIVEN a project already carrying its identity and plan repository
- WHEN the scaffold runs again
- THEN it writes nothing and succeeds, because a second run is a question and not a command

Verify: `cargo nextest run --test scaffold`

### `attachment:refusals-are-collected-before-exit` — Refusals are collected before exit

When the scaffold's preflight finds more than one refusal, the implementation MUST report every refusal before it exits.

#### Scenario: Two preflight faults are present at once

- GIVEN an existing identity file and a registry slot holding another project
- WHEN the scaffold runs
- THEN both are reported in one run, because fixing one at a time is the loop a collected report removes

Verify: `cargo nextest run --test scaffold`

### `attachment:the-scaffold-self-checks-its-emission` — The scaffold self-checks its emission

After the scaffold emits a zone, the implementation MUST validate that zone with the shipped checker and MUST fail when it does not pass.

#### Scenario: A payload change breaks the emitted zone

- GIVEN a payload edit that makes a lane file invalid
- WHEN the scaffold runs
- THEN the self-check fails the run, because a zone that cannot pass its own gates is worse than no zone

Verify: `cargo nextest run --test scaffold`

### `attachment:attach-takes-exactly-one-form` — Attach takes exactly one form

The attach verb MUST accept exactly one of the four forms the command surface states, and MUST reject any other count as a usage error.

#### Scenario: Two forms are given

- GIVEN a location and the create flag in one invocation
- WHEN the verb parses them
- THEN it is a usage error, because each form answers its own question: adopt a plan, create one, give an identity to a record that predates the key, or fill the slots this plan's peers need

Verify: `cargo nextest run --test verb_contracts`

### `attachment:a-clone-must-agree-on-identity` — A clone must agree on identity

When attach clones a plan repository, the implementation MUST verify the identity agreement, and MUST remove the clone and fail on a mismatch.

#### Scenario: The wrong location is given

- GIVEN a clone whose configuration names another project
- WHEN the check runs
- THEN the clone is removed and the failure names both sides and the choice, because a half-attached slot is a trap for the next verb

Verify: `cargo nextest run --test attachment`

### `attachment:attaching-is-idempotent` — Attaching is idempotent

Where the registry slot already holds this project's plan repository, attach MUST report it, leave it alone, and succeed.

#### Scenario: Attach runs twice on one machine

- GIVEN a slot already filled for this id
- WHEN attach runs again
- THEN nothing changes and the run succeeds, while a slot holding another id fails naming both

Verify: `cargo nextest run --test attachment`

### `attachment:attach-touches-neither-host-nor-remote` — Attach touches neither host nor remote

The attach verb MUST NOT write into the host repository and MUST NOT push.

#### Scenario: A fresh plan repository is created locally

- GIVEN the create form on a project whose host already carries its identity
- WHEN the repository is created
- THEN no remote is configured and the host gains nothing, because hosting is the operator's decision per project

Verify: `cargo nextest run --test attachment`

## Diagnostics

Each resolution failure names its own resolution.

- No identity file on the upward walk. Exit 2, naming the scaffold at the project root.
- The project is not attached on this machine. Exit 1, naming both attach forms.
- The attached plan repository is gone from disk. Exit 1, naming re-attachment.
- The identity files disagree. Exit 1, naming the host value, the plan value, and the choice between re-attaching and correcting whichever file is wrong.
- The scaffold finds an existing identity. Exit 1, reporting the id it found and naming attach for a machine that lacks the plan.
- A clone that is not a plan repository. Exit 1, naming the create form as the way to start one.
- A clone carrying no `plan_uid`. Exit 1, naming the upgrade form that gives a record which predates the key an identity of its own.
- A clone whose uid is already in a slot. Exit 1, naming the slot that holds the plan and the location offered. The message says that one plan gets one slot. A copy meant to be a plan of its own needs an identity of its own, which no verb mints yet.
