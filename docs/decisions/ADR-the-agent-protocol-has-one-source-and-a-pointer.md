# The agent protocol has one source and a pointer

## Context and Problem Statement

The zone already carries one self-sufficient travelling document. Copying that method into a host instruction file would create a second source that drifts, while naming its installed path would expose one machine's layout.

## Considered Options

- Point from the host to the command that serves the travelling document — chosen.
- Copy the full method into the host instruction file — rejected: two copies would drift after a method change.
- Point to the travelling document's absolute path — rejected: the location differs between machines.

## Decision Outcome

Chosen option: print a short marked host block that names `wipctl man`. The scaffold asks which host file the operator uses, and the operator places the block because the scaffold did not create that file.

Enforced by `scaffold:the-host-pointer-names-a-command-and-not-a-path`, `scaffold:the-host-pointer-is-placed-where-the-operator-says`, and `metrics:a-marked-row-is-never-acted-on-without-an-answer`.

## Consequences

- Good: the travelling document remains the protocol's single source.
- Good: the pointer works across machines and host layouts.
- Bad: the operator places the printed block into the host file.

## Status

Accepted
