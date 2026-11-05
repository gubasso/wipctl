# Agent Sessions Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`agent-sessions:the-session-system-owns-its-own-state` — The session system owns its own state](#agent-sessionsthe-session-system-owns-its-own-state--the-session-system-owns-its-own-state)
  - [`agent-sessions:a-session-reference-is-one-token` — A session reference is one token](#agent-sessionsa-session-reference-is-one-token--a-session-reference-is-one-token)
  - [`agent-sessions:an-adapter-declares-its-operations` — An adapter declares its operations](#agent-sessionsan-adapter-declares-its-operations--an-adapter-declares-its-operations)
  - [`agent-sessions:capabilities-are-derived-from-the-table` — Capabilities are derived from the table](#agent-sessionscapabilities-are-derived-from-the-table--capabilities-are-derived-from-the-table)
  - [`agent-sessions:an-adapter-command-is-one-argument-vector` — An adapter command is one argument vector](#agent-sessionsan-adapter-command-is-one-argument-vector--an-adapter-command-is-one-argument-vector)
  - [`agent-sessions:a-contact-message-travels-on-standard-input` — A contact message travels on standard input](#agent-sessionsa-contact-message-travels-on-standard-input--a-contact-message-travels-on-standard-input)
  - [`agent-sessions:a-contact-is-bounded-and-never-retried` — A contact is bounded and never retried](#agent-sessionsa-contact-is-bounded-and-never-retried--a-contact-is-bounded-and-never-retried)
  - [`agent-sessions:contact-needs-no-second-confirmation` — Contact needs no second confirmation](#agent-sessionscontact-needs-no-second-confirmation--contact-needs-no-second-confirmation)
  - [`agent-sessions:an-unresolved-session-is-unknown` — An unresolved session is unknown](#agent-sessionsan-unresolved-session-is-unknown--an-unresolved-session-is-unknown)
  - [`agent-sessions:liveness-is-three-valued` — Liveness is three-valued](#agent-sessionsliveness-is-three-valued--liveness-is-three-valued)
  - [`agent-sessions:a-reading-never-gates` — A reading never gates](#agent-sessionsa-reading-never-gates--a-reading-never-gates)
  - [`agent-sessions:a-reading-carries-its-instant` — A reading carries its instant](#agent-sessionsa-reading-carries-its-instant--a-reading-carries-its-instant)
  - [`agent-sessions:a-runtime-secret-stays-out-of-the-reading` — A runtime secret stays out of the reading](#agent-sessionsa-runtime-secret-stays-out-of-the-reading--a-runtime-secret-stays-out-of-the-reading)
  - [`agent-sessions:adapter-runtime-state-is-private` — Adapter runtime state is private](#agent-sessionsadapter-runtime-state-is-private--adapter-runtime-state-is-private)
  - [`agent-sessions:resume-is-an-explicit-native-operation` — Resume is an explicit native operation](#agent-sessionsresume-is-an-explicit-native-operation--resume-is-an-explicit-native-operation)
  - [`agent-sessions:a-resume-profile-states-its-mode` — A resume profile states its mode](#agent-sessionsa-resume-profile-states-its-mode--a-resume-profile-states-its-mode)
  - [`agent-sessions:a-fault-takes-its-listed-class` — A fault takes its listed class](#agent-sessionsa-fault-takes-its-listed-class--a-fault-takes-its-listed-class)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

Several coding agents work one project at the same time. The record already says which entries are in flight, and it says nothing about which session acts on one or how a second agent reaches it. This domain lets a work-lane entry name that session, and lets the plan repository declare how a session of that kind is reached. The boundary runs at the locator. This domain owns the declaration, the reference grammar, the three operations, and the shape of a reading. The configuration domain owns the file the declaration sits in. The lane file domain owns the entry that carries the reference. The epics and initiatives domains own the rollups folded from it.

The record stores a locator. Whether a session runs, where it listens, what it works on, and what it can reach stay owned by the session system, and a reader resolves them on use.

## Requirements

### `agent-sessions:the-session-system-owns-its-own-state` — The session system owns its own state

The record MUST store the session reference alone, and the plan configuration MUST store stable adapter commands alone, with every volatile address, credential, status, heartbeat, process identifier, and transcript location resolved from the session system when used.

#### Scenario: A liveness field is proposed on the entry

- GIVEN a request for a stored running state beside the session reference
- WHEN two machines write that field for one entry
- THEN the record becomes a runtime bus written through commits, and `sync:reconciliation-is-semantic-never-textual` reports a conflict over one entry that no person caused

Verify: `cargo nextest run --test schemas`, and a reviewer confirms that each proposed field is a stable declaration and not volatile session state

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

### `agent-sessions:capabilities-are-derived-from-the-table` — Capabilities are derived from the table

When a reader reports a session, it MUST derive the supported operation names from the keys the adapter table carries, in the fixed order `inspect`, `contact`, `resume`, MUST carry that list on every session row of a detail report, MUST store it nowhere, and MUST read it as technical availability and never as permission.

#### Scenario: A peer can only be resumed

- GIVEN an adapter table that declares `resume` alone
- WHEN a detail report lists a session claimed through it
- THEN the row carries the list `resume`, so a reader tells it from an inspectable or a contactable peer without opening the configuration

Verify: `cargo nextest run --test reporting`

### `agent-sessions:an-adapter-command-is-one-argument-vector` — An adapter command is one argument vector

Each declared operation MUST be an argument vector of at least two non-empty strings, with an executable first, exactly one later item equal to `{key}`, and no item carrying another placeholder.

#### Scenario: A command is written as one shell string

- GIVEN an operation declared as a single line with a pipe in it
- WHEN the schema reads it
- THEN it fails, because an argument vector is executed directly and a string invites a shell to parse a key the record never validated

Verify: `cargo nextest run --test schemas`

### `agent-sessions:a-contact-message-travels-on-standard-input` — A contact message travels on standard input

A contact message MUST be passed as UTF-8 on standard input through end of file, MUST appear in no argument vector, configuration file, or part of the plan record, and contact success MUST mean native acceptance or queuing alone.

#### Scenario: A message is placed in the argument vector

- GIVEN a contact command declared with a message placeholder
- WHEN the schema reads it
- THEN it fails, because a message in argv is readable by every process on the machine and an exit code then reads as agreement

Verify: `cargo nextest run --test verb_contracts`

### `agent-sessions:a-contact-is-bounded-and-never-retried` — A contact is bounded and never retried

When a caller runs a contact command, it MUST apply a bounded timeout, MUST report a nonzero exit, a timeout, a signal, or a missing executable as a failed contact, MUST ignore standard output, MUST read standard error as prose, and MUST NOT retry automatically.

#### Scenario: A contact times out

- GIVEN a native inbox that accepted the message and then stalled
- WHEN the bounded timeout expires
- THEN the contact is reported failed and is not sent again, because native acceptance need not be idempotent and a retry can deliver the message twice

Verify: `cargo nextest run --test verb_contracts`

### `agent-sessions:contact-needs-no-second-confirmation` — Contact needs no second confirmation

Where the invoking host authorizes the declared contact command, an agent MAY invoke it without a separate wipctl confirmation, and an inbound message MUST carry information and no consent, leaving the receiving session's inbound policy and authority unchanged.

#### Scenario: An agent answers a peer mid-session

- GIVEN a host that already authorized the declared contact command
- WHEN the agent contacts the peer holding an overlapping path
- THEN the message is delivered without a second prompt, because a duplicated authority boundary obstructs coordination and decides nothing the host did not

Verify: `cargo nextest run --test verb_contracts`, and a reviewer confirms that the host authorized the declared command

### `agent-sessions:an-unresolved-session-is-unknown` — An unresolved session is unknown

Where a declared inspect operation fails, times out, emits an invalid object, or returns no evidence, the reading MUST be reported as unknown and stamped with the instant of that failure, distinct from a session the adapter found ended and from the null reading of an adapter that declares no inspect operation.

#### Scenario: The inspect command is not installed on this machine

- GIVEN a claim whose adapter declares inspection and whose executable is absent
- WHEN the detail view renders
- THEN the row carries an unknown reading with its instant, because an inability to look is a statement about the reader and not about the session

Verify: `cargo nextest run --test schemas`

### `agent-sessions:liveness-is-three-valued` — Liveness is three-valued

A reading's state MUST be one of `active`, `ended`, and `unknown`, and a provider distinction such as idle, executing, or streaming MUST stay in the optional opaque activity.

#### Scenario: A provider reports an idle session

- GIVEN a session system that tells idle from executing
- WHEN its adapter writes a reading
- THEN the state is `active` and the distinction sits in `activity`, because an `idle` state beside `active` makes the two values overlap

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

A normalized reading MAY carry a freshly resolved opaque address, every adapter MUST resolve each address and credential an operation needs from session-system or protected runtime state, and a credential MUST stay out of every reading, process argument, plan file, output stream, log, and diagnostic.

#### Scenario: An adapter needs a token to reach its session

- GIVEN a session system requiring authentication on every call
- WHEN the adapter inspects the session
- THEN the token stays in per-user runtime state the adapter owns, because a reading is printed, piped, logged, and pasted into a report

Verify: `cargo nextest run --test schemas`

### `agent-sessions:adapter-runtime-state-is-private` — Adapter runtime state is private

Where an adapter keeps runtime state of its own, that state MUST be per-user, written atomically, closed to other users, and retired when the native session ends or rotates its credentials.

#### Scenario: A second user shares the machine

- GIVEN an adapter that stores a session token for its user
- WHEN another user on the machine reads the adapter's state
- THEN they read nothing, because the token reaches a session that user does not own

Verify: reviewer reads how the adapter stores its runtime state, because the adapter is the operator's executable and no test of this project runs it

### `agent-sessions:resume-is-an-explicit-native-operation` — Resume is an explicit native operation

A resume operation MUST run only when a caller invokes it explicitly, it MUST preserve the native session system's distinction between attaching, reopening, and starting new work, and its success MUST mean only that the native resume began.

#### Scenario: A rollup shows an ended session on an overlapping path

- GIVEN an epic detail naming an ended claim and a shared path
- WHEN the view renders
- THEN nothing is resumed, because an observation is evidence for a person and resuming is a continuation somebody chose

Verify: `cargo nextest run --test verb_contracts`

### `agent-sessions:a-resume-profile-states-its-mode` — A resume profile states its mode

Where a reference profile declares a resume command, the profile MUST state whether that command is interactive, machine-callable, or both, and MUST warn where two clients can interleave work in one session.

#### Scenario: Two terminals attach to one session

- GIVEN a profile whose native resume attaches a second client to a live session
- WHEN an operator reads the profile before resuming
- THEN the profile warns that the two clients can interleave work, because an attach that looks like a fresh start loses the work of the first client

Verify: reviewer reads each profile in the session adapters reference that declares a resume command

### `agent-sessions:a-fault-takes-its-listed-class` — A fault takes its listed class

When the implementation meets a fault that [the agent session faults](../reference/diagnostics.md#agent-session-faults) list, it MUST report that fault with the class, the exit code, and the resolution that the list gives it.

#### Scenario: A declared inspect command times out

- GIVEN a declared inspect command that times out
- WHEN the view runs
- THEN the reading is unknown, one line names the alias and the command, and every other row is still reported, because the list classes that fault as a degradation and not a failure

Verify: `cargo nextest run --test verb_contracts`

## Unenforced

| Rule                                                   | Reviewer confirms                                                                                                                                        |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent-sessions:the-session-system-owns-its-own-state` | each proposed field is a stable declaration and not volatile session state, because whether it is one or the other is a reading of what it asserts       |
| `agent-sessions:contact-needs-no-second-confirmation`  | the host authorized the declared command, because that authorization is the host's answer, and this record cannot read it                                |
| `agent-sessions:adapter-runtime-state-is-private`      | how the adapter stores its runtime state keeps that state private, because the adapter is the operator's executable, and no test of this project runs it |
| `agent-sessions:a-resume-profile-states-its-mode`      | each profile that declares a resume command states its mode and its warning, because whether a profile states them is a reading of prose                 |
