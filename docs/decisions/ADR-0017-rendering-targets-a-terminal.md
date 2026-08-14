# ADR-0017: Rendering targets a terminal

## Context and Problem Statement

A dashboard invites a writer matrix: HTML for sharing, Markdown for wikis, an interactive TUI for
manipulation. Every writer beyond the first is a second implementation to keep true.

## Considered Options

- Terminal text as the only rendering target
- A writer matrix — every panel change multiplies across writers, and each writer is a second
  implementation to keep true
- A TUI — interactivity implies mutation, and mutation has exactly four owners, none of them a
  cursor

## Decision Outcome

Chosen option: terminal text only — one target keeps every panel one implementation. Two layouts
per composite view, one breakpoint at 100 columns, read from the terminal at render time. No HTML,
no image writer, and no TUI — dragging a card is exactly the judgment the method reserves for a
person and a `move` invocation. Every panel opens with its headline fact in prose, so a log file or
screen reader receives the finding before the picture.

## Consequences

- Good: pipe safety becomes a testable contract (no escapes when piped, `NO_COLOR` honoured,
  meaning survives stripping), and sharing a view is copying text — the record's own register.
- Bad: no shareable HTML or wiki rendering exists; a reader elsewhere receives pasted text.

## Status

Accepted
