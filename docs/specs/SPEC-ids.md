# Ids Specification

<!--TOC-->

- [Purpose](#purpose)
- [Grammar and slugification](#grammar-and-slugification)
- [An id and a prefixed id](#an-id-and-a-prefixed-id)
- [Requirements](#requirements)
  - [`ids:one-id-names-one-thing` — One id names one thing](#idsone-id-names-one-thing--one-id-names-one-thing)
  - [`ids:a-title-and-its-id-always-match` — A title and its id always match](#idsa-title-and-its-id-always-match--a-title-and-its-id-always-match)
  - [`ids:slugification-is-stated-once` — Slugification is stated once](#idsslugification-is-stated-once--slugification-is-stated-once)
  - [`ids:the-id-is-the-filename` — The id is the filename](#idsthe-id-is-the-filename--the-id-is-the-filename)
  - [`ids:an-id-is-opaque-to-every-consumer` — An id is opaque to every consumer](#idsan-id-is-opaque-to-every-consumer--an-id-is-opaque-to-every-consumer)
  - [`ids:a-prefixed-id-is-split-once` — A prefixed id is split once](#idsa-prefixed-id-is-split-once--a-prefixed-id-is-split-once)
  - [`ids:the-mint-checks-under-the-lock` — The mint checks under the lock](#idsthe-mint-checks-under-the-lock--the-mint-checks-under-the-lock)
  - [`ids:a-postfix-is-passed-never-guessed` — A postfix is passed, never guessed](#idsa-postfix-is-passed-never-guessed--a-postfix-is-passed-never-guessed)
  - [`ids:a-postfix-qualifies-a-held-base` — A postfix qualifies a held base](#idsa-postfix-qualifies-a-held-base--a-postfix-qualifies-a-held-base)
  - [`ids:a-counter-is-forbidden-across-machines` — A counter is forbidden across machines](#idsa-counter-is-forbidden-across-machines--a-counter-is-forbidden-across-machines)
  - [`ids:a-freed-id-is-burned` — A freed id is burned](#idsa-freed-id-is-burned--a-freed-id-is-burned)
  - [`ids:a-burned-id-is-refused-at-the-mint` — A burned id is refused at the mint](#idsa-burned-id-is-refused-at-the-mint--a-burned-id-is-refused-at-the-mint)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

How an entry, an epic, an initiative, and a question are named, and what happens when two of them want one name. The boundary runs at allocation. This domain owns the grammar, the mint, and what a freed name owes the record. The transition journal domain owns the tombstone that holds a burned name.

## Grammar and slugification

```text
[a-z0-9]+(-[a-z0-9]+)*
```

A question id carries the same grammar behind the fixed prefix `Q-`.

```text
1. lowercase
2. apostrophes are removed rather than hyphenated      don't -> dont
3. every other run of characters outside a-z0-9 becomes a single hyphen
4. leading and trailing hyphens are trimmed
```

```text
id  ==  slugify(title)                     the ordinary case
id  ==  slugify(title) + "-" + <postfix>   where the slug alone was taken
```

## An id and a prefixed id

A dependency can name an entry in another plan, so two shapes appear where one used to.

```text
an id             one token; never parsed; names one thing in one record

a prefixed id     an alias, a separator, and an id; split exactly once on
                  the first separator; the alias half goes to the peer
                  table and the id half is carried whole
```

Exactly one function in an implementation performs that split. Nothing else in the code splits a dependency value. A second splitter is how the two halves come to disagree.

After the split, the id half is an ordinary id under the ordinary rule. So the opacity rule below survives the addition: it is the id that is never parsed, and a prefixed id is a pair that was never one id.

## Requirements

### `ids:one-id-names-one-thing` — One id names one thing

One id MUST name one thing across the whole record, for the whole life of the record.

#### Scenario: A story and an epic want one name

- GIVEN an epic already holding a slug
- WHEN a story's title slugifies to it
- THEN the second is refused, because a reference that resolves to two things resolves to neither

Verify: `cargo nextest run --test validation`

### `ids:a-title-and-its-id-always-match` — A title and its id always match

An id MUST equal its title's slug, optionally followed by a qualifying postfix, and MUST NOT diverge from its title in any other way.

#### Scenario: A title is edited in place

- GIVEN a document whose title is reworded
- WHEN the checker runs
- THEN the change is a rename with its own verb, not a drift a reader has to tolerate

Verify: `cargo nextest run --test validation`

### `ids:slugification-is-stated-once` — Slugification is stated once

The implementation MUST use one slugification function for the mint, the rename, and the gate.

#### Scenario: Three consumers compute the same string

- GIVEN an apostrophe in a title
- WHEN each consumer slugifies it
- THEN all three remove it rather than hyphenate it, because three implementations of one function is how an id and a title come to disagree

Verify: `cargo nextest run --test ids`

### `ids:the-id-is-the-filename` — The id is the filename

Every document, journal, and fragment MUST take its id as its filename stem, checked in both directions.

#### Scenario: An agent knows a title and wants the path

- GIVEN a title with no qualifying postfix
- WHEN the agent slugifies it
- THEN the path is guessable, and where a postfix disambiguated the id the id verb reports it

Verify: `cargo nextest run --test validation`

### `ids:an-id-is-opaque-to-every-consumer` — An id is opaque to every consumer

A consumer MUST treat an id as one token and MUST NOT parse it into parts.

#### Scenario: A tie-break compares two ids

- GIVEN two entries tied on every ordering key
- WHEN the residual tie-break runs
- THEN it compares the full id lexically as one string, because the id carries no order, date, rank, or interior meaning

Verify: `cargo nextest run --test ids`

### `ids:a-prefixed-id-is-split-once` — A prefixed id is split once

A prefixed dependency MUST be split once, on the first separator, by one function of the implementation.

#### Scenario: A view wants the alias for a heading

- GIVEN a report grouping dependencies by the plan they name
- WHEN it splits the value itself
- THEN two splitters disagree on the first edge case, so the report asks the one function instead

Verify: reviewer confirms one function splits a prefixed dependency

### `ids:the-mint-checks-under-the-lock` — The mint checks under the lock

When the implementation mints an id, it MUST read the live entries and every tombstone inside the lock, and MUST refuse a held id.

#### Scenario: A title collides with an entry in a planning lane

- GIVEN a slug already held by a live entry
- WHEN the mint runs
- THEN it refuses naming the holder and the lane, and offers rephrasing and an explicit id as the two resolutions

Verify: `cargo nextest run --test ids`

### `ids:a-postfix-is-passed-never-guessed` — A postfix is passed, never guessed

The implementation MUST NOT append a qualifying postfix on the operator's behalf.

#### Scenario: A title is genuinely right as written

- GIVEN a collision the operator does not want to resolve by rephrasing
- WHEN they pass an explicit id
- THEN it mints through the same checks, because a qualifying postfix is a naming judgment and no id is chosen by a machine

Verify: `cargo nextest run --test ids`

### `ids:a-postfix-qualifies-a-held-base` — A postfix qualifies a held base

Where an id carries a qualifying postfix, the id without it MUST be held by a live artifact or by a tombstone.

#### Scenario: A postfix is added to a free slug

- GIVEN an id whose base slug nothing holds
- WHEN the mint runs
- THEN it is refused, and the same case fails at the gate. That is what keeps the postfix a disambiguation rather than a second naming convention

Verify: `cargo nextest run --test ids`

### `ids:a-counter-is-forbidden-across-machines` — A counter is forbidden across machines

The implementation MUST NOT allocate an id from a counter where allocation is concurrent across machines.

#### Scenario: Two machines mint while apart

- GIVEN two machines capturing the same title
- WHEN they reconcile
- THEN the collision is surfaced and never prevented, and the recovery renames the losing capture. Inside one machine the lock makes a numeric postfix legal

Verify: `cargo nextest run --test sync`

### `ids:a-freed-id-is-burned` — A freed id is burned

When a rename frees an id, the implementation MUST burn that id with a tombstone, unless the rename resolves a cross-machine collision.

#### Scenario: A collision recovery renames the losing capture

- GIVEN a surviving capture that still holds the id on a replica
- WHEN the losing capture is renamed
- THEN no tombstone is left, because a reference to that id still resolves to exactly one thing

Verify: `cargo nextest run --test ids`

### `ids:a-burned-id-is-refused-at-the-mint` — A burned id is refused at the mint

An entry, epic, initiative, or fragment MUST NOT claim an id a tombstone holds, and the mint MUST refuse it at capture time.

#### Scenario: A deleted title is proposed again

- GIVEN an id held by a deletion tombstone
- WHEN the mint runs
- THEN it refuses naming the tombstone and its date. The tombstone occupies the id-named path, so releasing the slug loses the fact of the deletion

Verify: `cargo nextest run --test ids`

## Unenforced rules

| Rule                                    | Why no command decides it                                                          |
| --------------------------------------- | ---------------------------------------------------------------------------------- |
| `ids:a-postfix-is-passed-never-guessed` | Whether a passed postfix distinguishes anything for a reader is a naming judgment. |
| `ids:a-prefixed-id-is-split-once`       | No command tells one splitter from two that happen to agree today.                 |

Rephrasing is the expected resolution for a collision: two titles that slug identically usually want different names anyway. A postfix answers a collision inside one record, and a rephrase answers one between two.
