# The inbound heading is Reads

## Context and Problem Statement

The two reference headings were `Governed by` and `Amends`: a passive governance phrase paired with an active verb. The asymmetry costs every author a recall step — one heading is remembered, the other looked up — and the passive form undersells what the section is: the list of documents a work session actually loads before starting.

## Considered Options

- Rename `Governed by` to `Reads`.
- Rename `Amends` to `Governed by`'s register instead.
- Keep the pair as is.

## Decision Outcome

Chosen option: `rename Governed by to Reads` — the inbound heading is `Reads`, the outbound heading stays `Amends`. Two present-tense verbs, one per direction: what the session reads, what the work amends. The semantics are unchanged: individual sources stated as claims linked to stable anchors, ungated because the right sources are a judgment about the host's documents.

Moving `Amends` to a passive register instead would have traded the stronger name to rescue the weaker one, and keeping the pair preserves a permanent recall cost to avoid a one-time rename of a heading string in the contract, the templates, and the scaffold payload.

## Consequences

- Good: the pair is symmetric and each heading says what a session does with the section.
- Good: `Reads` states the load obligation directly, which is the section's entire purpose.
- Bad: existing story and epic documents, the gated headings arrays, and the scaffold payload must change together.
- Bad: prose that leaned on "governed" to mean bindingness must now say so explicitly where it matters.

## Status

Accepted
