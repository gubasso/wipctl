# ADR-0009: The program exposes one command

## Context and Problem Statement

The product's capabilities could ship as many commands, one command with verbs, or a library first.
Completion, help, and documentation need a root, and a partial build must not claim capabilities it
lacks.

## Considered Options

- One command with capabilities as verbs
- One binary per capability — a namespace of commands to install, complete, and document, with no
  shared conventions enforced by structure
- A library-first design with thin CLIs — the record's consumers are people and hooks; a stable
  command contract serves both, and a library remains an implementation choice inside it

## Decision Outcome

Chosen option: one command, `wipctl`, with capabilities as verbs — one root enforces shared
conventions. The shipped verb list is derived from the build's own dispatch units, never
hard-coded, so `help` cannot claim a verb the build lacks and a new verb joins the surface by
existing. No aliases. Exit codes are uniform across verbs (0 did / 1 check failed / 2 bad
invocation / 3 lock timeout), and stdout carries data while every diagnostic goes to stderr.

## Consequences

- Good: completion, help, and documentation have one root, and partial builds are honest by
  construction — the help's "not in this build" section is derived from what is absent.
- Bad: every capability shares one surface, so a new verb inherits the fixed conventions (exit
  codes, stream split) before its first line is written.

## Status

Accepted
