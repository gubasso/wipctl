# ADR-0030: The program mints the identity

## Context and Problem Statement

The id grammar and the two-file capture shape could be a documented convention a person follows, or
an act the program performs. The invariant that the two files agree spans both files.

## Considered Options

- The program mints, through `wipctl new`
- A documented convention plus template copy — every hand-minted capture is a chance to disagree
  (filename versus fragment id versus title line), and the gate catches it only after the fact
- A hook that repairs a filename after the fact — a writer wired into a gate, which the method
  forbids, repairing what should never have been wrong

## Decision Outcome

Chosen option: the program mints — an invariant spanning two files needs an actor, and a template
cannot hold it. `wipctl new` writes both files — the story document from the shipped template and
the pending fragment beside it — and mints the id itself. The program owns the invariant that the
two files agree with each other and with the grammar.

## Consequences

- Good: the verb's write discipline (create-only, preflight, no overwrite) comes for free.
- Bad: a zone without the tool at hand falls back to the hand-mint incantation the method document
  teaches, stated as the same grammar, without the program's guarantees; the supported path is the
  verb.

## Status

Accepted
