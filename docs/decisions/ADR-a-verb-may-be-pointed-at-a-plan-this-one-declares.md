# A verb may be pointed at a plan this one declares

## Context and Problem Statement

A peer is a plan, validated in full and read like any other. A caller who wants to act on one has one route: change directory into it. That suits a person at a shell. It does not suit an agent, which must then know where every slot lives.

## Considered Options

- A global flag taking an alias from the invoking plan's own peer table, one hop
- A flag taking a path — rejected: it makes any directory a plan a writer might enter, so a typo reaches the filesystem rather than a file under review
- A path-shaped selector resolving each segment in the plan the previous one named — deferred: it needs a grammar, a per-segment diagnostic, and a cycle rule, and it buys reaching a plan this project never named. Revisit when an operator wants that
- Changing directory, as today — rejected: it makes an agent responsible for a layout the tool resolves

## Decision Outcome

Chosen option: the alias flag. The alias comes from a committed table, so a typo reaches only a plan this record already named and a reviewer already read.

Every verb that resolves a record takes it, writers included, because a lesser mode for a peer is a second idea of a plan. A writer under the flag takes the target's lock and commits to the target's trunk. Four verbs refuse it: `init` never walks, and three others resolve nothing.

This builds on the minted-slug decision, which said nothing about naming another plan. That record is not edited: a merged record keeps its wording, and the spec carries what binds today.

Enforced by `plan-targeting:the-flag-selects-from-this-plans-table`, `plan-targeting:a-verb-that-resolves-nothing-refuses-it`, and `plan-targeting:the-target-owns-the-lock-and-the-commit`.

## Consequences

- Good: an agent acts on any plan this project declared without knowing where its slot lives.
- Bad: a writer can now change another team's record from a shell in this project, and the tool refuses nothing on ownership.

## Status

Accepted
