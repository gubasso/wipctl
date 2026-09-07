# The plan record lives in its own per-project repository

## Context and Problem Statement

The plan record is coordination state — what is in flight right now — and coordination state wants exactly one live copy. Tracked working-tree files give one copy per checkout, each drifting until merge time, which defeats the record under concurrent agents in several worktrees. The drift is a property of keeping the record in tracked working-tree files, not of git: everything under `refs/` is already one state per repository across all worktrees.

## Considered Options

- A separate per-project repository at a user-level location, shared by every checkout on the machine
- Tracked files written only through one trunk-pinned checkout — keeps clone completeness and pull-request review, but gives no per-project control over where the plan is hosted, and plan commits interleave with code commits on every adopting trunk
- The record in out-of-branch refs of the project's own repository — solves the drift as completely, but stops the record being plain files a person can open, diff, and edit without the tool, which is a pillar
- Tracked working-tree files — fails outright: agents cannot see each other's claims until a merge, after the decisions that needed them

## Decision Outcome

Chosen option: a separate per-project repository — one live record per project, hosted wherever its operator chooses, shared by every worktree, branch, and clone of that project on the machine, still plain files in a plain git repository. The host repository keeps only the project's identity.

## Consequences

- Good: several agents in several checkouts read and write one record; hosting and visibility are decided per project.
- Bad: a host clone no longer carries the plan, a branch is no longer the record's proposal mechanism, and atomic plan-and-code commits are gone — the loss ADR-a-plan-change-and-a-code-change-are-no-longer-one-commit records.

## Status

Accepted

Supersedes ADR-the-plan-record-stays-in-version-control.
