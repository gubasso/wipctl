# A plan names its peers locally and attaches them before any check runs

## Context and Problem Statement

A plan needs to name another project's plan repository before an entry here can depend on an entry there. The name must be readable to this project's own people, must survive the peer moving host, and must not collide with a name some other project chose.

## Considered Options

- A committed table binding a local alias to the peer's uid and to one or more locations, with a walk that fills every named plan's slot before any check reads one
- A `[peers]` table inside `config.toml` — rejected: that file carries facts about this plan, and a peer is a fact about another
- One global name per plan, written into each dependency — rejected: two projects then cannot both call a peer `payments`
- A shared table mapping a uid to a location for a group of plans — deferred: revisit if copying one row between files proves a real cost. Such a table can never be authoritative, because a value guessed on a project's behalf is what the no-defaults rule forbids

## Decision Outcome

Chosen option: the local table. A peer carries three names and they are three facts. The alias is chosen here and means nothing elsewhere. The uid is the identity, declared by the peer and never renamed. A location is a hint, replaceable on its own. The command surface writes the table, so nobody reads a uid out of another plan's file and types it.

Attaching is separate from checking and generous: the walk fills every plan the closure declares, transitively, deduplicated by uid. A check then reads slots and opens nothing, so an answer stays a function of committed files.

Enforced by `peers:a-row-carries-a-uid-and-a-location`, `peers:an-alias-is-local-and-unique`, `peers:one-row-names-one-other-plan`, `peers:the-table-is-earned`, `peers:a-url-is-used-as-written`, `peers:no-check-opens-a-url`, `peer-attachment:the-command-surface-owns-the-table`, and `peer-attachment:the-walk-fills-the-declared-set`.

## Consequences

- Good: a peer can be renamed here and rehosted there, and neither disturbs the other.
- Bad: a plan whose dependencies reach peers cannot be checked from a bare clone.

## Status

Accepted
