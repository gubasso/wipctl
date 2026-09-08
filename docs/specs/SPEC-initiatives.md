# Initiatives Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`initiatives:an-initiative-is-reported-never-executed` — An initiative is reported, never executed](#initiativesan-initiative-is-reported-never-executed--an-initiative-is-reported-never-executed)
  - [`initiatives:membership-is-searched-from-the-epics` — Membership is searched from the epics](#initiativesmembership-is-searched-from-the-epics--membership-is-searched-from-the-epics)
  - [`initiatives:every-entry-is-counted-once` — Every entry is counted once](#initiativesevery-entry-is-counted-once--every-entry-is-counted-once)
  - [`initiatives:work-outside-the-tier-is-outside-the-arithmetic` — Work outside the tier is outside the arithmetic](#initiativeswork-outside-the-tier-is-outside-the-arithmetic--work-outside-the-tier-is-outside-the-arithmetic)
  - [`initiatives:an-unjoined-initiative-appears-at-zero` — An unjoined initiative appears at zero](#initiativesan-unjoined-initiative-appears-at-zero--an-unjoined-initiative-appears-at-zero)
  - [`initiatives:a-bare-invocation-lists` — A bare invocation lists](#initiativesa-bare-invocation-lists--a-bare-invocation-lists)
  - [`initiatives:the-rollup-reports-no-verdict` — The rollup reports no verdict](#initiativesthe-rollup-reports-no-verdict--the-rollup-reports-no-verdict)
  - [`initiatives:the-decomposition-is-derived-every-call` — The decomposition is derived on every call](#initiativesthe-decomposition-is-derived-every-call--the-decomposition-is-derived-on-every-call)

<!--TOC-->

## Purpose

Decomposing one initiative into its member epics, and rolling every initiative up into one row each. An initiative is reported and never executed: the entry-level execution plan stays the epic resolution's answer and has exactly one home. The boundary runs at the tier: this domain owns the third tier, and the epics domain owns the arithmetic every tier shares.

## Requirements

### `initiatives:an-initiative-is-reported-never-executed` — An initiative is reported, never executed

The decomposition MUST report one row per member epic and MUST NOT resolve entries into eligible and blocked groups.

#### Scenario: A reader wants work from an initiative

- GIVEN a decomposition listing three member epics
- WHEN the reader wants entries
- THEN they run the epic resolution against one of the rows, because execution has exactly one home

Verify: `cargo nextest run --test initiatives`

### `initiatives:membership-is-searched-from-the-epics` — Membership is searched from the epics

The implementation MUST search membership from the epic documents' initiative sections, and the initiative document MUST hold no member list.

#### Scenario: An epic joins an initiative

- GIVEN an epic gaining an initiative section
- WHEN the decomposition runs
- THEN the member appears with no edit to the initiative document, because a member list goes stale on the first change

Verify: `cargo nextest run --test initiatives`

### `initiatives:every-entry-is-counted-once` — Every entry is counted once

An initiative's numbers MUST be its member epics' numbers summed, computed by the shared implementation.

#### Scenario: Two epics of one initiative share nothing

- GIVEN entries whose membership flows only through their epic
- WHEN the tiers are totalled
- THEN every entry is counted at most once at every tier, with no deduplication step to get wrong

Verify: `cargo nextest run --test initiatives`

### `initiatives:work-outside-the-tier-is-outside-the-arithmetic` — Work outside the tier is outside the arithmetic

An entry carrying no epic MUST be outside every initiative's arithmetic, and a prerequisite from outside the initiative MUST NOT be counted or shown.

#### Scenario: A member epic waits on outside work

- GIVEN a prerequisite belonging to no member epic
- WHEN the decomposition renders
- THEN it is absent, because the epic resolution is where an outside prerequisite surfaces, marked by what it serves

Verify: `cargo nextest run --test initiatives`

### `initiatives:an-unjoined-initiative-appears-at-zero` — An unjoined initiative appears at zero

An initiative no epic joined MUST appear at zero rather than being omitted, and an absent initiative directory MUST print nothing and succeed.

#### Scenario: A project uses two tiers

- GIVEN a record with no initiative directory
- WHEN the rollup runs
- THEN it prints nothing and succeeds, because the absence is legal and means two tiers

Verify: `cargo nextest run --test initiatives`

### `initiatives:a-bare-invocation-lists` — A bare invocation lists

A bare invocation of the decomposition verb MUST list the initiative ids and MUST NOT fall back to the rollup.

#### Scenario: A reader asks which initiatives exist

- GIVEN a bare invocation
- WHEN it runs
- THEN it lists the ids, and each verb's usage names the other, which is the same pairing the epic tier carries

Verify: `cargo nextest run --test verb_contracts`

### `initiatives:the-rollup-reports-no-verdict` — The rollup reports no verdict

The rollup MUST NOT report a staleness verdict or retirement advice.

#### Scenario: An initiative has nothing open

- GIVEN an initiative whose member epics are all closed out
- WHEN the rollup renders
- THEN it states the numbers alone, because retirement is a review question asked alongside whether the charter is still true

Verify: `cargo nextest run --test initiatives`

### `initiatives:the-decomposition-is-derived-every-call` — The decomposition is derived on every call

The decomposition MUST be derived on every call and MUST be stored nowhere in the record.

#### Scenario: A stored rollup is proposed

- GIVEN a request to keep the numbers in the zone
- WHEN a member closes
- THEN the stored copy is wrong, so the decomposition is computed each time

Verify: `cargo nextest run --test initiatives`
