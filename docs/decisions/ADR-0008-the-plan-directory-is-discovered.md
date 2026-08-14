# ADR-0008: The plan directory is discovered

## Context and Problem Statement

Requiring every invocation to name the zone taxes the common case: an operator standing anywhere
inside their project.

## Considered Options

- Upward discovery to the nearest config, with one optional positional override
- A `--zone` flag — asks the same question as the positional twice, and flags on every invocation
  are the tax discovery exists to remove
- An environment variable — invisible state; two terminals answer differently with no diff to
  review

## Decision Outcome

Chosen option: upward discovery with a positional override — the common case costs nothing and the
exception stays explicit. Every reading verb discovers the zone by walking upward from the working
directory to the nearest `.wipctl.toml`, then joining `plan_dir`. One optional positional argument
overrides the start of the walk, which is how a verb reaches another project's zone. Finding no
config is exit 2 (the invocation named no project); a config that is incomplete or disagrees with
the directory given is exit 1 (a check failed). `init` alone never walks — a scaffold that walked
would write into a parent project by surprise.

## Consequences

- Good: `wipctl next` works from any depth, and the positional stays an override, so scripts remain
  explicit while humans stay unencumbered.
- Bad: a verb's answer depends on where the walk starts; outside any project the operator gets exit
  2, never a guess.

## Status

Accepted
