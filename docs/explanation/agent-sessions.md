# Agent sessions

A lane entry can name the coding session acting on it, and the plan configuration declares how a session of each kind is reached. This chapter explains two choices in that contract. [Session adapters](../reference/session-adapters.md) holds the values, and [../specs/SPEC-agent-sessions.md](../specs/SPEC-agent-sessions.md) states the rules.

## An adapter table is closed

An adapter table takes `inspect`, `contact`, and `resume`, and the configuration rejects every other key. Payload extensibility is not configuration extensibility, so a misspelled operation fails rather than reading as unsupported. A later operation changes this contract and its schema together, and a third-party extension gets one explicit namespaced extension point rather than making every unknown key meaningful.

## Why idle is not a state

A reading's state is `active`, `ended`, or `unknown`, and `active` means present evidence that the session can continue, not that it executes at this instant. Adding `idle` beside the present meaning of `active` makes the two values overlap. A provider's own vocabulary, idle, executing, or streaming, stays in the reading's opaque activity, so the state stays portable across session systems.
