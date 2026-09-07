# Every message teaches

## Context and Problem Statement

The readers of this tool are increasingly agents, and an agent cannot infer a next step from a message that only names a failure. The two failure modes are not symmetric: a person who is confused asks; an agent that is confused acts. A diagnostic that says what broke without saying what unblocks it costs a round trip for a person and a wrong action for an agent.

## Considered Options

- A product principle: a pillar in the charter and binding obligations on every verb page
- A style note in the conventions — advisory prose that binds nothing and decays with the first hurried verb
- A diagnostic code namespace with a lookup — moves the teaching out of the message into a manual the reader may not have; the message is the diagnostic

## Decision Outcome

Chosen option: a product principle. Every message tells its reader what happened, what it means for the record, and what to do next; a diagnostic that names a failure without naming its resolution is unfinished. A failure carries four things: where, what was expected, what was found, and the resolution — a command to run or the judgment to make. A refusal names the verb or the edit that unblocks it. Success output states what changed and names the next step where one exists and is not obvious. Guidance goes to stderr under the existing stream discipline, and the text is written for a person while staying stable enough for an agent to match on. The obligation is gated, not asserted: every refusal path's contract case asserts that its message names the resolution.

## Consequences

- Good: a refused verb is a course correction rather than a dead end, for people and for agents alike.
- Bad: every message is a wording judgment no test settles, and the contract cases grow an assertion per refusal path.

## Status

Accepted
