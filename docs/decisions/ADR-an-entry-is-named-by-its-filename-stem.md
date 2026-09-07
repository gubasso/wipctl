# An entry is named by its filename stem

## Context and Problem Statement

With counters forbidden, the id needs a grammar that is readable, completable, and collision-safe without coordination — and a single place where the name is authoritative.

## Considered Options

- `<slug>-<uid>`, authoritative as the filename stem
- A bare random token — completable but unreadable; the reader learns nothing at a glance
- A bare slug — readable but collision-prone the moment two titles rhyme, with no repair short of renaming
- `<uid>-<slug>` — tab completion works from what a person remembers, which is the words
- Longer uids — sixteen hex characters make filenames unreadable for a collision risk four hex already makes negligible at record scale, and the gate catches the exception

## Decision Outcome

Chosen option: `<slug>-<uid>` as the filename stem — readable words first, a collision guard after, one authoritative home. An id is the title's lowercase-kebab slug plus four lowercase hexadecimal characters. The id is the filename stem of everything it names — story document, epic document, journal, fragment — and the document's title line repeats it, the same pattern as `lane:` repeating the file's basename. No consumer parses an id's interior: no separate `slug` field exists, and nothing derives meaning, order, or date from the characters. Question ids carry the grammar behind a `Q-` prefix.

## Consequences

- Good: a person completes an id from its words, the record library's path arithmetic is trivial (join, stem), and filename-agreement checks become identity checks.
- Bad: four hexadecimal characters leave a small collision risk, caught by the gate rather than prevented by the mint.

## Status

Accepted

Amended by ADR-capture-disjointness-holds-outside-collision — the gate catches claimants that coexist in one record; a same-path collision between clones surfaces as a version-control conflict instead.

Amended by ADR-the-slug-is-the-id — the stem is the title's slug alone, plus at most a qualifying postfix; the four-hex tail is gone, and the filename-as-identity rule is unchanged.
