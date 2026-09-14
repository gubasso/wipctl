# Agent Sessions Specification

<!--TOC-->

- [Purpose](#purpose)
- [The declared adapter](#the-declared-adapter)
- [The reference](#the-reference)
- [Capability discovery](#capability-discovery)
- [The inspect operation](#the-inspect-operation)
- [The contact operation](#the-contact-operation)
- [The resume operation](#the-resume-operation)
- [Runtime addresses and credentials](#runtime-addresses-and-credentials)
- [Requirements](#requirements)
  - [`agent-sessions:the-session-system-owns-its-own-state` — The session system owns its own state](#agent-sessionsthe-session-system-owns-its-own-state--the-session-system-owns-its-own-state)
  - [`agent-sessions:a-session-reference-is-one-token` — A session reference is one token](#agent-sessionsa-session-reference-is-one-token--a-session-reference-is-one-token)
  - [`agent-sessions:an-adapter-declares-its-operations` — An adapter declares its operations](#agent-sessionsan-adapter-declares-its-operations--an-adapter-declares-its-operations)
  - [`agent-sessions:an-adapter-command-is-one-argument-vector` — An adapter command is one argument vector](#agent-sessionsan-adapter-command-is-one-argument-vector--an-adapter-command-is-one-argument-vector)
  - [`agent-sessions:a-contact-message-travels-on-standard-input` — A contact message travels on standard input](#agent-sessionsa-contact-message-travels-on-standard-input--a-contact-message-travels-on-standard-input)
  - [`agent-sessions:contact-needs-no-second-confirmation` — Contact needs no second confirmation](#agent-sessionscontact-needs-no-second-confirmation--contact-needs-no-second-confirmation)
  - [`agent-sessions:an-unresolved-session-is-unknown` — An unresolved session is unknown](#agent-sessionsan-unresolved-session-is-unknown--an-unresolved-session-is-unknown)
  - [`agent-sessions:a-reading-never-gates` — A reading never gates](#agent-sessionsa-reading-never-gates--a-reading-never-gates)
  - [`agent-sessions:a-reading-carries-its-instant` — A reading carries its instant](#agent-sessionsa-reading-carries-its-instant--a-reading-carries-its-instant)
  - [`agent-sessions:a-runtime-secret-stays-out-of-the-reading` — A runtime secret stays out of the reading](#agent-sessionsa-runtime-secret-stays-out-of-the-reading--a-runtime-secret-stays-out-of-the-reading)
  - [`agent-sessions:resume-is-an-explicit-native-operation` — Resume is an explicit native operation](#agent-sessionsresume-is-an-explicit-native-operation--resume-is-an-explicit-native-operation)
- [Diagnostics](#diagnostics)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Several coding agents work one project at the same time. The record already says which entries are in flight, and it says nothing about which session acts on one or how a second agent reaches it. This domain lets a work-lane entry name that session, and lets the plan repository declare how a session of that kind is reached. The boundary runs at the locator. This domain owns the declaration, the reference grammar, the three operations, and the shape of a reading. The configuration domain owns the file the declaration sits in. The lane file domain owns the entry that carries the reference. The epics and initiatives domains own the rollups folded from it.

The record stores a locator. Whether a session runs, where it listens, what it works on, and what it can reach stay owned by the session system, and a reader resolves them on use.

## The declared adapter

The `session_adapters` section sits in `.wipctl/plan.toml` beside the `sources` section, one table per alias. An alias names an integration profile rather than an agent. The section is optional, and a project that reaches no discoverable coding session declares nothing.

```toml
# the session_adapters section of .wipctl/plan.toml

[session_adapters.sessions]
inspect = ["<the command that reads one session>", "{key}"]
contact = ["<the command that contacts one session>", "{key}"]
resume = ["<the command that resumes one session>", "{key}"]
```

The example is a placeholder, and no coding agent is a requirement. Each operation is optional, and a present adapter table carries at least one of them. A declared command is the capability, so the configuration carries no second capability list that can disagree with the commands.

Every command is an argument vector of at least two non-empty strings. Its first item is the executable, and exactly one later item is `{key}`. No item carries another placeholder. The contact operation admits no message placeholder, because its message arrives on standard input and is read through end of file. The specification supplies no default alias, command, or operation.

The adapter table is closed. `inspect`, `contact`, and `resume` are its only keys, and `configuration:an-unknown-key-is-rejected` rejects every other one. Payload extensibility is not configuration extensibility, so a misspelled operation fails rather than reading as unsupported. A later operation changes this contract and its schema together, and a third-party extension gets one explicit namespaced extension point rather than making every unknown key meaningful.

The `session_adapters` half of `plan.schema.json` owns the section's shape, in the configuration spec's companion directory. The cross-file checker owns every fact spanning the section and a lane entry.

## The reference

A session reference is one token, `<alias>#<key>`. The alias names a table in the `session_adapters` section. The key is whatever opaque token that adapter's commands accept: an identifier, a stable name, or another native locator. This record never parses it.

```yaml
- id: rate-limit-the-search-endpoint
  session: "sessions#4f19a2"
```

The token is double-quoted in a lane file, for the reason `lane-file:a-prefixed-dependency-is-quoted` gives about the comment marker. A session is optional, and its absence means the entry makes no session claim. That covers work performed without a resolvable coding session, so nobody needs a synthetic adapter alias.

## Capability discovery

A reader derives the supported operation names from the keys the adapter table carries, in the fixed order `inspect`, `contact`, `resume`, and stores that list nowhere. Every session row in a detail report carries the derived list, so a reader tells an inspectable peer from a contactable one and from a session that can only be resumed, without opening the configuration separately.

The presence of a command declares technical availability and never permission. The invoking environment owns authorization to run it. Where that host authorization permits the declared contact command, an agent invokes it without a separate wipctl confirmation. No reading, prediction, overlap, age, or failed contact invokes contact or resume because the fact was observed. Contact is an agent's explicit coordination action, and resume is an explicit continuation action.

## The inspect operation

An inspect adapter receives the key in place of the one `{key}` argument. On success it writes one `session-reading.schema.json` object on standard output.

```text
state        active | ended | unknown
observed_at  the instant the reading was taken
location     optional, opaque: where the session is working
activity     optional, opaque: what the session is doing
address      optional, opaque: where the adapter found the live session
```

`active` means the adapter holds present evidence that the session can continue. It does not mean that the session executes at that instant. `ended` means the adapter holds present evidence that the session cannot continue. Where a declared inspect command fails, times out, emits an invalid object, or supplies no evidence, the reader reports `unknown` and stamps when it observed that failure. An inability to look is never read as proof that a session ended.

The liveness state stays three-valued. A provider distinction such as idle, executing, streaming, compacting, or waiting stays in the optional opaque activity until a later domain establishes a portable second field. Adding `idle` beside the present meaning of `active` makes the two values overlap.

The reading is a closed object requiring `state` and `observed_at`. The instant uses the transition journal's textual form. The optional address is freshly resolved and carries no credential.

Where an adapter declares no inspect command, no inspection runs. A detail row then carries a null reading, which differs from an inspectable session whose attempted reading has state `unknown`.

## The contact operation

A contact adapter receives the key in argv and the complete UTF-8 message on standard input, and it reads to end of file. The message appears in no argument vector, in no configuration file, and in no part of the plan record. Exit status zero means that the native session system accepted or queued the message. It never means that the peer read, understood, approved, or obeyed it, and it never resumes an ended session. A nonzero exit, a timeout, a signal, or a missing executable is a failed contact and is reported as one.

The caller applies a bounded timeout and retries nothing automatically, because native acceptance need not be idempotent. Standard output is ignored in this contract. Standard error may supply a diagnostic, and a reader treats that diagnostic as prose rather than as structured state.

An inbound peer message carries information and no consent. It approves no protected operation and changes no authority of the receiving session. The receiving session system stays responsible for its own inbound policy.

## The resume operation

A resume adapter receives the key in argv and asks the native system to make that session available for continuation. It may attach a client or reopen stored context, as its reference profile states. Success means that the native resume operation began. It implies neither that the session had ended nor that the caller gained permission to act.

Resume is explicit. A view, an age calculation, an ended or unknown reading, a touch overlap, and a failed contact each trigger nothing. A reference profile states whether its resume command is interactive, machine-callable, or both, and it warns where two clients interleave work in one session.

## Runtime addresses and credentials

The plan record and the plan configuration store stable declarations alone. A process identifier, socket path, pipe name, base address, transcript path, heartbeat, token, and credential each stay where the session system owns them. An inspect adapter may return a current opaque address. A contact or resume adapter resolves every address and credential it needs from session-system or adapter-owned runtime state.

Adapter-owned runtime state is per-user, written atomically, closed to other users, and retired when the native session ends or rotates its credentials. A credential stays out of every normalized reading, process argument, plan file, output stream, log, and diagnostic. The adapter applies the native authentication and the native inbound policy, and this record applies neither.

## Requirements

### `agent-sessions:the-session-system-owns-its-own-state` — The session system owns its own state

The record MUST store the session reference alone, and the plan configuration MUST store stable adapter commands alone, with every volatile address, credential, status, heartbeat, process identifier, and transcript location resolved from the session system when used.

#### Scenario: A liveness field is proposed on the entry

- GIVEN a request for a stored running state beside the session reference
- WHEN two machines write that field for one entry
- THEN the record becomes a runtime bus written through commits, and `sync:reconciliation-is-semantic-never-textual` reports a conflict over one entry that no person caused

Verify: `cargo nextest run --test schemas`

### `agent-sessions:a-session-reference-is-one-token` — A session reference is one token

A session reference MUST be one `<alias>#<key>` token whose alias names a declared session adapter and whose key stays opaque past that token boundary.

#### Scenario: A separate agent field is proposed beside the session field

- GIVEN a pair of fields naming the program and the session
- WHEN one of the two is written and the other is left out
- THEN the entry states half a locator, so the prefixed token carries both halves and one field is written or absent

Verify: `cargo nextest run --test schemas`

### `agent-sessions:an-adapter-declares-its-operations` — An adapter declares its operations

A session adapter MUST declare at least one of `inspect`, `contact`, and `resume`, and the operations present MUST be the capabilities a reader reports.

#### Scenario: A capability list is proposed beside the commands

- GIVEN a table carrying both a declared command set and a written capability list
- WHEN the two disagree
- THEN a reader has two answers and no rule to pick one, so command presence is the single declaration

Verify: `cargo nextest run --test schemas`

### `agent-sessions:an-adapter-command-is-one-argument-vector` — An adapter command is one argument vector

Each declared operation MUST be an argument vector of at least two non-empty strings, with an executable first and exactly one later item equal to `{key}`.

#### Scenario: A command is written as one shell string

- GIVEN an operation declared as a single line with a pipe in it
- WHEN the schema reads it
- THEN it fails, because an argument vector is executed directly and a string invites a shell to parse a key the record never validated

Verify: `cargo nextest run --test schemas`

### `agent-sessions:a-contact-message-travels-on-standard-input` — A contact message travels on standard input

A contact message MUST be passed as UTF-8 on standard input through end of file, and contact success MUST mean native acceptance or queuing alone.

#### Scenario: A message is placed in the argument vector

- GIVEN a contact command declared with a message placeholder
- WHEN the schema reads it
- THEN it fails, because a message in argv is readable by every process on the machine and an exit code then reads as agreement

Verify: `cargo nextest run --test verb_contracts`

### `agent-sessions:contact-needs-no-second-confirmation` — Contact needs no second confirmation

Where the invoking host authorizes the declared contact command, an agent MAY invoke it without a separate wipctl confirmation, and the receiving session's inbound policy and authority stay unchanged.

#### Scenario: An agent answers a peer mid-session

- GIVEN a host that already authorized the declared contact command
- WHEN the agent contacts the peer holding an overlapping path
- THEN the message is delivered without a second prompt, because a duplicated authority boundary obstructs coordination and decides nothing the host did not

Verify: `cargo nextest run --test verb_contracts`

### `agent-sessions:an-unresolved-session-is-unknown` — An unresolved session is unknown

Where a declared inspect operation fails or returns no evidence, the reading MUST be reported as unknown, distinct from a session the adapter found ended and from an adapter that declares no inspect operation.

#### Scenario: The inspect command is not installed on this machine

- GIVEN a claim whose adapter declares inspection and whose executable is absent
- WHEN the detail view renders
- THEN the row carries an unknown reading with its instant, because an inability to look is a statement about the reader and not about the session

Verify: `cargo nextest run --test schemas`

### `agent-sessions:a-reading-never-gates` — A reading never gates

A reading or an adapter capability MUST NOT change record validity, gate a transition, or move, close, resume, contact, or reassign an entry on its own. A person or an agent acts on the reading explicitly.

#### Scenario: A session ends without handing the work on

- GIVEN a claim whose adapter reports the session ended
- WHEN validation and the views run
- THEN the entry stays where it is with its claim intact, and the honest record says that somebody claimed this and nobody resolved it

Verify: `cargo nextest run --test validation`

### `agent-sessions:a-reading-carries-its-instant` — A reading carries its instant

Every reported reading MUST carry `observed_at`, the instant it was taken, in the textual form `transition-journal:an-instant-is-one-textual-form` states.

#### Scenario: A reading is passed between two readers

- GIVEN a reading taken at one moment and read at another
- WHEN a reader decides whether to act on it
- THEN the instant says how old the observation is, because an undated liveness claim is a claim about now that nobody can check

Verify: `cargo nextest run --test schemas`

### `agent-sessions:a-runtime-secret-stays-out-of-the-reading` — A runtime secret stays out of the reading

A normalized reading MAY carry a freshly resolved opaque address, and every adapter MUST resolve a credential from protected runtime state when an operation needs one.

#### Scenario: An adapter needs a token to reach its session

- GIVEN a session system requiring authentication on every call
- WHEN the adapter inspects the session
- THEN the token stays in per-user runtime state the adapter owns, because a reading is printed, piped, logged, and pasted into a report

Verify: `cargo nextest run --test schemas`

### `agent-sessions:resume-is-an-explicit-native-operation` — Resume is an explicit native operation

A resume operation MUST run only when a caller invokes it explicitly, and it MUST preserve the native session system's distinction between attaching, reopening, and starting new work.

#### Scenario: A rollup shows an ended session on an overlapping path

- GIVEN an epic detail naming an ended claim and a shared path
- WHEN the view renders
- THEN nothing is resumed, because an observation is evidence for a person and resuming is a continuation somebody chose

Verify: `cargo nextest run --test verb_contracts`

## Diagnostics

Each fault names its own resolution.

- A session reference naming an undeclared alias. This is a failure, exit 1. The message names the alias, the entry, and the `session_adapters` section, and it offers the two resolutions: declare the alias, or correct the reference.
- A malformed reference token. This is a failure, exit 1. The message shows the token and the `<alias>#<key>` shape it must carry.
- A declared inspect command that is absent, exits non-zero, times out, or emits an unreadable object. This is a degradation rather than a failure. The reading is unknown, one line on the error stream names the alias and the command, and the view still reports every other row.
- A failed contact. This is a failure, exit 1. The message names the alias, the key, and that the message was not accepted, and it leaves the record unchanged.

## Unenforced rules

| Rule                                                   | Why no command decides it                                                                                   |
| ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `agent-sessions:the-session-system-owns-its-own-state` | Whether a proposed field is volatile session state or a stable declaration is a reading of what it asserts. |
| `agent-sessions:contact-needs-no-second-confirmation`  | Whether a host authorized the declared command is the host's answer, and this record cannot read it.        |
