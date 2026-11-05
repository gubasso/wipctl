# The push stage runs only what the commit stage does not

## Context and Problem Statement

The hooks of this repository ran every cheap check at both the commit stage and the push stage, so that a commit made with its hooks skipped still met the gate before it was published. CI ran the same two stages, and beside them three jobs that no hook ran: the full test suite, the release build, and the package build. A push that passed every hook could still fail CI, and every push repeated the commit stage for nothing.

## Considered Options

- `the split` — chosen.
- `the both-stage default` — rejected: CI runs both stages on every request, so a commit that skipped its hooks still meets every gate before it merges, and the repetition buys nothing.
- `keep the CI-only jobs` — rejected: a check that runs only in CI fails after a green push, and the contributor learns of the defect a round trip later.

## Decision Outcome

Chosen option: `the split` — the commit stage runs the cheap checks once, the push stage runs the costly checks once, and the two together run every check CI runs.

The push stage holds the integration tests, the release build, the package build, the package verification of the publish, and the networked checks. CI runs the two stages in the development environment and nothing beside them. A check that only CI can run is listed in `README.md` with its reason.

The plan repository the product scaffolds keeps its record gates at both stages, because it has no CI and its push stage is its last gate.

Enforced by `quality-gates:the-local-stages-hold-every-ci-check`.

## Consequences

- Good: a push that passed its hooks passes CI, and no check runs twice on one push.
- Good: CI holds two jobs and a gate, and a new check means a new hook alone.
- Bad: a commit made with its hooks skipped meets the cheap checks only in CI, after the push.
- Bad: the push stage takes longer, because the builds moved into it.

## Status

Implemented

The stages of `.pre-commit-config.yaml` and the jobs of `.github/workflows/ci.yml` enact it.
