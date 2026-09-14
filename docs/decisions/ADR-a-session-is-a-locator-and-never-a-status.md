# A session is a locator and never a status

## Context and Problem Statement

Several coding agents work one project at once. The record says which entries are in flight and nothing about which session acts on one, so an agent that wants to coordinate guesses from branches and commit times.

## Considered Options

- `a locator plus stable adapter commands, resolved on use` — chosen.
- `a stored state, address, credential, process identifier, or heartbeat` — rejected: it makes the record a runtime bus written through commits, and manufactures a `sync:reconciliation-is-semantic-never-textual` conflict.
- `a capability list beside the commands` — rejected: it can contradict command presence, and nothing decides which half wins.
- `ignored unknown operation keys` — rejected: a misspelled operation would read as unsupported, which `configuration:an-unknown-key-is-rejected` already refuses.
- `an Agent Card name, or a required foreign transport` — rejected: the declaration is a local profile claiming no wire compatibility.
- `a separate agent field beside a session field` — rejected: a pair can be written half complete.
- `a fourth transition-journal column carrying the actor` — rejected: `transition-journal:the-journal-records-no-reason` refuses it, and a claim is state rather than an event.
- `a required session on every entry` — rejected: work with no resolvable session would need a synthetic alias.

## Decision Outcome

Chosen option: `a locator plus stable adapter commands, resolved on use` — capability publication is separated from identity and from credentials. The record stores one `<alias>#<key>` token. The configuration declares each native operation by the presence of its command, so the operations a reader reports cannot disagree with the operations that exist. Everything volatile is resolved at the local command boundary. A2A capability discovery, ACP session operations, and Kubernetes Leases are cited for further reading alone.

Enforced by `agent-sessions:the-session-system-owns-its-own-state` and `agent-sessions:an-adapter-declares-its-operations`.

## Consequences

- Good: the record stays plain files with one live copy, and a reader learns a peer's reachability with no second list to keep in step.
- Bad: discovery reaches only entries carrying a claim, and an ended session keeps its claim until a person resolves it.

## Status

Accepted
