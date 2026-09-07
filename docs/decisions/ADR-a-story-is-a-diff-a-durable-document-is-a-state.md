# A story is a diff, a durable document is a state

## Context and Problem Statement

The plan zone is allowed to become false as work moves — that is what makes it a plan. Something must therefore carry the truth forward, and it cannot be the story.

## Considered Options

- Stories freeze at close; their truth transfers into the host's durable documents
- Stories as living documentation — every closed story becomes a page to keep true, and the documentation's truth is scattered across a history
- Generated documentation from stories — the generator would need to know which assertions survived later stories, which is exactly the judgment the transfer performs

## Decision Outcome

Chosen option: a story is a diff, a durable document is a state — truth moves forward by transfer. When a story closes, its acceptance assertions are rewritten in the present tense into the documents its `Amends` names, in the same change as the behaviour. The story then freezes as history, keeping its original acceptance. Durable documents carry no inverse list of the stories that shaped them. A spike exits through a decision record, a story revision, or a closed measurement — never by leaving its findings in the spike.

## Consequences

- Good: the zone can be archived, pruned, or ignored by a reader who only wants the truth — the documents have it.
- Bad: the transfer is the one step no tool can see, so it lives in the review checklist by name.

## Status

Accepted
