# Host integration

## The single assumption

wipctl asks the host project to carry one committed file: `.wipctl.toml` at its root, naming the project's identity. That is the entire footprint. The plan itself lives in the project's own plan repository, resolved through that identity ([../reference/record/resolution.md](../reference/record/resolution.md)); the host's layout, its document kinds, its conventions, and its history stay untouched — no verb commits to the host, ever. The config has no defaults for anything a project must declare, and no default may ever be added.

The scaffold writes exactly that one file into the host and owns the plan repository it creates, the method document `AGENTS.md` included. Host-owned files are read at most; content intended for them is printed for a person to place.

## Two directions of reference

Stories reference the host's documents across the two repositories, and the paths resolve against the host's project root — the directory holding `.wipctl.toml`:

- `Reads` — inbound. The individual sources a work session must load before starting. Unchecked by the gate — entirely a review responsibility — because the right sources are a judgment about the host's documents. A host whose documentation method excludes a class of document from that load set, as several exclude decision records so a session reads the rule that binds rather than the argument that shaped it, excludes it here too: this method adds no constraint of its own and lifts none.
- `Amends` — outbound. The documents the work must leave changed. Gated: each path is relative, carries no dot segments, and resolves against the project root — unless its assertion opens with `new:`, which promises a document the work will create and exempts existence for the life of the entry. Both directions may point anywhere in the project, not only its documentation.

A host that keeps requirement-level specifications identifies each rule by an id of the shape `<domain>:<rule-slug>`. An `Amends` assertion may then carry a typed delta — `ADDED`, `MODIFIED`, or `REMOVED` plus the rule id in inline code — so rule-to-story traceability is one fixed-string search over the plan zone. The gate checks a declared clause's shape and never its presence, for the same reason `Reads` is unchecked: which host documents keep rule ids is a fact about the host.

## A story is a diff, a durable document is a state

The doctrine the whole integration rests on: a story describes a change; the host's documents describe what is true. When a story closes, its acceptance assertions are rewritten in the present tense into the amended documents, together with the behaviour. The story then freezes as history; the durable document carries the truth forward and keeps no inverse list of the stories that shaped it.

Plan and code no longer share commits, so the record states the linkage explicitly instead: an entry may carry the branch that implements it and, once closed, the commit that delivered it ([../reference/record/lane-file.md](../reference/record/lane-file.md)) — stated, searchable, and never verified, because the record cannot see another repository. The transfer's same-change guarantee becomes a same-unit-of-review obligation: no gate can span two repositories, so the review checklist carries what the gate used to hold (ADR-0049), and the method is honest about that boundary — what the gate checks, it names; what it cannot check, it assigns.

## What the gate checks here, and what it does not

Gated: `Amends` shape and resolution, including the shape of a declared rule-delta clause; the identity file's presence, grammar, and agreement with the plan repository. Not gated, and stated as review responsibilities: whether a named document is the right one; whether the acceptance transfer reached the amended document in the same unit of review; whether a `new:` promise was kept; whether a story that changed a rule declared its delta clause, and whether the declared type matches the diff; whether the code references are honest; every `Reads` path.
