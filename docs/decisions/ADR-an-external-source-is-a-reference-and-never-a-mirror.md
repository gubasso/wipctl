# An external source is a reference and never a mirror

## Context and Problem Statement

A project plans in its record and carries pending work in other systems at the same time. The record has no way to name an item in one of those systems, so the link lives in prose that nothing reads. The question is what the record stores: a pointer, or a copy.

## Considered Options

- `a reference, storing the item's key alone` — chosen.
- `a mirrored artifact class, one file per remote item` — rejected: a remote title changes with no rename and no lock, so the id invariant breaks; two machines that refresh one mirror produce a conflict no person caused; and every refresh becomes a commit on an append-only trunk.
- `no link at all, kept in a note` — rejected: prose nothing parses cannot be searched, checked, or rendered.

## Decision Outcome

Chosen option: `a reference, storing the item's key alone` — the remote system already owns the item, so a second copy can only go stale. A story, epic, or initiative carries a `Sources` section, and each item opens with one `<alias>#<key>` token. The alias resolves in `sources.toml`; the key is opaque and no check reaches the network to test it. The reference has the same standing as the branch and the delivered commit: stated, searchable, and never verified, because the record cannot see another system. The local id stays the anchor, so a remote key never enters the id namespace, never collides, and never gets burned by a tombstone.

Enforced by `external-sources:the-remote-system-stays-the-source-of-truth` and `external-sources:a-source-ref-is-stated-and-never-verified`.

## Consequences

- Good: the record stays plain files, one live copy, with no state that ages behind a system nobody here controls. Two plans can reference one item without collision.
- Bad: no gate can tell whether a reference names the right item. That becomes a review responsibility, alongside the code references it sits beside.

## Status

Accepted
