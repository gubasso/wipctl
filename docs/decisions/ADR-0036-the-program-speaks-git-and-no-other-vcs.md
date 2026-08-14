# ADR-0036: The program speaks git and no other version control

## Context and Problem Statement

The opt-in transition commit needs a version-control interface. Operators increasingly layer other
tools over git; the program must decide what it will ever invoke.

## Considered Options

- Speak git exactly where the transition commit is enabled, and invoke nothing else, ever
- Probing for and preferring a companion — the program would take sides in the operator's tooling,
  and every companion behaviour difference becomes a support surface
- Speaking no VCS at all — forfeits the transition commit, which enabled projects value and the
  fixed message shape makes safe

## Decision Outcome

Chosen option: git and no other — one interface, invoked in exactly one opt-in place. The program
speaks git where the config enables the transition commit, and never invokes any other
version-control tool — not as a runtime dependency, not as an optional one, not from a hook or
recipe it ships. A git-compatible companion an operator layers on top is their own choice; the
method may document such a companion as optional, hazards first, and never names it in the
host-assumption contract. This is deliberately not an ADR-0016 optional dependency: no probe
exists, no view degrades, nothing is served.

## Consequences

- Good: the dependency surface of every verb except the opt-in commit is zero version-control
  tools.
- Bad: one hazard transfers to documentation — a companion that bypasses git hooks weakens
  commit-stage gates, which is why the record gates register at the push stage as well.

## Status

Accepted
