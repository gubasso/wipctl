# ADR-0058: A qualifying postfix is part of the id, and burning follows the freed id

## Context and Problem Statement

ADR-0051 makes the title's slug the id and resolves a collision by an operator-chosen postfix; ADR-0052 states that a title and its id always match. Two questions are left open: whether a postfixed id satisfies the invariant, and what a rename burns when the id it leaves is not free.

## Considered Options

- The postfix is part of the id, and burning follows whether the id was freed
- Strict equality with no postfix — removes the explicit-id flag, leaving rephrasing as the only recovery, which ADR-0051 rejected as the sole option
- Burn unconditionally — the surviving capture would claim a burned id, so the collision recovery fails the gate it must pass

## Decision Outcome

Chosen option: the postfix is part of the id. An id is its title's slug, optionally followed by a qualifying postfix — the whole permitted difference between a title and its id; the checker verifies the title slugifies to the id, or to the id minus that postfix. A postfix qualifies something in its own record — the id without it is live or tombstoned — so it disambiguates rather than becoming a second naming convention, and the record already holds what the check needs, since an id is freed only by a rename that burns it. That bounds the mechanism: recovering a cross-machine collision frees the base without burning it, and the capture keeping it has not reconciled in, so the result would carry a postfix qualifying nothing. That case rephrases instead. The postfix answers a collision inside one record; a rephrase answers one between two.

Burning follows the freed id. A rename burns the old id because a reference would otherwise resolve to nothing; one that frees nothing — the collision case — burns nothing, having no stale reference to protect.

## Consequences

- Good: `rename` is the recovery at both scales, and the title line stays checkable.
- Bad: a filename is guessable only in the unpostfixed case, and the checker carries one more rule.

## Status

Accepted

Amends ADR-0024, ADR-0051, and ADR-0052.
