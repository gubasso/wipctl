# Peers Specification

<!--TOC-->

- [Purpose](#purpose)
- [The section](#the-section)
- [Two sets, named once](#two-sets-named-once)
- [What the table is not](#what-the-table-is-not)
- [The view](#the-view)
- [Requirements](#requirements)
  - [`peers:the-view-answers-from-the-table-and-the-slots` — The view answers from the table and the slots](#peersthe-view-answers-from-the-table-and-the-slots--the-view-answers-from-the-table-and-the-slots)
  - [`peers:a-row-carries-a-uid-and-a-location` — A row carries a uid and a location](#peersa-row-carries-a-uid-and-a-location--a-row-carries-a-uid-and-a-location)
  - [`peers:an-alias-is-local-and-unique` — An alias is local and unique](#peersan-alias-is-local-and-unique--an-alias-is-local-and-unique)
  - [`peers:one-row-names-one-other-plan` — One row names one other plan](#peersone-row-names-one-other-plan--one-row-names-one-other-plan)
  - [`peers:the-table-is-earned` — The table is earned, never scaffolded](#peersthe-table-is-earned--the-table-is-earned-never-scaffolded)
  - [`peers:a-url-is-used-as-written` — A url is used as written](#peersa-url-is-used-as-written--a-url-is-used-as-written)
  - [`peers:no-check-opens-a-url` — No check opens a url](#peersno-check-opens-a-url--no-check-opens-a-url)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

The table a plan keeps of the other plans it names. A peer is another project's plan repository, named here by a local alias and identified by the uid that plan declares. The boundary runs at the table. This domain owns the section, its keys, and its shape. The configuration domain owns the file the section sits in. The peer attachment domain owns the verb that writes a row and the walk that fills a peer's slot. The validation domain owns the checks that read the table.

## The section

The `peers` section sits in `.wipctl/plan.toml` at the plan zone root. There is one table per peer, keyed by the alias.

```toml
[peers.payments]
uid = "9f2c41a08b7d4e63a15c8f02d7e4b619"
urls = ["https://git.example.org/acme/payments-plan.git"]

[peers.platform]
uid = "4c81d0e7f39a4b25861d7c04e9a2f358"
urls = [
  "https://git.example.org/acme/platform-plan.git",
  "https://git.example.net/mirrors/platform-plan.git",
]
```

```text
<alias>    the table key. The slug grammar, [a-z0-9]+(-[a-z0-9]+)*. Local
           to this plan and meaningful nowhere else.

uid        REQUIRED. The peer's plan_uid, 32 lowercase hexadecimal
           characters. This is the identity, and everything else is a hint.

urls       REQUIRED. A non-empty array of locations where the peer's plan
           repository can be cloned. Order is the order the walk tries them.
```

Two schemes are accepted for a peer url, `https` and `ssh`, and each MUST be written out. A bare account name in the userinfo is allowed, because it names an account. A colon there is where a password goes, so a url carrying one is refused rather than redacted. The percent sign is refused there too, because `%3A` carries the same colon past a check that reads the url as written. A committed file is read by everyone who clones this plan, so a credential in it is a leak whatever the tool prints.

A peer has three names, and they are three separate facts. The alias is chosen here. The uid is declared by the peer, and it never changes. The url is a locator that changes when the plan moves host, and changing it changes nothing else.

This table is the only place an alias resolves. A dependency written as `<alias>#<id>` reads the alias here, in the record the dependency was written in, and never in any other plan's table. The shape is the `peers` half of `plan.schema.json`, in the configuration spec's companion directory.

## Two sets, named once

```text
the declared set   every row of this plan's peers section, plus every row
                   of each of their peers sections, transitively. This is
                   what the peer attachment walk fills.

the read set       every peer a prefixed needs id in this record reaches,
                   plus the peers those peers' own prefixed ids reach,
                   transitively. This is what validation requires attached.
```

The read set is a subset of the declared set. The difference is exactly the rows nothing references. Those rows are legal, the walk fills them, and validation reports them stale rather than demanding them.

The split follows the two jobs. Attaching is an operator saying which plans they work with, so it is generous. Checking is a proof, so it demands what it reads and no more. A machine that attached less than everything then gets a short list rather than a wall.

## What the table is not

- The table is not a registry. Nothing allocates an alias, nothing approves a uid, and no service is consulted. Two plans that name one peer write two independent rows.
- The table is not authoritative about the peer. The peer's own `.wipctl/plan.toml` is, and the walk refuses a clone that disagrees with the row.
- The table is not a cache. Reading a peer is a read of that peer's slot, and no peer state is stored here.

## The view

```text
wipctl peers [--json]
```

The view reads the table, this record's five lane files, and the slot of each attached peer.

```text
$ wipctl peers
payments   9f2c41a08b7d4e63a15c8f02d7e4b619   attached  91a47c0d
           https://git.example.org/acme/payments-plan.git
           2 entries need it
             profile-composition   needs secure-session-storage   todo
             profile-export        needs token-rotation           closed

platform   4c81d0e7f39a4b25861d7c04e9a2f358   not attached
           https://git.example.net/mirrors/platform-plan.git
           run 'wipctl attach --peers'
```

The contract states five things. The view lists every declared peer. It names whether each is attached and at which revision. It groups this record's prefixed dependencies under the peer they name, with the lane each target sits in. It marks a row no dependency uses as stale. It never fetches, which the page says out loud, because a reader who sees a location expects a network call.

`peers` is a reader in the family of `epics` and `initiatives`, and a view belongs beside the views. The health report names whether each declared peer is attached and stops there. A report that repeats a view is a second copy of it.

The machine format is `peer-list.schema.json`, in the companion directory of this spec. A field is absent rather than null where the state does not have it, which is the rule an entry's optional fields already follow. Stale is derived from the reference list rather than stated beside it, so the schema holds the two in agreement.

A prefixed dependency that resolves to no entry has no message on this page. The validation domain owns that condition and answers it three ways from the peer's own tombstones. A fourth, vaguer message here gives one condition two answers, and the vaguer one is the one a reader meets first.

## Requirements

### `peers:the-view-answers-from-the-table-and-the-slots` — The view answers from the table and the slots

The peer view MUST list every declared row, name whether each is attached and at which revision, and mark an unused row stale.

#### Scenario: The plan names no peers

- GIVEN a record with no table
- WHEN the view runs
- THEN it succeeds and says so. Naming no peers is the ordinary case, and a verb that fails on the ordinary case teaches the wrong thing

Verify: `cargo nextest run --test verb_contracts`

### `peers:a-row-carries-a-uid-and-a-location` — A row carries a uid and a location

Every peer row MUST carry a `uid` of 32 lowercase hexadecimal characters and a non-empty `urls` array, and MUST carry no other key.

#### Scenario: A row is written with a location and no uid

- GIVEN a row naming a location and no identity
- WHEN the record is checked
- THEN it fails, because a peer is identified by the uid its own plan declares and a location is a hint beside it

Verify: `cargo nextest run --test schemas`

### `peers:an-alias-is-local-and-unique` — An alias is local and unique

Every alias MUST parse under the slug grammar, MUST be unique within the section, and MUST NOT equal this plan's own `project_id`.

#### Scenario: A plan gives a peer the name it calls itself

- GIVEN a row keyed by this plan's own project id
- WHEN a prefixed dependency uses that alias
- THEN the string has two candidate meanings, so the alias is refused and the ambiguity is closed

Verify: `cargo nextest run --test validation`

### `peers:one-row-names-one-other-plan` — One row names one other plan

Each uid MUST appear in at most one row, and a row's uid MUST NOT equal this plan's own `plan_uid`.

#### Scenario: A row is added under the alias `mirror`

- GIVEN a row whose uid is this plan's own
- WHEN the record is checked
- THEN it fails, because the alias rule stops one spelling of a self-reference and this rule stops every other

Verify: `cargo nextest run --test validation`

### `peers:the-table-is-earned` — The table is earned, never scaffolded

The table MUST be created when a plan first names a peer, and its absence MUST mean that the plan names no peers.

#### Scenario: A scaffolded plan is read for the first time

- GIVEN a plan repository the scaffold created whole
- WHEN a reader opens it
- THEN no empty table is deleted before the record can be believed, which is how the earned directories already behave

Verify: `cargo nextest run --test scaffold`

### `peers:a-url-is-used-as-written` — A url is used as written

Every url MUST carry an accepted scheme, MUST NOT carry credentials, and MUST reach the replication substrate exactly as the file states it.

#### Scenario: A machine rewrites one location into another

- GIVEN a host configuration that rewrites a url before the transport opens it
- WHEN the walk runs
- THEN the rewritten url is not the one under review, so the tool relies on no such rewriting

Verify: `cargo nextest run --test validation`

### `peers:no-check-opens-a-url` — No check opens a url

A check MUST answer from the table and from the slots on this machine, and MUST open no url.

#### Scenario: A reader sees a location in the table and expects a fetch

- GIVEN a record naming two peers
- WHEN validation runs
- THEN no connection is opened, because every answer is a function of committed files and a fetch makes it a function of the hour

Verify: `cargo nextest run --test validation`

## Diagnostics

Each failure names its resolution. The validation domain owns the check catalog, and this page owns the message text for the table's own shape.

```text
a peer row with no uid                                                    exit 1
  wipctl: peer 'payments' declares no uid
  wipctl: a peer is identified by its plan_uid, not by its url; read the
          uid from that plan's plan file and add it to the peers
          section

a peer row with an empty url list                                         exit 1
  wipctl: peer 'payments' declares no url
  wipctl: a peer nobody can locate is not a peer; add at least one
          location with 'wipctl peer url payments --add <url>'

a row naming this plan                                                    exit 1
  wipctl: peer 'mirror' names this plan
  wipctl:   uid 9f2c41a08b7d4e63a15c8f02d7e4b619 is this plan's own
  wipctl: a peer is another plan; a dependency inside this record is a
          bare needs id, so remove the row

a url carrying credentials                                                exit 1
  wipctl: peer 'payments' declares a url with credentials in it
  wipctl:   https://<redacted>@git.example.org/acme/payments-plan.git
  wipctl: a committed file is read by everyone who clones this plan;
          remove the credentials from the url and let the transport get
          them from the machine

a url with an unaccepted scheme                                           exit 1
  wipctl: peer 'payments' declares a url wipctl will not open
  wipctl:   ext::sh -c 'anything'
  wipctl: a peer url carries one of the schemes the specification accepts;
          correct the row, and the url is not attempted meanwhile

two rows claiming one uid                                                 exit 1
  wipctl: peers 'payments' and 'billing' name one plan
  wipctl:   uid 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl: one plan gets one alias; drop the row you do not use

the view runs against a plan with no table                                exit 0
  wipctl: this plan names no peers
  wipctl: a peer is declared in the peers section of .wipctl/plan.toml;
          see 'wipctl help peers'
```
