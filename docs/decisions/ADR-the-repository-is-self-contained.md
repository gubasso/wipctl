# The repository is self-contained

## Context and Problem Statement

The method and the specification were drafted against canonical documents held outside this repository, on a personal machine and under paths no contributor or agent can resolve. A clone that cannot be understood, built, or gated without those paths is not a shippable product, and this project asks its clients to keep their own plan record complete in exactly the same way.

## Considered Options

- Cite the external canonical documents as the load-bearing source and link to them.
- Hold every load-bearing fact in-repo and allow an outbound link only as a citation.
- Vendor the external documents wholesale into the repository.

## Decision Outcome

Chosen option: `hold every load-bearing fact in-repo` — the repository states what it depends on, so a clone on a machine with nothing else still explains itself, builds, and gates. Borrowed substance is rewritten into the zone that owns it rather than referenced, which also forces it through the language-agnostic constraint the reference carries. Vendoring was rejected because a copied document arrives with a second voice, its own history, and no owner here, and it decays without anyone noticing. The rule and its test are stated in `docs/specs/SPEC-quality-gates.md`; `AGENTS.md` points at that statement for every agent entering the repository.

## Consequences

- Good: a clone is complete, the product practises the self-sufficiency it sells, and no reader is sent to a path that only one machine has.
- Bad: restating borrowed substance costs authoring effort, and a rewritten fact can drift from the external source it came from with nothing to detect it.

## Status

Accepted
