# The config is TOML at the project root and declares the plan directory

## Context and Problem Statement

The config must both parameterise the tool and anchor discovery: something must mark the project root, and something must place the zone inside it.

## Considered Options

- One hidden TOML file at the project root, declaring `plan_dir`
- Deriving the root from version control — binds the method to a VCS and fails in an exported tree
- A second marker file separate from the config — two files can disagree; one file cannot
- YAML for the config — the record's YAML is a deliberately narrow subset read by the tool's own scanner; the config wants a full-fidelity general parser, and TOML gives every implementation one without inheriting YAML's ambiguities

## Decision Outcome

Chosen option: one hidden TOML file at the root — one file marks the root, places the zone, and cannot disagree with itself. The config is `.wipctl.toml`; the directory holding it is the project root, by definition — nothing is derived upward or downward. It declares `plan_dir`, the root-relative path to the zone, under a strict path grammar. TOML brings an unambiguous specification, native dates and integers, comments, and wide first-party parsing support across languages — the config is read by the tool in every implementation language, where the record's YAML subset is read by the tool's own scanner. The file carries the product's name, not a verb's, because it outlives any one command surface.

## Consequences

- Good: discovery is one upward walk to one filename.
- Good: a stale `plan_dir` is caught where it happened, because the declared path is held against the directory actually checked.
- Bad: the config and the record speak two formats, a split every adopter learns once.

## Status

Accepted

Amended by ADR-a-project-is-identified-by-a-minted-slug — the root file declares `project_id` in place of `plan_dir`; one hidden TOML file still marks the root, and the plan repository carries its own `config.toml`.
