# Delay cost names the ordering fact

## Context and Problem Statement

The `class` field records when waiting harms an entry, but its name does not state that fact. Its values mix an instruction, a category, and an economic label.

The field needs plain language that a reader understands without knowledge of another planning method. Each stored value must add information.

## Considered Options

- `delay_cost` with `immediate`, ordinary absence, and `deferred`. Chosen.
- `urgency` with high, normal, and low. Rejected because urgency reads as absolute priority and can appear to override eligibility.
- `ordering_preference` with earlier and later. Rejected because it stores rank manipulation instead of a planning fact.
- Keep class-of-service terminology. Rejected because the name does not explain the recorded fact.

## Decision Outcome

Chosen option: `delay_cost` with `immediate`, ordinary absence, and `deferred`.

`immediate` means waiting causes material harm now. `deferred` means the harm appears later, remains uncertain, or is negligible now. Ordinary work omits the field, so every stored value states an exception.

The `delay-cost` ranking preference places immediate first, absence second, and deferred last. Delay cost never overrides eligibility or dependency order.

Enforced by `lane-file:delay-cost-records-an-exception`, `lane-file:absent-delay-cost-is-ordinary`, `ranking:a-delay-cost-preference-orders-before-size`, `rendering:delay-cost-is-a-glyph`, and `messages:multiple-immediate-delay-costs-name-the-count`.

## Consequences

- Good: the field and values state their meaning in plain language.
- Good: ordinary entries carry no redundant neutral value.
- Bad: adopters must replace or remove every `class` field.

## Status

Accepted

Supersedes [ADR-a-class-of-service-is-the-one-stated-key.md](./ADR-a-class-of-service-is-the-one-stated-key.md).
