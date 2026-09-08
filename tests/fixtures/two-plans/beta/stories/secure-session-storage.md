# Secure session storage

## Goal

A session identity survives a restart, and no other process on the machine can read it.

## Example

```text
$ systemctl restart app && curl -s -H "Cookie: session=abc" https://app.example.org/whoami
{"identity": "ada"}
```

## Core

The store holds one record per session, on disk, mode 0600, keyed by the token.

## In scope

The on-disk store and its permissions.

## Out of scope

A shared store across machines, which needs a decision this project has not made.

## Reads

None

## Amends

None

## Sources

None

## Acceptance

- A session survives a restart, proven by the restart case.
- The store file is mode 0600, proven by the permission case.

## Tasks

- Write the store.
- Set the permissions at creation.

## Rabbit holes

- A slow store tempts a cache. The escape is to measure first and stop.

## Revisions

None
