# A plan change and a code change are no longer one commit

## Context and Problem Statement

While the record lived in the host's working tree, a story's closing transition and the code that delivered it could land in one commit, and the acceptance transfer could be gated within one change. Moving the record to its own repository (ADR-the-plan-record-lives-in-its-own-repository) makes that impossible, and the loss should be recorded rather than buried.

## Considered Options

- Accept the loss: cross-reference explicitly, and carry the same-change guarantee as a review obligation
- Keep the record in the host tree — reverses ADR-the-plan-record-lives-in-its-own-repository and its reasons
- A two-repository commit protocol — no gate can span two repositories; a protocol that pretends one can fails silently, which is worse than a stated obligation

## Decision Outcome

Chosen option: accept the loss and state the linkage. An entry gains optional code references — the branch carrying the work, and the commit that delivered it — written by a person, opaque to every check beyond shape, because the record cannot verify another repository's contents. The acceptance-transfer doctrine survives: a story is a diff, a durable document is a state. Its same-change guarantee becomes a same-unit-of-review obligation carried by the review checklist — what the gate checks, it names; what it cannot check, it assigns.

## Consequences

- Good: the linkage is explicit and searchable instead of implicit in a commit boundary, and it survives squashes and rebases that would have destroyed the implicit form.
- Bad: nothing mechanical proves a closed story's code landed, or that the transfer happened in the same unit of review — two review responsibilities where one gate used to stand.

## Status

Accepted
