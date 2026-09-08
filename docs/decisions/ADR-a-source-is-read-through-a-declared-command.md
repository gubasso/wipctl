# A source is read through a declared command

## Context and Problem Statement

The view has to list a declared source's open items, and every candidate system speaks a different protocol. The specification is language-agnostic and names no third-party tool as a requirement, so it cannot name one here either. It still has to say how the items arrive.

## Considered Options

- `a list command the project declares, emitting a documented shape` — chosen.
- `a built-in client per system` — rejected: it names concrete third-party services in the specification, ships credentials handling, and dates the moment a system changes its interface.
- `a plugin interface` — rejected: the architecture has no plugin system and no state beyond the four stated locations, and one read does not earn either.

## Decision Outcome

Chosen option: `a list command the project declares, emitting a documented shape` — the specification states the contract, and any command meeting it serves. The command is an argument vector, so no shell parses it, and it is spawned once per run rather than loaded into the process. A missing or failing command is an optional dependency going absent: one line on the error stream names the degradation, every source that answered still reports, and the view never gates. The command is discovered from the project's own declaration file, so the shipped dependency manifest gains no row per system and the specification names no forge, vendor, or ticket system.

Enforced by `external-sources:the-view-runs-each-declared-command` and `external-sources:a-missing-source-command-degrades-loudly`.

## Consequences

- Good: the specification stays buildable in any language against any system, and the tool learns nothing about a forge.
- Bad: the project supplies the command, so two projects reading one system write that command twice. Getting the items is the operator's problem, and the report is the tool's.

## Status

Accepted
