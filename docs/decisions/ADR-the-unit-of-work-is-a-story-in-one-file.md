# The unit of work is a story in one file

## Context and Problem Statement

The method needs a unit that can be agreed, estimated, reviewed, and closed as one thing. It also needs a storage shape that makes each of those acts a readable diff.

## Considered Options

- One story per Markdown document, tasks as an internal checklist
- A task tree, where any node may be scheduled — a unit that may be any node is not a unit; estimation and review lose their object
- One directory per story with multiple files — the diff of an agreement change stops being one file, and the simplest story pays a directory's overhead

## Decision Outcome

Chosen option: one story per Markdown document — the unit and its diff coincide. The unit is a story: one vertical, demonstrable change, in one Markdown document under `stories/`. Tasks live inside the story as a checklist and never become records of their own. Non-narrative artifacts live in a sibling directory named by the story's id.

## Consequences

- Good: everything downstream — points, lanes, transitions, capture — operates on one id naming one document.
- Good: splitting work is an explicit act that produces new stories, never a silent regrouping.
- Bad: work that outgrows its story must be split explicitly; no lighter regrouping exists.

## Status

Accepted
