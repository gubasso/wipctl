# Peer Attachment Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`peer-attachment:the-command-surface-owns-the-table` — The command surface owns the table](#peer-attachmentthe-command-surface-owns-the-table--the-command-surface-owns-the-table)
  - [`peer-attachment:a-clone-is-verified-before-it-fills-a-slot` — A clone is verified before it fills a slot](#peer-attachmenta-clone-is-verified-before-it-fills-a-slot--a-clone-is-verified-before-it-fills-a-slot)
  - [`peer-attachment:a-fresh-plan-id-is-provisional` — A fresh plan id is provisional](#peer-attachmenta-fresh-plan-id-is-provisional--a-fresh-plan-id-is-provisional)
  - [`peer-attachment:a-row-identity-comes-from-the-clone` — A row identity comes from the clone](#peer-attachmenta-row-identity-comes-from-the-clone--a-row-identity-comes-from-the-clone)
  - [`peer-attachment:the-network-read-is-outside-the-lock` — The network read is outside the lock](#peer-attachmentthe-network-read-is-outside-the-lock--the-network-read-is-outside-the-lock)
  - [`peer-attachment:a-failed-clone-writes-nothing` — A failed clone writes nothing](#peer-attachmenta-failed-clone-writes-nothing--a-failed-clone-writes-nothing)
  - [`peer-attachment:the-walk-fills-the-declared-set` — The walk fills the declared set](#peer-attachmentthe-walk-fills-the-declared-set--the-walk-fills-the-declared-set)
  - [`peer-attachment:no-table-is-nothing-to-attach` — No table is nothing to attach](#peer-attachmentno-table-is-nothing-to-attach--no-table-is-nothing-to-attach)
  - [`peer-attachment:the-walk-remembers-where-it-has-been` — The walk remembers where it went](#peer-attachmentthe-walk-remembers-where-it-has-been--the-walk-remembers-where-it-went)
  - [`peer-attachment:a-peer-mints-a-local-slot-name` — A peer mints a local slot name](#peer-attachmenta-peer-mints-a-local-slot-name--a-peer-mints-a-local-slot-name)
  - [`peer-attachment:a-reference-follows-its-alias` — A reference follows its alias](#peer-attachmenta-reference-follows-its-alias--a-reference-follows-its-alias)
  - [`peer-attachment:a-row-in-use-is-not-removed` — A row in use is not removed](#peer-attachmenta-row-in-use-is-not-removed--a-row-in-use-is-not-removed)
  - [`peer-attachment:the-walk-writes-into-no-peer` — The walk writes into no peer](#peer-attachmentthe-walk-writes-into-no-peer--the-walk-writes-into-no-peer)
  - [`peer-attachment:a-failure-writes-its-listed-message` — A failure writes its listed message](#peer-attachmenta-failure-writes-its-listed-message--a-failure-writes-its-listed-message)

<!--TOC-->

## Purpose

How a peer row comes to exist, and how the plan it names comes to sit in a slot on this machine. The boundary runs at the write. This domain owns the verb that edits the table and the walk that fills the slots. The peers domain owns the table's own shape, and the attachment domain owns resolving this project's own plan.

## Requirements

### `peer-attachment:the-command-surface-owns-the-table` — The command surface owns the table

The implementation MUST offer a verb that adds a row from a location, renames an alias, adds, removes, and replaces locations, and removes a row.

#### Scenario: An operator wants to depend on another team's plan

- GIVEN a plan whose uid the operator has never read
- WHEN the operator names its location
- THEN the verb clones it, reads the uid, and writes the row, because nobody types 32 hexadecimal characters by hand

Verify: `cargo nextest run --test verb_contracts`

### `peer-attachment:a-clone-is-verified-before-it-fills-a-slot` — A clone is verified before it fills a slot

When a row matches neither `plan_id` nor `superseded_plan_ids`, the implementation MUST remove the clone and fail, naming the values.

#### Scenario: A row's location is edited to another team's plan

- GIVEN a row whose url serves a plan that claims neither its value nor a superseded match
- WHEN the walk clones it
- THEN the clone is removed and both uids are named, because a slot filled with the wrong plan is a trap for the next verb

Verify: `cargo nextest run --test attachment`

### `peer-attachment:a-fresh-plan-id-is-provisional` — A fresh plan id is provisional

When a plan identity is freshly minted, the implementation MUST treat it as provisional until its commit reaches the published plan trunk.

#### Scenario: Another replica publishes first

- GIVEN two replicas that minted different plan identities
- WHEN one push loses the fast-forward race
- THEN its identity remains provisional and replication reports both values for settlement

Verify: `cargo nextest run --test sync`

### `peer-attachment:a-row-identity-comes-from-the-clone` — A row identity comes from the clone

When `peer add` writes a row, the implementation MUST use the canonical `plan_id` read from its clone as identity proof.

#### Scenario: A caller knows a plan identity

- GIVEN a location and an identity supplied outside the clone
- WHEN `peer add` prepares the row
- THEN it ignores the supplied value and records the clone's canonical identity

Verify: `cargo nextest run --test attachment`

### `peer-attachment:the-network-read-is-outside-the-lock` — The network read is outside the lock

The implementation MUST clone outside every slot, MUST take the writer lock only after the clone is verified, and MUST fill the slot create-only.

#### Scenario: Two agents attach one peer at the same time

- GIVEN two invocations cloning one plan
- WHEN both reach the promotion step
- THEN one fills the slot and the other reports the winner and removes its own clone, and neither held the lock across the network

Verify: `cargo nextest run --test writer_guarantees`

### `peer-attachment:a-failed-clone-writes-nothing` — A failed clone writes nothing

When the clone of a peer fails, the implementation MUST leave the peer table byte-identical.

#### Scenario: The url does not answer

- GIVEN `wipctl peer add` naming a url that refuses the connection
- WHEN the clone fails outside the lock
- THEN the table is byte-identical and no row was written, because a row is written only after its clone is verified

Verify: `cargo nextest run --test attachment`

### `peer-attachment:the-walk-fills-the-declared-set` — The walk fills the declared set

The walk MUST read each attached peer's own table and MUST continue until every plan in the declared set sits in a slot.

#### Scenario: A peer names a plan this record never named

- GIVEN a peer whose own table names a third plan
- WHEN the walk runs
- THEN the third plan is attached too, because a check reads slots and never fetches, so the closure is complete before any proof runs

Verify: `cargo nextest run --test attachment`

### `peer-attachment:no-table-is-nothing-to-attach` — No table is nothing to attach

Where the plan configuration declares no peers, the walk MUST exit 0 with one line saying that the plan names no peers.

#### Scenario: A plan with no peers section

- GIVEN a plan configuration without `peers`
- WHEN `wipctl attach --peers` runs
- THEN it prints one line saying the plan names no peers and exits 0, because an absent table is the ordinary state of a plan that names no peers

Verify: `cargo nextest run --test attachment`

### `peer-attachment:the-walk-remembers-where-it-has-been` — The walk remembers where it went

The walk MUST keep a set of the plan identities it visited and MUST visit each plan once.

#### Scenario: Two plans name each other

- GIVEN two records whose tables each hold a row for the other
- WHEN the walk runs
- THEN it stops, because two plans naming each other is ordinary and a walk with no memory of it never ends

Verify: `cargo nextest run --test attachment`

### `peer-attachment:a-peer-mints-a-local-slot-name` — A peer mints a local slot name

When the walk attaches a peer, the walk MUST mint a local slot name from its first url by `slot-naming:a-taken-name-escalates`.

#### Scenario: A peer name is already taken

- GIVEN a peer whose first url gives the name of an occupied slot
- WHEN the walk reaches it
- THEN the walk escalates to a free name by the same ladder used for every other slot

Verify: `cargo nextest run --test attachment`

### `peer-attachment:a-reference-follows-its-alias` — A reference follows its alias

When an alias changes, the implementation MUST rewrite every prefixed dependency that used it, in the same transaction as the table.

#### Scenario: A project renames a peer for its own readers

- GIVEN entries depending on the peer under its old alias
- WHEN the alias changes
- THEN every reference moves with it, because a lane file naming an alias the table lost resolves to nothing

Verify: `cargo nextest run --test verb_contracts`

### `peer-attachment:a-row-in-use-is-not-removed` — A row in use is not removed

Where a prefixed dependency still names an alias, removing that row MUST be refused, naming the entries that use it.

#### Scenario: A peer is dropped while work still depends on it

- GIVEN two entries depending on the peer
- WHEN the row is removed
- THEN the refusal names both entries, exactly as deleting an id is refused while anything names it

Verify: `cargo nextest run --test verb_contracts`

### `peer-attachment:the-walk-writes-into-no-peer` — The walk writes into no peer

The walk MUST NOT write into a peer's repository, MUST NOT push, and MUST NOT touch the host repository.

#### Scenario: A peer's slot is behind its remote

- GIVEN a peer whose slot is older than the plan it names
- WHEN the walk runs over it
- THEN nothing in that slot is changed, because bringing a plan up to date is replication and this verb only fills what is empty

Verify: `cargo nextest run --test attachment`

### `peer-attachment:a-failure-writes-its-listed-message` — A failure writes its listed message

When the peer writer or the attachment walk meets a condition that [the peer attachment messages](../reference/diagnostics.md#peer-attachment-messages) show, the implementation MUST write the message text and exit with the code shown there, which is the one statement of that text for the writer and for the walk.

#### Scenario: The last url of a row would be removed

- GIVEN a peer row with one url
- WHEN the operator removes that url
- THEN the writer exits 1 and names the two ways out, adding the replacement first or dropping the peer, because a row keeps at least one location

Verify: `cargo nextest run --test verb_contracts`
