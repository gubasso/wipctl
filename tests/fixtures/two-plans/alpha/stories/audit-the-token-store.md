# Audit the token store

## Goal

A reader of the audit sees every identity in the token store that another process on the machine can reach, so the hardening work starts from a list rather than from a guess.

## Example

```text
$ alpha-audit token-store
3 identities, 1 readable by another process: /var/run/alpha/tokens/ada
```

## Core

The audit walks the store, reports each identity it finds, and marks the ones whose permissions let another process read them. It changes nothing.

## In scope

The permission check and the report.

## Out of scope

Repairing a permission, which is the hardening work this audit feeds.

## Reads

- `docs/specs/SPEC-session.md` — the store's stated permissions

## Amends

- `docs/specs/SPEC-session.md` — record what the audit found about the stated permissions

## Sources

None

## Acceptance

- A store holding a world-readable identity is reported as readable by another process, proven by the permissive store case.
- A store whose identities are closed to other users reports none, proven by the closed store case.

## Tasks

- Walk the store.
- Check each identity's permissions.
- Print the report.

## Rabbit holes

- A permission that looks wrong tempts a repair. The escape is to report it and stop.

## Refused option

- Repairing each permission as the audit finds it, refused because an audit that writes cannot be run to find out what is true.

## Revisions

None
