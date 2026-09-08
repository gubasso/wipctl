# Messages Specification

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

### `messages:a-refusal-names-what-unblocks-it` — A refusal names what unblocks it

When a verb refuses an operation, the message MUST name the verb or the edit that unblocks it.

#### Scenario: A move is refused for an unclosed dependency

- GIVEN an entry whose dependency is still open
- WHEN the move is refused
- THEN the message names that dependency, as a refused drain names the fragment's file and field

Verify: `cargo nextest run --test verb_contracts`

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
