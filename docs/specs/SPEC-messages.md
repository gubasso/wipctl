# Messages Specification

<!--TOC-->

- [Purpose](#purpose)
- [Relaying another tool's output](#relaying-another-tools-output)
- [Requirements](#requirements)
  - [`messages:a-failure-names-its-resolution` — A failure names its resolution](#messagesa-failure-names-its-resolution--a-failure-names-its-resolution)
  - [`messages:a-warning-names-its-resolution` — A warning names its resolution](#messagesa-warning-names-its-resolution--a-warning-names-its-resolution)
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

## Relaying another tool's output

One class of failure comes from a tool the command drove rather than from the record. The replication substrate cannot complete a fetch, a fast-forward, or a push, or a slot is not as the tool left it. The message relays that tool's own output, states what changed on disk, and then orients the reader. The reader is as likely to be a coding agent as a person, and the orientation is written for both.

```text
git could not complete the operation                                      exit 1
  wipctl: sync of payments-acme stopped: git reported an error during fetch
  wipctl: ---- git output, first 200 lines; whole output in the file below ----
  <git's stdout and stderr, in order, with credentials in any url replaced
   by <redacted> and control bytes escaped, and nothing else changed>
  wipctl: ---- end of git output ----
  wipctl: whole output: <state dir>/sync/payments-acme-2026-09-06T11-04-12Z.log
  wipctl: on disk: the plan trunk on this machine is unchanged, and nothing
          was pushed
  wipctl: before you act, read the state in three places and make sure
          that they agree with what you expect:
  wipctl:   local    git -C <slot> log --oneline -10 <trunk>
  wipctl:   remote   git -C <slot> log --oneline -10 origin/<trunk>
  wipctl:   stray    git -C <slot> status --short; git -C <slot> branch --all;
                     git -C <slot> stash list
  wipctl: every transition on both sides must land in the result. A merge
          that drops one re-opens a story that was closed, forgets a claim
          someone made, or loses a capture.
  wipctl: do not force-push, do not reset the trunk, and do not settle a
          difference by picking a side textually. When both sides moved
          one entry, 'wipctl resolve <id> --keep here|remote' is the tool.
  wipctl: when the state is clear, re-run 'wipctl sync'

the slot is not as the tool left it                                        exit 1
  wipctl: sync of payments-acme stopped: the slot is not on the plan trunk
  wipctl: ---- git output, first 200 lines; whole output in the file below ----
  <git status --short and git branch --show-current, under the same rule>
  wipctl: ---- end of git output ----
  wipctl: whole output: <state dir>/sync/payments-acme-2026-09-06T11-04-12Z.log
  wipctl: on disk: nothing was fetched and nothing was pushed
  wipctl: something outside wipctl checked out a branch, left changes
          unstaged, or detached HEAD in the plan repository. Read the
          three places above before touching anything; the uncommitted
          state may be a plan change that has not landed yet.
  wipctl: return the slot to the trunk with the changes landed or
          deliberately discarded, then re-run 'wipctl sync'
```

Five properties hold across the class.

- The tool's output is relayed between two marker lines, in stream order, with nothing paraphrased and nothing summarized. A reader debugging a transport failure sees what the transport said.
- Two things change on the way through, and only these two. Credentials inside a url become `<redacted>`, and control bytes are escaped. A diagnostic is copied into terminals, transcripts, and continuous-integration logs. Relaying a credential leaks it further than it started, and relaying raw control bytes hands a remote server the reader's terminal.
- The excerpt is bounded and the whole output is kept. The message carries the first 200 lines and names a file holding everything, so fidelity is in the file and the terminal stays readable. The bound is a number on this page, because an implementer who has to choose one will choose a different one.
- The on-disk statement comes next, so a reader knows whether anything moved before reading further. Then the three places to read, the same three in every message of the class, so an agent learns one procedure.
- The orientation names the failure that matters, which is a lost state. It names the verbs that are safe and the actions that are not, and it addresses the reader as you.

Every line of the class is a diagnostic, so all of it goes to the diagnostic stream. The relayed lines carry the prefix on the marker lines rather than on the tool's own text, so a reader can lift the block unchanged.

The plan trunk has one published branch and its history is append-only, so a stray branch here means one that something outside the tool created. The message says so rather than suggesting the plan has branches to reconcile.

## Requirements

### `messages:a-failure-names-its-resolution` — A failure names its resolution

When a verb reports a failure, the message MUST name where, what was expected, what was found, and the resolution.

#### Scenario: A check fails with a location and no next step

- GIVEN a diagnostic naming a file, a line, and a mismatch
- WHEN it stops there
- THEN the message is unfinished, because a reader who knows what broke and not what to do is still stuck

Verify: `cargo nextest run --test verb_contracts`

### `messages:a-warning-names-its-resolution` — A warning names its resolution

When a stale warning is emitted, the message MUST state the number of entries past the stale line and name `wipctl stale` as the resolution.

#### Scenario: Four entries are stale

- GIVEN four rows that the ordinary stale view shows
- WHEN a workflow verb finishes
- THEN the warning states the count and the command that shows the rows, because a warning without a resolution leaves the reader stuck

The warning is exactly:

```text
wipctl: warning: 4 entries are past the stale line
wipctl: run `wipctl stale` to see them
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

When a verb relays another tool's output, it MUST redact credentials, escape control bytes, bound the excerpt, and keep the whole in a named file.

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
