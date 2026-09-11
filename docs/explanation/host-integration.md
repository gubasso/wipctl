# Host integration

## The single assumption

wipctl asks the host project to carry one committed file: `.wipctl/project.toml` at its root, naming the project's identity. That is the entire footprint. The plan itself lives in the project's own plan repository, resolved through that identity, as [../specs/SPEC-attachment.md](../specs/SPEC-attachment.md) states. The host's layout, its document kinds, its conventions, and its history stay untouched. No verb commits to the host, ever. The configuration has no defaults for anything a project must declare, and no default is ever added.

The scaffold writes exactly that one file into the host. It owns the plan repository it creates, including the method document that travels with it. Host-owned files are read at most. Content intended for them is printed for a person to place.

## Two directions into the host

Stories reference the host's documents across the two repositories. The paths resolve against the host's project root, which is the directory holding the project file.

`Reads` is the inbound direction. It names the individual documents a work session must load before starting. The gate does not check it, because the right documents are a judgment about the host's own tree. That makes it entirely a review responsibility.

A host whose documentation method excludes a class of document from that load set excludes it here too. Several methods exclude decision records, so a session reads the rule that binds rather than the argument that shaped it. This method adds no constraint of its own and lifts none.

`Amends` is the outbound direction. It names the documents the work must leave changed, and it is gated. Each path is relative, carries no dot segment, and resolves against the project root. An assertion opening with `new:` is the exception. It promises a document the work will create, and it exempts existence for the life of the entry. Both directions point anywhere in the project, not only at its documentation.

A host that keeps requirement-level specifications identifies each rule by an id of the shape `<domain>:<rule-slug>`. An `Amends` assertion can then carry a typed delta: `ADDED`, `MODIFIED`, or `REMOVED`, plus the rule id in inline code. Rule-to-story traceability is then one fixed-string search over the plan zone. The gate checks a declared clause's shape and never its presence, for the same reason the inbound direction is unchecked. Which host documents keep rule ids is a fact about the host.

## A reference out of the project

A project rarely plans in one list. Forge issues, ticket-system items, and bug reports name pending work at the same time. The record needs a way to say which of them a piece of work answers. `Sources` is that way, and it runs in a third direction. It points out to a system neither repository can see.

The shape holds the same doctrine as the branch and the commit. The record stores the item's key and nothing else. The remote system stays the source of truth, so nothing here goes stale behind it. A reference is `<alias>#<key>`, and the alias is declared once in the plan configuration's `sources` section, per [../specs/SPEC-external-sources.md](../specs/SPEC-external-sources.md). The host's footprint does not grow: the declaration lives with the plan, and the host still carries one file.

Two rules keep the tier ladder intact. A reference never sequences and never counts, so `needs` stays the one sequencing fact. A reference also carries no hierarchy, so a story's reference is never checked against its epic's. Membership runs one way through the local ladder, and a remote tree read as membership becomes a second store of it. The cost is stated rather than hidden. Nothing tells a reader that a story and its epic point at unrelated outside trees, and that stays a review question.

Sharing is legal in both directions, and it needs no machinery. A story names every item it answers. An epic names the item that states its end state, and its member stories name that same item where they answer it too. Each claim lives in the document that makes it, and the record keeps no index of the reverse. The reverse is a search over the plan zone:

```bash
rg -l 'issues#412' <the plan zone root>
```

That answer costs nothing to keep true, because there is nothing to keep. It needs no network, no declared list command, and no cache. It also finds an artifact whose item the remote system has already closed, which a view built from an open list cannot.

## A story is a diff, a durable document is a state

This is the doctrine the whole integration rests on. A story describes a change, and the host's documents describe what is true. When a story closes, its acceptance assertions are rewritten in the present tense into the amended documents, together with the behavior. The story then freezes as history. The durable document carries the truth forward and keeps no inverse list of the stories that shaped it.

Plan and code no longer share commits, so the record states the linkage explicitly instead. An entry carries the branch that implements it and, once closed, the commit that delivered it, per [../specs/SPEC-lane-file.md](../specs/SPEC-lane-file.md). Both are stated, searchable, and never verified, because the record cannot see another repository.

The transfer's same-change guarantee becomes a same-unit-of-review obligation. No gate spans two repositories, so the review specification carries what the gate used to hold, per ADR-a-plan-change-and-a-code-change-are-no-longer-one-commit. The method is honest about that boundary: what the gate checks, it names, and what it cannot check, it assigns.

## What the gate checks here, and what it does not

The gate checks the outbound direction's shape and resolution, including the shape of a declared rule-delta clause. It checks the project file's presence, its grammar, and its agreement with the plan repository. It checks a source reference's shape and its alias against the declared sources, and stops there.

These are review responsibilities instead, and are stated as such:

- whether a named document is the right one
- whether the acceptance transfer reached the amended document in the same unit of review
- whether a promised document was created
- whether a story that changed a rule declared its delta clause, and whether the declared type matches the diff
- whether the code references are honest
- whether a source reference names work the artifact delivers
- every inbound path
