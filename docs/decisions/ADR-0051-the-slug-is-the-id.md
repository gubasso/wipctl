# ADR-0051: The slug is the id; nothing random is minted

## Context and Problem Statement

The four-hex suffix existed for exactly one reason: allocation was unobservable. Two clones with no shared state could not ask each other what the next id was, so the mint drew randomness and collisions were surfaced rather than prevented. With one live record behind one machine-level lock (ADR-0045, ADR-0047), a mint can read the record it is about to write into, and the reason is gone.

## Considered Options

- The slug alone is the id, checked at mint time under the lock
- Keep the suffix — carries no meaning, defends against a race the lock now prevents, and makes no filename guessable
- A machine-appended disambiguator on collision — a name chosen by a machine; a qualifying postfix is a naming judgment

## Decision Outcome

Chosen option: the slug is the id, for every artifact kind. The grammar is the slug grammar; question ids are `Q-<slug>`. The mint checks under the lock, against the live record and every tombstone, and refuses a taken or burned id before the first byte, suggesting a rephrase; a qualifying postfix is passed explicitly, never appended by the tool. Id opacity becomes absolute — the drain's uid tie-break, the one stated exemption, is replaced by a residual tie on the full id compared lexically. Between machines allocation is still concurrent: two machines can mint one slug, and the collision surfaces at `sync` as a conflict naming both sides, recovered by rephrasing one title.

## Consequences

- Good: filenames become guessable, ids become legible, and project ids and entry ids share one philosophy.
- Bad: cross-machine slug collisions are likelier than matching hex draws ever were, and the mint now needs the lock.

## Status

Accepted

Supersedes ADR-0030. Re-scopes ADR-0026 and ADR-0038; amends ADR-0029.
