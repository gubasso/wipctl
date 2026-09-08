# The scaffold writes only what it creates

## Context and Problem Statement

`init` writes into a project it has never touched. Anything it overwrites or merges into is someone else's file.

## Considered Options

- A create-only, preflighted scaffold
- Merging into host files — requires parsing another project's config formats, and the correct hook set is unknowable at scaffold time
- A `--force` overwrite — turns a scaffold into a reset; the destructive path deserves a deliberate act, not a flag
- Seeding a first story — it would have to be deleted before the record could be believed

## Decision Outcome

Chosen option: a create-only scaffold — nothing it did not create is ever touched. The scaffold holds five guarantees: deterministic; payload-faithful (byte identical to the shipped payload but for an enumerated substitution set); create-only — an existing path is reported and left alone, with no `--force`; additive — it never removes; and preflighted — every refusal is collected before the first byte. Content destined for host-owned files (hook entries) is printed for a person to place, never merged.

## Consequences

- Good: a second run over a complete zone writes nothing and exits 0, which makes `init` safely re-runnable and makes `--print-hooks` a stable way to regenerate wiring.
- Bad: hook wiring is always a manual placement; the scaffold never finishes that last step itself.

## Status

Accepted
