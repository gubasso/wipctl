# Ids

One id names one thing — a story, an epic, an initiative, or a question — across the whole record, for the whole life of the record.

## Grammar

The id is the title's slug, plus at most a qualifying postfix where that slug was already taken:

```text
[a-z0-9]+(-[a-z0-9]+)*
```

Examples: `rate-limit-the-search-endpoint`, `session-hardening`, `trustworthy-by-default`. Question ids carry the same grammar behind a fixed prefix: `Q-<slug>`, for example `Q-what-identifies-a-caller`.

## Slugification, stated once

One function turns a title into its id's slug, specified here and used by the mint, by `rename`, and by the gate — three consumers computing the same string three ways is how an id and a title come to disagree in a system that claims they cannot:

```text
1. lowercase
2. apostrophes are removed rather than hyphenated      don't -> dont
3. every other run of characters outside a-z0-9 becomes a single hyphen
4. leading and trailing hyphens are trimmed
```

## A title and its id always match

An id is its title's slug, optionally followed by a qualifying postfix, for a story, an epic, an initiative, and a question:

```text
id  ==  slugify(title)                     the ordinary case
id  ==  slugify(title) + "-" + <postfix>   where the slug alone was taken
```

The postfix is the disambiguation below, and it is the whole of the permitted difference: an id MUST NOT diverge from its title in any other way. So a title and its id can never drift apart, changing a title changes the id, and that is a supported operation with its own verb ([../cli/verbs/rename.md](../cli/verbs/rename.md)) rather than a drift a reader has to tolerate. The document title line `# <id> — <short title>` is therefore an invariant the checker verifies, not a repetition (see [documents.md](./documents.md)).

## The id is the filename

A story's document is `stories/<id>.md`; an epic's is `epics/<id>.md`; an initiative's is `initiatives/<id>.md`; a journal is `journal/<id>.tsv`; a fragment is `pending/<id>.yml`. The filename stem and the id are one fact, checked against each other in both directions. Because the id is the title's slug, filenames are guessable in the ordinary case: an agent that knows a title knows the path, unless a qualifying postfix disambiguated the id, which `ids` reports.

## Opacity

A consumer MUST NOT parse an id into parts. The record library, the checker, the schemas, and shell completion all treat the id as one token. The id carries no order, no date, no rank, and no meaning in its interior; anything that wants the title reads the document. There is no exemption: the drain's residual tie-break compares the full id lexically as one string ([../cli/verbs/land.md](../cli/verbs/land.md)).

## The mint checks under the lock

Allocation on a machine is observable: one live record sits behind one transaction lock, so a mint MUST read the record it is about to write into — the live entries and every tombstone — inside the lock, and MUST refuse a taken or burned id before the first byte, naming the holder and the resolution:

```text
$ wipctl new story "Rate limit the search endpoint"
wipctl: id rate-limit-the-search-endpoint is taken by an entry in todo
wipctl: rephrase the title, or pass --id rate-limit-the-search-endpoint-v2
```

Rephrasing is the expected resolution: two titles that slug identically usually want different names anyway. A qualifying postfix — a number, `v2`, anything that distinguishes — is the fallback when the title is genuinely right as written, and it is passed, never guessed: the tool MUST NOT append a postfix on the operator's behalf, because a qualifying postfix is a naming judgment and no id is chosen by a machine (`new --id`, and `rename` with an explicit id, both mint through the same checks).

A postfix MUST qualify something in this record. Where an id carries one, `slugify(title)` — the id without it — MUST be held by a live artifact or by a tombstone; a postfix on an id whose base slug is free is refused at the mint and is a failure at the gate. The record holds everything this needs: an id is freed only by a rename, which burns it, so a base slug that was ever taken is still either live or tombstoned. This is what keeps the postfix a disambiguation rather than a second naming convention.

It also bounds where the postfix applies. A cross-machine collision is the one case where a postfix cannot qualify, and the reason is what the rename leaves behind rather than what it starts from: the losing capture holds the base slug now, but the recovery frees it without burning it — the surviving capture keeps it, on a replica that has not reconciled in — so the committed result would carry a postfixed id whose base is neither live nor tombstoned here. That recovery rephrases instead ([../cli/verbs/rename.md](../cli/verbs/rename.md)). The postfix answers a collision inside one record; a rephrase answers one between two.

A counter is still forbidden where allocation is concurrent (ADR-0026): within a machine the lock makes allocation serial, so a qualifying numeric postfix is legal there; between machines allocation is still concurrent, and two machines can mint the same slug. That collision is surfaced by reconciliation, never prevented — `sync` reports it naming both sides, and the recovery is `rename` on the losing capture, the one rename that leaves no tombstone because the surviving capture still holds the id ([../cli/verbs/rename.md](../cli/verbs/rename.md)).

The one excluded counter is the decision record sequence (`ADR-NNNN`), allocated serially by one person merging one decision at a time.

## Burned ids

A deleted entry leaves a tombstone: a journal whose last event targets `deleted`; a renamed entry leaves one whose last event targets `renamed` (see [transition-journal.md](./transition-journal.md)). Burning is what a freed id owes the record: a reference to it would otherwise resolve to nothing. So the one rename that leaves no tombstone is the one that frees nothing — a rename resolving an id collision `sync` reported, where the surviving capture still holds the id and a reference to it still resolves to exactly one thing ([../cli/verbs/rename.md](../cli/verbs/rename.md)). Otherwise the id is burned for the life of the record — an entry, epic, initiative, or fragment claiming an id a tombstone holds is a failure naming both sides, and the mint refuses it at capture time in the same words as any other taken id:

```text
$ wipctl new story "Fix the parser"
wipctl: id fix-the-parser is held by a tombstone: deleted 2026-08-30
wipctl: that id is burned for the life of the record; rephrase the title
```

Burning is decided on a mechanical basis, not a philosophical one: a tombstone occupies the id-named path `journal/<id>.tsv`, so releasing the slug would mean deleting the tombstone — losing the fact that a deletion happened — or relocating tombstones out of the id-named path, a storage change made to buy back a title somebody can simply rephrase. Burning is what makes "one id names one thing" hold across the record's whole life rather than only its present.
