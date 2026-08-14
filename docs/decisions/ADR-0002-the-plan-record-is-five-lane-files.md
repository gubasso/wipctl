# ADR-0002: The plan record is five lane files

## Context and Problem Statement

Workflow state and priority need a storage shape that is reviewable, mergeable, and impossible to contradict itself. A record that stores state or rank as data on entries can hold two entries claiming the same slot.

## Considered Options

- Five lane files, where the file is the state and the position is the rank
- A `status` field on entries in one file — two entries can claim the same status order, a lane move is an in-place edit rather than a relocation, and the diff shows a field change instead of the move
- A directory per entry with lane recorded inside — the reviewable diff of a rank change disappears entirely
- Numeric priority fields — two fields can tie, inviting a second ordering rule; a list cannot tie

## Decision Outcome

Chosen option: five lane files — state and order become facts of the filesystem. Five YAML files under `lanes/` — `backlog`, `todo`, `doing`, `review`, `closed` — all present at all times, even when empty. The file is the lane: no `status` field exists. The sequence position is the ranking: no `priority` field exists. An entry is in exactly one lane by construction, because it is in exactly one file.

## Consequences

- Good: state and order are facts of the filesystem, and the checker can hold them without parsing anybody's semantics.
- Bad: a lane move touches two files, and closing may touch a third (`todo.yml` via the repair).

## Status

Accepted
