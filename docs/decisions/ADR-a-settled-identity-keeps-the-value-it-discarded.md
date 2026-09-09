# A settled identity keeps the value it discarded

## Context and Problem Statement

Two replicas can mint different plan identities before either publishes. Another plan can record the losing value before the conflict is settled, and no machine can discover every holder.

## Considered Options

- Prevent publication races without retaining the losing value — rejected because prevention cannot repair a value that another plan already committed.
- Migrate every peer row — rejected because it fixes only reachable consumers. Go ignores a dependency's consumer-side replacement, so the owner must publish the durable answer ([Go module reference](https://go.dev/doc/modules/gomod-ref)).
- Retain the discarded identity beside the canonical identity — chosen because every holder reads the redirect from its owner.

## Decision Outcome

Chosen option: the plan keeps discarded identities in `superseded_plan_ids`. A peer row carrying any retained value resolves, while new rows record `plan_id`.

The list is narrow, append-only, unique, and disjoint across plans. These limits prevent one plan from claiming another plan's identity. Maven uses an owner-published relocation record under old coordinates ([Maven relocation guide](https://maven.apache.org/guides/mini/guide-relocation.html)). Terraform keeps move declarations with configuration history, and removing one is a breaking change ([Terraform module refactoring](https://developer.hashicorp.com/terraform/language/modules/develop/refactoring)).

W3C DID guidance treats equivalent identifiers as a strong assertion that needs guaranteed equivalence and retained values ([DID Core](https://www.w3.org/TR/did/)). This citation is further reading. The limits above state the complete local rule.

Enforced by `configuration:the-current-plan-id-is-not-superseded`, `configuration:superseded-plan-ids-are-append-only`, `configuration:only-mint-settlement-adds-a-superseded-id`, `configuration:plan-identity-claims-are-disjoint`, and `configuration:a-peer-row-resolves-to-one-canonical-plan`.

## Consequences

- Good: every existing holder continues to resolve without an edit.
- Bad: the plan keeps each discarded identity permanently.

## Status

Accepted
