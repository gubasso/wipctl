# Fixed stages frame ranking preferences

## Context and Problem Statement

The ranking configuration lists eligibility, dependencies, preferences, and the final ID comparison as peers. A project cannot remove or reorder three of those names, so the list exposes product invariants as false choices.

The configuration needs to show only decisions the project can make. The procedure must still produce one valid, deterministic order.

## Considered Options

- Fix constraints and the final tie-break while configuring preferences alone. Chosen.
- Configure every ordering stage. Rejected because invalid stage orders can place blocked work first or leave entries tied.
- Fix the entire procedure. Rejected because projects need to choose whether delay cost or size matters first.

## Decision Outcome

Chosen option: fix constraints and the final tie-break while configuring preferences alone.

The procedure applies close date where present, eligibility, and same-lane dependency order. It then applies `[ranking].preferences`. The full ID resolves the final tie. A project can order or omit `delay-cost` and `points-ascending`, or declare an empty list.

Enforced by `configuration:the-ranking-section-is-required`, `ranking:constraints-lead-and-are-fixed`, `ranking:a-preference-name-is-known-and-appears-once`, `ranking:the-ranking-procedure-is-total`, and `ranking:the-final-tie-break-is-the-full-id`.

## Consequences

- Good: configuration contains only project choices.
- Good: invalid constraint orders cannot be represented.
- Good: an empty preference list states no preference without weakening validity or determinism.
- Bad: adopters must replace `[ranking].keys` with `[ranking].preferences`.

## Status

Accepted

Supersedes [ADR-an-order-is-computed-and-never-stored.md](./ADR-an-order-is-computed-and-never-stored.md).
