# Peers Specification

<!--TOC-->

- [Purpose](#purpose)
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

The table a plan keeps of the other plans it names. A local alias names each peer, and the peer's declared plan identity identifies it. This domain owns the section, keys, and shape.

## Requirements

### `peers:the-view-answers-from-the-table-and-the-slots` — The view answers from the table and the slots

The peer view MUST list every declared row, name whether each is attached and at which revision, group this record's prefixed dependencies under the peer they name with each target's lane, mark an unused row stale, and open no url.

#### Scenario: The plan names no peers

- GIVEN a record with no table
- WHEN the view runs
- THEN it succeeds and says so. Naming no peers is the ordinary case, and a verb that fails on the ordinary case teaches the wrong thing

Verify: `cargo nextest run --test verb_contracts`

### `peers:a-row-carries-a-uid-and-a-location` — A row carries a uid and a location

Every peer row MUST carry `plan_id` and non-empty `urls`, use the canonical identity when written, and resolve a superseded identity when read.

#### Scenario: A row is written with a location and no plan identity

- GIVEN a row naming a location and no identity
- WHEN the record is checked
- THEN it fails because the location is a hint beside the identity that the peer declares

Verify: `cargo nextest run --test schemas`

### `peers:an-alias-is-local-and-unique` — An alias is local and unique

Every alias MUST parse under the slug grammar and MUST be unique within the section.

#### Scenario: Two rows use one alias

- GIVEN two rows keyed by `payments`
- WHEN the section is read
- THEN the duplicate is refused because an alias resolves to one row in this plan

Verify: `cargo nextest run --test validation`

### `peers:one-row-names-one-other-plan` — One row names one other plan

Each `plan_id` MUST appear in at most one row, and a row's value MUST differ from this plan's own `plan_id`.

#### Scenario: A row is added under the alias `mirror`

- GIVEN a row whose plan identity is this plan's own
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

Every url MUST write out the `https` or the `ssh` scheme, MUST carry nothing in its userinfo but a bare account name, so a colon or a percent sign there is refused rather than redacted, and MUST reach the replication substrate exactly as the file states it.

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
a peer row with no plan_id                                                exit 1
  wipctl: peer 'payments' declares no plan_id
  wipctl: a peer is identified by its plan_id, not by its url; read the
          identity from that plan's plan file and add it to the peers
          section

a peer row with an empty url list                                         exit 1
  wipctl: peer 'payments' declares no url
  wipctl: a peer nobody can locate is not a peer; add at least one
          location with 'wipctl peer url payments --add <url>'

a row naming this plan                                                    exit 1
  wipctl: peer 'mirror' names this plan
  wipctl:   plan_id 9f2c41a08b7d4e63a15c8f02d7e4b619 is this plan's own
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

two rows claiming one plan_id                                             exit 1
  wipctl: peers 'payments' and 'billing' name one plan
  wipctl:   plan_id 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl: one plan gets one alias; drop the row you do not use

the view runs against a plan with no table                                exit 0
  wipctl: this plan names no peers
  wipctl: a peer is declared in the peers section of .wipctl/plan.toml;
          see 'wipctl help peers'
```
