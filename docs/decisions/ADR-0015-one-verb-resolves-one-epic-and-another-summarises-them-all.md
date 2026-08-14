# ADR-0015: One verb resolves one epic and another summarises them all

## Context and Problem Statement

Two questions look alike and are not: which epics exist and how are they going; and what should a session do next for this epic. One surface answering both blurs two return shapes.

## Considered Options

- Two verbs, one per question
- One verb with modes — the modes return different shapes with different schemas; a flag that changes the return type is two verbs wearing one name
- A graph filter (`graph --epic`) instead of a resolution — a plan is not a drawing; a session needs groups and paths, not a picture

## Decision Outcome

Chosen option: two verbs — they return different things. `epics` is the rollup: one row per document, delivered against promised. `epic <id>` is the resolution: the transitive execution plan, partitioned into eligible and blocked. A bare `epic` lists ids and deliberately does not fall back to the rollup — a caller that asked for a listing gets a listing. Each verb's help names the other.

## Consequences

- Good: three output schemas, one per shape, and a pairing rule that generalises — a verb answers one question, and a new question earns a new verb before it earns a flag.
- Bad: two verbs one letter apart must each name the other in help to stay discoverable.

## Status

Accepted
