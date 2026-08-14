# ADR-0003: A lane entry carries a capped summary

## Context and Problem Statement

A lane file should answer a scanning reader without a file open per row, yet must not become a
second specification of the story.

## Considered Options

- A required summary per entry, bounded on both ends
- A title only — too short to answer a scanner; the reader opens the document anyway
- Unbounded prose — differs in kind, not degree: a paragraph becomes a specification and drifts from
  the document
- No summary, boards read documents — every view pays a file read per row, and the lane file answers
  nothing on its own

## Decision Outcome

Chosen option: a required capped summary — long enough to answer a scanner, short enough to stay a
restatement. Every entry carries a required `summary`: one double-quoted line, 60 to 400 characters.
It restates what the work is; the story document stays the source of truth. The schema owns both
bounds.

## Consequences

- Good: boards and rollups render from lane files alone.
- Bad: review carries one ungated duty — that each summary still describes its story.

## Status

Accepted
