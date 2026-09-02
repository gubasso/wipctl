# ADR-0055: An epic declares its initiative in its document

## Context and Problem Statement

An entry declares its epic in the lane file, because an entry has a lane block. An epic has no lane block — it is a document and nothing else — so the fact that this epic serves that initiative needs a home.

## Considered Options

- A section in the epic document holding one id or the literal `None`
- A member list in the initiative document — a second store of membership, stale on the first epic change
- A mapping file from epic to initiative — a third file class, a fourth instance schema, and a second store of a fact the epic document could hold alone
- An `initiative` field on every lane entry — cheap for the checker, and refused because it repeats the fact per member and can disagree with the member's own epic

## Decision Outcome

Chosen option: the section. The epic document's heading sequence gains `Initiative`, first, before `Goal`, because the end state an epic serves scopes everything the reader is about to read. The section carries exactly one inline-code id token or the literal `None`, and is the one heading besides `Amends` that is never left empty. The required-headings gate owns presence and order; the cross-file checker owns resolution, exactly as it already owns the `Amends` path tokens. An initiative document carries no such section, and the heading gate refuses one, so the no-parent rule needs no separate check.

## Consequences

- Good: membership has one store, one direction, and one checker rule; an initiative no epic has joined is a legal first draft.
- Bad: the epic document shape changes — the heading sequence and the gate configuration both move, and every epic document must carry the section.

## Status

Accepted
