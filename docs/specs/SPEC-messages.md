# Messages Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`messages:a-failure-names-its-resolution` — A failure names its resolution](#messagesa-failure-names-its-resolution--a-failure-names-its-resolution)
  - [`messages:a-warning-names-its-resolution` — A warning names its resolution](#messagesa-warning-names-its-resolution--a-warning-names-its-resolution)
  - [`messages:multiple-immediate-delay-costs-name-the-count` — Multiple immediate delay costs name the count](#messagesmultiple-immediate-delay-costs-name-the-count--multiple-immediate-delay-costs-name-the-count)
  - [`messages:the-preview-says-when-it-broke-a-tie` — The preview says when it broke a tie](#messagesthe-preview-says-when-it-broke-a-tie--the-preview-says-when-it-broke-a-tie)
  - [`messages:a-refusal-names-what-unblocks-it` — A refusal names what unblocks it](#messagesa-refusal-names-what-unblocks-it--a-refusal-names-what-unblocks-it)
  - [`messages:relayed-output-is-redacted-and-bounded` — Relayed output is redacted and bounded](#messagesrelayed-output-is-redacted-and-bounded--relayed-output-is-redacted-and-bounded)
  - [`messages:success-states-what-changed` — Success states what changed](#messagessuccess-states-what-changed--success-states-what-changed)
  - [`messages:a-message-offers-no-undefined-flag` — A message offers no undefined flag](#messagesa-message-offers-no-undefined-flag--a-message-offers-no-undefined-flag)
  - [`messages:a-message-is-never-a-bare-code` — A message is never a bare code](#messagesa-message-is-never-a-bare-code--a-message-is-never-a-bare-code)
  - [`messages:the-text-is-stable-enough-to-match` — The text is stable enough to match](#messagesthe-text-is-stable-enough-to-match--the-text-is-stable-enough-to-match)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What a command's output owes its reader, human or agent. A message tells the reader what happened, what it means for the record, and what to do next. The boundary runs at content: this domain says what a message must say, while the command surface domain says which stream carries it and which exit code accompanies it.

## Requirements

### `messages:a-failure-names-its-resolution` — A failure names its resolution

When a verb reports a failure, the message MUST name where, what was expected, what was found, and the resolution.

#### Scenario: A check fails with a location and no next step

- GIVEN a diagnostic naming a file, a line, and a mismatch
- WHEN it stops there
- THEN the message is unfinished, because a reader who knows what broke and not what to do is still stuck

Verify: `cargo nextest run --test verb_contracts`

### `messages:a-warning-names-its-resolution` — A warning names its resolution

When a stale or cold-reading warning is emitted, the message MUST state each applicable count and name `wipctl stale` as the resolution.

#### Scenario: Four entries are stale and two watch readings are cold

- GIVEN four rows that the ordinary stale view shows and two watches unread for 30 days
- WHEN a workflow verb finishes
- THEN the warning states both counts and the command that shows them, because a warning without a resolution leaves the reader stuck

The warning is exactly:

```text
wipctl: warning: 4 entries are past the stale line
wipctl: warning: 2 watches have not been read in 30 days
wipctl: run `wipctl stale` to see them
```

Verify: `cargo nextest run --test verb_contracts`

### `messages:multiple-immediate-delay-costs-name-the-count` — Multiple immediate delay costs name the count

When a workflow verb finds multiple open entries with immediate delay cost, the implementation MUST warn once on stderr at exit 0, name the count, preserve its result, and leave resolution to a person.

#### Scenario: Two open entries have immediate delay cost

- GIVEN two open entries whose `delay_cost` is `immediate`
- WHEN a workflow verb finishes
- THEN stderr names the count once and says the person may leave or edit the values while stdout and the exit code keep their ordinary result

The warning is exactly:

```text
wipctl: warning: 2 open entries have immediate delay cost
wipctl: leave them or edit their delay_cost
```

Verify: `cargo nextest run --test verb_contracts`

### `messages:the-preview-says-when-it-broke-a-tie` — The preview says when it broke a tie

Where the final ID tie-break reached the head, the preview under the explain flag MUST say so in one line and MUST name the entry it tied against.

#### Scenario: The head was reached by the final tie-break

- GIVEN two scheduled entries tied on every constraint and preference
- WHEN the preview runs under the explain flag
- THEN it says the tie was broken by the id, because a reader taking the head deserves to know the tool chose where they did not

The line is exactly:

```text
wipctl: the id broke a tie with profile-composition
```

Verify: `cargo nextest run --test verb_contracts`

### `messages:a-refusal-names-what-unblocks-it` — A refusal names what unblocks it

When a verb refuses an operation, the message MUST name the verb or the edit that unblocks it.

#### Scenario: A move is refused for an unclosed dependency

- GIVEN an entry whose dependency is still open
- WHEN the move is refused
- THEN the message names that dependency, as a refused drain names the fragment's file and field

Verify: `cargo nextest run --test verb_contracts`

### `messages:relayed-output-is-redacted-and-bounded` — Relayed output is redacted and bounded

When a verb relays another tool's output, it MUST redact credentials, escape control bytes, bound the excerpt to the first 200 lines, and keep the whole in a named file.

#### Scenario: A failing fetch prints a url carrying a credential

- GIVEN a transport error a reader will paste into a ticket
- WHEN the message is written
- THEN the credential is gone and the control bytes are inert, because a diagnostic travels further than the terminal that produced it

Verify: `cargo nextest run --test sync`

### `messages:success-states-what-changed` — Success states what changed

When a verb succeeds, the output MUST state what changed and where, and MUST name the next step where one exists and is not obvious.

#### Scenario: An entry is captured

- GIVEN a successful capture
- WHEN the report prints
- THEN it names the file written and the step that follows, because a silent success leaves the reader guessing whether anything happened

Verify: `cargo nextest run --test verb_contracts`

### `messages:a-message-offers-no-undefined-flag` — A message offers no undefined flag

A message MUST NOT offer a flag that no verb contract defines.

#### Scenario: A diagnostic suggests a force flag

- GIVEN a refusal suggesting an override the surface does not carry
- WHEN a reader tries it
- THEN the suggestion fails as a usage error, so the message names only what exists

Verify: `cargo nextest run --test verb_contracts`

### `messages:a-message-is-never-a-bare-code` — A message is never a bare code

A message MUST NOT be a bare code, a bare rejection word, a stack trace, or text whose meaning requires reading the source.

#### Scenario: A parse error surfaces from a library

- GIVEN an underlying error with no context
- WHEN it reaches the reader
- THEN the verb wraps it in the four parts above, because a relayed trace names no resolution

Verify: `cargo nextest run --test verb_contracts`

### `messages:the-text-is-stable-enough-to-match` — The text is stable enough to match

The implementation MUST write message text for a person and MUST keep it stable enough for an agent to match on.

#### Scenario: A message is reworded for clarity

- GIVEN an agent matching on a refusal's wording
- WHEN the wording changes
- THEN the change is a contract change, because the audience includes callers that cannot read around it

Verify: reviewer confirms a reworded message is treated as a contract change

## Unenforced rules

| Rule                                          | Why no command decides it                                                                                    |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `messages:the-text-is-stable-enough-to-match` | Whether a rewording breaks a matcher depends on callers outside this project, which no command here can see. |
