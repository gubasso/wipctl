# Peer Attachment Specification

<!--TOC-->

- [Purpose](#purpose)
- [The writer](#the-writer)
- [Filling the slots](#filling-the-slots)
- [Requirements](#requirements)
  - [`peer-attachment:the-command-surface-owns-the-table` — The command surface owns the table](#peer-attachmentthe-command-surface-owns-the-table--the-command-surface-owns-the-table)
  - [`peer-attachment:a-clone-is-verified-before-it-fills-a-slot` — A clone is verified before it fills a slot](#peer-attachmenta-clone-is-verified-before-it-fills-a-slot--a-clone-is-verified-before-it-fills-a-slot)
  - [`peer-attachment:the-network-read-is-outside-the-lock` — The network read is outside the lock](#peer-attachmentthe-network-read-is-outside-the-lock--the-network-read-is-outside-the-lock)
  - [`peer-attachment:the-walk-fills-the-declared-set` — The walk fills the declared set](#peer-attachmentthe-walk-fills-the-declared-set--the-walk-fills-the-declared-set)
  - [`peer-attachment:the-walk-remembers-where-it-has-been` — The walk remembers where it went](#peer-attachmentthe-walk-remembers-where-it-has-been--the-walk-remembers-where-it-went)
  - [`peer-attachment:a-peer-occupies-the-slot-its-own-id-names` — A peer occupies the slot its own id names](#peer-attachmenta-peer-occupies-the-slot-its-own-id-names--a-peer-occupies-the-slot-its-own-id-names)
  - [`peer-attachment:a-reference-follows-its-alias` — A reference follows its alias](#peer-attachmenta-reference-follows-its-alias--a-reference-follows-its-alias)
  - [`peer-attachment:a-row-in-use-is-not-removed` — A row in use is not removed](#peer-attachmenta-row-in-use-is-not-removed--a-row-in-use-is-not-removed)
  - [`peer-attachment:the-walk-writes-into-no-peer` — The walk writes into no peer](#peer-attachmentthe-walk-writes-into-no-peer--the-walk-writes-into-no-peer)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

How a peer row comes to exist, and how the plan it names comes to sit in a slot on this machine. The boundary runs at the write. This domain owns the verb that edits the table and the walk that fills the slots. The peers domain owns the table's own shape, and the attachment domain owns resolving this project's own plan.

## The writer

```text
wipctl peer add <url> [--as <alias>]
wipctl peer alias <alias> <new-alias>
wipctl peer url <alias> (--add <url> | --remove <url> | --set <url>...)
wipctl peer remove <alias>
```

The first positional argument is the operation, exactly as `wipctl new <type> "<title>"` takes a type first. So `peer` is one dispatchable unit, the verb list stays derived from the build, and the surface gains no subcommands.

```text
add        clone the plan at the url outside the lock, read its plan_uid
           and its project_id, then write the row. The alias is the peer's
           project_id unless --as gives one.

alias      change a row's alias to a name this project's readers prefer,
           and rewrite every prefixed id that used the old one, in the
           same transaction.

url        --add appends a location. --remove drops one, and is refused
           where it empties the list. --set replaces the whole list in the
           order given, which is how a row is reordered and how a stale
           first locator is taken out.

remove     drop a row, refused while any prefixed id still uses the alias.
```

Two operations carry the weight. `add` makes the uid a machine's problem rather than a person's, which is what lets the design afford an opaque identity at all. `alias` makes the alias genuinely local, because a project renames a peer to whatever its own readers understand.

Every form holds this plan's writer lock and ends in one plan trunk commit, like the writing verbs already named in the command surface. `add` is the one that also clones, so it follows the five steps below. Re-reading the table under the lock is what makes two concurrent additions of one plan a refusal rather than a duplicate row. A failed clone leaves the table byte-identical.

An alias completes from the peer view, in the shape the command surface already requires: derived at completion time, and offering nothing where the source exits non-zero.

## Filling the slots

```text
wipctl attach --peers
```

The verb resolves this project's plan first, as every verb does, then reads its table. No table is exit 0 with one line saying the plan names no peers.

For each row whose uid sits in no slot on this machine, the verb clones from the first url that answers and verifies the clone. The sequence is five steps, because every one of them is a way to get this wrong.

```text
1. clone the url into a temporary directory outside every slot
2. read the clone's config.toml and verify the uid and the project_id
3. take this plan's writer lock, and re-read peers.toml under it
4. promote the temporary directory into the slot, create-only: a slot
   that appeared meanwhile is a lost race, and the loser removes its
   own temporary clone and reports the winner
5. release the lock; the temporary directory is removed on every path,
   including every failure path
```

The reason is the lock's own contract. A writer holds the lock from the deciding read to the commit, and hold times are subsecond. A clone over a network is neither bounded nor subsecond. Cloning straight into the slot is the other way to get this wrong. A reader resolving that uid then sees a directory that is not yet a verified plan.

The walk is transitive and generous. After a peer is attached, its own table is read and every row of it is attached the same way. The walk ends when every plan in the declared set sits in a slot. It fills rows no entry references, because a row this plan declared is a plan this plan meant to work with.

The walk fetches so that no check ever fetches. What it fills is a superset of what validation demands, and the peers domain names both sets.

## Requirements

### `peer-attachment:the-command-surface-owns-the-table` — The command surface owns the table

The implementation MUST offer a verb that adds a row from a location, renames an alias, adds, removes, and replaces locations, and removes a row.

#### Scenario: An operator wants to depend on another team's plan

- GIVEN a plan whose uid the operator has never read
- WHEN the operator names its location
- THEN the verb clones it, reads the uid, and writes the row, because nobody types 32 hexadecimal characters by hand

Verify: `cargo nextest run --test verb_contracts`

### `peer-attachment:a-clone-is-verified-before-it-fills-a-slot` — A clone is verified before it fills a slot

When a clone's configuration declares a uid other than the one its row states, the implementation MUST remove the clone and fail, naming both values.

#### Scenario: A row's location is edited to another team's plan

- GIVEN a row whose url now serves a different plan
- WHEN the walk clones it
- THEN the clone is removed and both uids are named, because a slot filled with the wrong plan is a trap for the next verb

Verify: `cargo nextest run --test attachment`

### `peer-attachment:the-network-read-is-outside-the-lock` — The network read is outside the lock

The implementation MUST clone outside every slot, MUST take the writer lock only after the clone is verified, and MUST fill the slot create-only.

#### Scenario: Two agents attach one peer at the same time

- GIVEN two invocations cloning one plan
- WHEN both reach the promotion step
- THEN one fills the slot and the other reports the winner and removes its own clone, and neither held the lock across the network

Verify: `cargo nextest run --test writer_guarantees`

### `peer-attachment:the-walk-fills-the-declared-set` — The walk fills the declared set

The walk MUST read each attached peer's own table and MUST continue until every plan in the declared set sits in a slot.

#### Scenario: A peer names a plan this record never named

- GIVEN a peer whose own table names a third plan
- WHEN the walk runs
- THEN the third plan is attached too, because a check reads slots and never fetches, so the closure is complete before any proof runs

Verify: `cargo nextest run --test attachment`

### `peer-attachment:the-walk-remembers-where-it-has-been` — The walk remembers where it went

The walk MUST keep a set of the uids it visited and MUST visit each plan once.

#### Scenario: Two plans name each other

- GIVEN two records whose tables each hold a row for the other
- WHEN the walk runs
- THEN it stops, because two plans naming each other is ordinary and a walk with no memory of it never ends

Verify: `cargo nextest run --test attachment`

### `peer-attachment:a-peer-occupies-the-slot-its-own-id-names` — A peer occupies the slot its own id names

An attached peer MUST occupy the slot its own `project_id` names, and a slot already holding another plan MUST be left as it was.

#### Scenario: Two plans minted one slug on two machines

- GIVEN a peer whose project id matches a slot holding a different uid
- WHEN the walk reaches it
- THEN the refusal names both uids and the slot. One side needs a different project id, which the attachment domain names as a recorded operation and no verb performs yet

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

## Diagnostics

Each failure names its resolution. This page owns the message text for the writer and for the walk.

```text
the last url would be removed                                             exit 1
  wipctl: peer 'payments' is left with no url
  wipctl: a row keeps at least one location; add the replacement first,
          or drop the peer with 'wipctl peer remove payments'

a row still in use                                                        exit 1
  wipctl: peer 'payments' cannot be removed: 2 entries still name it
  wipctl:   profile-composition   needs payments#secure-session-storage
  wipctl:   profile-export        needs payments#token-rotation
  wipctl: clear the references first, or rename the alias instead of
          removing the row

the clone declares another uid                                            exit 1
  wipctl: peer 'payments' cloned from
          https://git.example.org/acme/payments-plan.git
  wipctl:   peers.toml expects 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl:   config.toml declares 4c81d0e7f39a4b25861d7c04e9a2f358
  wipctl: the url serves a different plan; fix the url or the uid

two plans minted one slug                                                 exit 1
  wipctl: peer 'payments' cannot be attached: slot payments-acme is taken
  wipctl:   in the slot   4c81d0e7f39a4b25861d7c04e9a2f358
  wipctl:   the peer      9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl: two plans chose the same project_id on two machines; one of
          them needs a different one, which no verb changes yet, so the
          slot stays as it is until that operation exists

every url failed                                                          exit 1
  wipctl: could not clone peer 'payments' from any of 2 urls
  wipctl: <the transport's own error for each, relayed>
  wipctl: check the urls and your access, then re-run
          'wipctl attach --peers'

the plan names no peers                                                   exit 0
  wipctl: this plan names no peers
  wipctl: a peer is declared in peers.toml at the plan zone root; add one
          with 'wipctl peer add <url>'
```
