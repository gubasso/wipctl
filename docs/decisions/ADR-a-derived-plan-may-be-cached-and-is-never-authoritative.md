# A derived plan may be cached and is never authoritative

## Context and Problem Statement

A session wants the resolved epic plan as a file it can be pointed at; the charter forbids a second store.

## Considered Options

- An explicit flag writing a disposable cache that no verb reads back
- Writing the plan into the zone — a second store the gate would have to police for staleness
- No file at all — consumers re-run the verb and parse a pipe; workable, but a file a session can hold is materially more useful and costs nothing if it lies, because nothing believes it

## Decision Outcome

Chosen option: a disposable cache behind an explicit flag — a file a session can hold, that nothing believes. `epic --write` writes the resolved plan to the user cache directory and prints the path. Four properties keep it from being a store: rewritten on every call, read back by no verb, required by no check, and never committed. A missing, stale, or corrupt cache changes no answer and fails nothing. The cache path embeds a digest of the project root so two same-named projects cannot collide.

## Consequences

- Good: the record stays the single source of truth while sessions get a stable artifact.
- Bad: any consumer that starts trusting the cache is out of contract by definition.

## Status

Accepted
