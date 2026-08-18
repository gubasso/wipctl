# ADR-0041: An Amends item may carry a typed rule delta

## Context and Problem Statement

A host that keeps requirement-level specifications identifies each rule by a stable id of the shape `<domain>:<rule-slug>`. A story that changes such a rule records only the amended document's path, so the change to the rule itself is invisible to a grep: nothing in the record says which rule a story added, reworded, or retired, and traceability from a spec to the work that enacted it requires reading every story.

## Considered Options

- A typed delta clause on the `Amends` item, shape-gated when present.
- A dedicated `Enacts` heading holding rule ids.
- A parallel delta-spec directory per story, mirroring the amended documents.
- Leave rule-level traceability entirely to review.

## Decision Outcome

Chosen option: `a typed delta clause on the Amends item` — an `Amends` assertion may open, after its separator, with `ADDED`, `MODIFIED`, or `REMOVED` followed by one rule id as an inline-code token matching `[a-z0-9-]+:[a-z0-9-]+`. The `new:` marker composes and comes first. The item already names the amended document, so the clause lands where the reader is; a dedicated heading would split one fact across two sections, and a parallel delta directory is derived state beside the record.

The gate checks the clause's shape when one is present and never its presence: the program cannot know which host documents are specifications, and asserting nothing about the host is ADR-0033. Whether a story that changed a rule declared the clause is a review responsibility, stated as such.

## Consequences

- Good: rule-to-story traceability is one fixed-string search over the plan zone.
- Good: hosts without rule ids are untouched; the clause is optional and its absence is legal.
- Bad: a declared clause can lie about the type or the id, and only review catches it.
- Bad: the assertion grammar now has two optional prefixes (`new:`, the delta type) whose order must be specified and gated.

## Status

Accepted
