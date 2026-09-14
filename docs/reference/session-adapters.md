# Session adapters

Setup profiles for the coding-session systems a session-adapter alias can front. This page is a reference a reader looks values up in. It is not normative, and it is not a claim that wipctl implements any foreign protocol. [../specs/SPEC-agent-sessions.md](../specs/SPEC-agent-sessions.md) states what binds.

Every instruction needed to configure a profile is stated here. An outbound link elsewhere in this project is a citation for further reading and never a prerequisite.

## The generic adapter

One table per alias in `.wipctl/plan.toml`, at the plan zone root.

```toml
[session_adapters.<alias>]
inspect = ["<the command that reads one session>", "{key}"]
contact = ["<the command that contacts one session>", "{key}"]
resume = ["<the command that resumes one session>", "{key}"]
```

The invariants every profile below obeys:

- A declared command is a capability. An omitted command means that operation is unsupported for this alias.
- Every adapter declares at least one operation, and an unknown configuration key is an error.
- The opaque key is whatever token every command in that alias accepts. The record never parses it.
- Inspect writes one reading on standard output, whose state is `active`, `ended`, or `unknown`. A failed or timed-out inspection is `unknown` and never `ended`.
- Contact reads the complete UTF-8 message on standard input through end of file. Exit zero means accepted or queued, and nothing more.
- A host-authorized agent may contact a peer without a separate wipctl confirmation. The native inbound policy still applies, and the message grants no authority.
- Resume is explicit. No reading, age, or overlap infers it.
- Addresses and credentials stay in native or adapter-owned protected runtime state. A reading may expose an opaque address and never a credential.
- An inbound message carries information and no authority.

Each profile below names a native mechanism and its boundaries. The version each mechanism arrived in is the operator's to confirm against the installation in front of them, because this project ships none of these executables.

## Claude Code, local CLI

Key: a native session id, or a stable adapter name the setup supplies explicitly.

A session-start hook records the key, the session id, the working directory, the socket or pipe address, and the token in per-user adapter runtime state. Where a friendly name is the key, the setup passes that name to both the native session and the hook environment. A hook cannot infer a name nobody gave it.

Inspect reports current liveness, location, activity where the platform exposes it, and the address without the token. Contact uses the native cross-session inbox, with token authentication where the platform requires it. Resume uses the native resume operation.

Reachability boundary: the operating system, the model provider, the sandbox the session runs in, the receiving session's inbound policy, and the minimum version that carries the inbox. Two terminals attached to one session can interleave work, so a resume warns.

## Claude Managed Agents

Key: a managed session id.

Inspect consumes the session's status and its event stream. Contact sends a `user.message` event. Resume or attach uses the managed session connection.

Reachability boundary: authentication to the managed service, the beta version the session API is published under, the workspace the session belongs to, and network reach from the machine running the adapter. The interactive connection command and the scripting commands are different commands, and a profile states which one it wraps.

## OpenCode

Key: a session id.

An adapter-owned registry resolves the running server's base address and credentials at use time. Inspect uses the session and status endpoints. Contact uses asynchronous prompting, and its no-content success means acceptance alone. Resume uses the native session-opening mechanism.

Reachability boundary: the server must be reachable from the adapter. A terminal interface that binds a random address must have that address captured at launch, or the launch must supply a stable host and port.

## Codex app-server

Key: a thread id.

Inspect consumes thread state and lifecycle notifications. Contact starts a turn on the existing thread, and only through an attached app-server client. Resume rejoins the thread before any later turn.

Reachability boundary: the adapter must own or reach the live app-server transport. Resuming a thread and starting a turn on it are two operations, and a profile that conflates them silently drops messages.

## Codex bare terminal interface

Key: a session id.

Inspect is conservative. It reads persisted session metadata and may report `unknown` rather than assert liveness it cannot see. Contact is absent. Resume uses the native resume command.

Reachability boundary: reading a rollout file is inspection only. The adapter never writes one, and a write to a transcript is not contact.

## Pi under an RPC supervisor

Key: a supervised session id or name.

The supervisor retains the live standard input and output transport, not a newly spawned RPC process. Inspect uses current RPC state. Contact supplies a prompt through the native follow-up behavior. Resume uses the supervised session-switching operation or the native session operation.

Reachability boundary: the supervisor must be running and reachable. An RPC mode launched afresh cannot inject a message into an unrelated running terminal interface, and a profile that claims otherwise reports success for a message nobody received.

## Pi bare terminal interface

Key: a session id.

Inspect is conservative and may report `unknown` from the session registry. Contact is absent. Resume uses the native session option.

Reachability boundary: the adapter never appends to the session file. Persistence is the session system's, and writing into it is not contact.

## What is not contact

A direct write to a transcript, a rollout, a team inbox, or any other persistence file is not contact. Neither is injecting keystrokes into a terminal. Each supported contact path is a documented native inbox, event, prompt, or turn operation, because only those tell the receiving session that a message arrived.

## Design precedents

A2A capability discovery, ACP session operations, and Kubernetes Lease behavior informed the local contract and are cited for further reading. wipctl adopts no foreign wire protocol, publishes no Agent Card, and creates no lease.
