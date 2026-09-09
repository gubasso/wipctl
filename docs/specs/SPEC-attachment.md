# Attachment Specification

<!--TOC-->

- [Purpose](#purpose)
- [Where each identity enters resolution](#where-each-identity-enters-resolution)
- [Requirements](#requirements)
  - [`attachment:resolution-is-two-steps` — Resolution is two steps](#attachmentresolution-is-two-steps--resolution-is-two-steps)
  - [`attachment:a-plan-uid-is-minted-only-at-creation` — A global identity is minted only at creation](#attachmenta-plan-uid-is-minted-only-at-creation--a-global-identity-is-minted-only-at-creation)
  - [`attachment:one-plan-occupies-one-slot` — One plan occupies one slot](#attachmentone-plan-occupies-one-slot--one-plan-occupies-one-slot)
  - [`attachment:duplicate-project-identities-are-ambiguous` — Duplicate project identities are ambiguous](#attachmentduplicate-project-identities-are-ambiguous--duplicate-project-identities-are-ambiguous)
  - [`attachment:git-is-the-only-version-control` — Git is the only version control](#attachmentgit-is-the-only-version-control--git-is-the-only-version-control)
  - [`attachment:the-registry-is-the-filesystem` — The registry is the filesystem](#attachmentthe-registry-is-the-filesystem--the-registry-is-the-filesystem)
  - [`attachment:the-scaffold-does-not-walk-upward` — The scaffold does not walk upward](#attachmentthe-scaffold-does-not-walk-upward--the-scaffold-does-not-walk-upward)
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

## Where each identity enters resolution

```text
the ordinary path   walk upward to .wipctl/project.toml, read project_id,
                    then find the slot whose plan file declares that value

where a plan id     read plan_id from the peer row, then find the slot whose
is given            plan file declares that value
```

The search reads `projects/*/plan-repo/.wipctl/plan.toml`, where both identities sit at a fixed depth. Two slots claiming one project are an ambiguity and never a choice.

The directory that resolution opens carries a slot name, as [the slot naming domain](./SPEC-slot-naming.md) defines. Neither identity appears in a path.

A slot holds one plan, whether it arrived as this machine's own project or as a peer another plan named. Once filled, nothing distinguishes the two. The peer attachment domain owns the walk that fills a peer's slot. The plan targeting domain owns the third resolution path, where an invocation names one of this plan's peers.

## Requirements

### `attachment:resolution-is-two-steps` — Resolution is two steps

When a verb touches a record, it MUST walk to the project id and find the one slot whose plan configuration declares it.

#### Scenario: A verb runs outside any project

- GIVEN a working directory with no `.wipctl/project.toml` at or above it
- WHEN the verb resolves
- THEN it is a usage error, because the invocation named no project. An unattached id is a failed check instead, naming the attachment that fixes it

Verify: `cargo nextest run --test attachment`

### `attachment:duplicate-project-identities-are-ambiguous` — Duplicate project identities are ambiguous

Where two slots declare one `project_id`, the implementation MUST fail resolution and name both slots and both plan identities.

#### Scenario: A plan repository is copied into another slot

- GIVEN `payments` and `payments-old` declaring one project identity
- WHEN a verb resolves that project
- THEN it chooses neither and tells the operator to detach the copy

Verify: `cargo nextest run --test attachment`

### `attachment:a-plan-uid-is-minted-only-at-creation` — A plan identity is minted only at creation

When a verb creates a plan, it MUST mint a fresh `plan_id`, and a verb that adopts one MUST read the identity it carries.

#### Scenario: A record that predates the key is upgraded

- GIVEN a plan repository whose configuration carries no `plan_id`
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

### `attachment:the-scaffold-is-create-only` — The scaffold is create-only

The scaffold MUST write a path only where it is absent, MUST report an existing path and leave it alone, and MUST offer no overwrite.

#### Scenario: The scaffold runs twice over a complete project

- GIVEN a project already carrying its project file and plan repository
- WHEN the scaffold runs again
- THEN it writes nothing and succeeds, because a second run is a question and not a command

Verify: `cargo nextest run --test scaffold`

### `attachment:refusals-are-collected-before-exit` — Refusals are collected before exit

When the scaffold's preflight finds more than one refusal, the implementation MUST report every refusal before it exits.

#### Scenario: Two preflight faults are present at once

- GIVEN an existing project file and a registry slot holding another project
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

- No `.wipctl/project.toml` on the upward walk. Exit 2, naming the scaffold at the project root.
- A `.wipctl` directory on the walk with no `project.toml` in it. Exit 2, naming that directory. The walk reports and stops rather than passing over it. Passing over it attaches the operator to a project further up the tree.
- The project is not attached on this machine. Exit 1, naming both attach forms.
- The attached plan repository is gone from disk. Exit 1, naming re-attachment.
- The two configuration files disagree. Exit 1, naming the host value, the plan value, and the choice between re-attaching and correcting whichever file is wrong.
- The scaffold finds an existing project file. Exit 1, reporting the id it found and naming attach for a machine that lacks the plan.
- A clone that is not a plan repository. Exit 1, naming the create form as the way to start one.
- A clone carrying no `plan_id`. Exit 1, naming the upgrade form that gives a record which predates the key an identity of its own.
- A clone whose uid is already in a slot. Exit 1, naming the slot that holds the plan and the location offered. The message says that one plan gets one slot. A copy meant to be a plan of its own needs an identity of its own, which no verb mints yet.

```text
two slots claim one project                                       exit 1
  wipctl: two slots hold a plan declaring this project
  wipctl:   payments      plan 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl:   payments-old  plan 4c81d0e7f39a4b25861d7c04e9a2f358
  wipctl: one project has one plan; detach the copy, and keep the plan
          whose plan_id the peers around you name
```
