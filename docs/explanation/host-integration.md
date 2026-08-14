# Host integration

## The single assumption

wipctl requires that the host project has a documentation directory. That is the entire
assumption. It asserts nothing about how that directory is organised, whether the project keeps
decision records, or what kind of document an outbound reference names. The config keys have no
defaults for this reason, and no default may ever be added. Anything that resolves is valid;
whether it was the right document is a review responsibility, stated as such.

The project root is the directory holding `.wipctl.toml`; `plan_dir` places the zone; the
scaffold writes exactly those two things and owns exactly one document inside the zone it
created: `AGENTS.md`, the method itself. Host-owned files are read at most; content intended for
them is printed for a person to place.

## Two directions of reference

- `Governed by` — inbound. The individual sources a work session must load before starting.
  Unchecked by the gate — entirely a review responsibility — because the right sources are a
  judgment about the host's documents.
- `Amends` — outbound. The documents the work must leave changed. Gated: each path is relative,
  carries no dot segments, and resolves against the project root — unless its assertion opens
  with `new:`, which promises a document the work will create and exempts existence for the life
  of the entry. Both directions may point anywhere in the project, not only the documentation
  directory.

## A story is a diff, a durable document is a state

The doctrine the whole integration rests on: a story describes a change; the host's documents
describe what is true. When a story closes, its acceptance assertions are rewritten in the
present tense into the amended documents, in the same change as the behaviour. The story then
freezes as history; the durable document carries the truth forward and keeps no inverse list of
the stories that shaped it.

No gate can see whether the transfer happened — a resolving path is not a kept promise. The
review checklist carries it instead, and the method is honest about that boundary: what the gate
checks, it names; what it cannot check, it assigns.

## What the gate checks here, and what it does not

Gated: `Amends` shape and resolution; config presence, completeness, and agreement with the
directory checked. Not gated, and stated as review responsibilities: whether a named document is
the right one; whether the acceptance transfer reached the amended document; whether a `new:`
promise was kept; every `Governed by` path.
