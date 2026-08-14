# ADR-0022: A transition commit is generated and opt-in

## Context and Problem Statement

Projects that commit per transition want the message uniform; projects that batch or review differently want no commit at all. The method must not demand a git practice.

## Considered Options

- An opt-in commit with a fixed message shape
- A message template language — a template is a program in the config; the two facts a message needs (the transition, the project's convention prefix) fit a fixed shape
- Committing by default — a default git practice exceeds the host assumption and surprises every project that batches
- Bypassing hooks for speed — the transition commit would become the one unguarded write path in a project that guards everything

## Decision Outcome

Chosen option: generated and opt-in — the commit exists only where the config asks for it, in one fixed shape. `move` commits only when the config carries a `commit` table, with both `type` and `scope` required and no defaults. The message shape is fixed, not templated: `<type>(<scope>): move <id> from <from> to <to>` and `<type>(<scope>): close <id> as <outcome>`. No body, no trailers. Only the three written paths are staged; the project's hooks always run — there is no verification bypass. `--no-commit` reproduces the absent-table behaviour for one invocation.

## Consequences

- Good: transition history in enabled projects is uniform and searchable.
- Bad: a hook can refuse the commit, leaving the record written and the paths staged — stated plainly, so no state is hidden.

## Status

Accepted
