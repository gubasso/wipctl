# Token rotation

## Goal

Every sign-in replaces the session token, so a token read from an old log grants nothing.

## Example

```text
$ curl -si -X POST https://app.example.org/sign-in | grep -i set-cookie
set-cookie: session=<a new value on every sign-in>
```

## Core

The sign-in path mints a token, writes it to the store, and drops the previous one.

## In scope

The sign-in path.

## Out of scope

Rotation on a timer, which needs a scheduler this project does not run.

## Reads

None

## Amends

None

## Sources

None

## Acceptance

- Two sign-ins produce two tokens, proven by the rotation case.
- The previous token stops working, proven by the stale token case.

## Tasks

- Mint a token on sign-in.
- Drop the previous token.

## Rabbit holes

- A dropped token tempts a grace period. The escape is to drop it at once and record the choice.

## Revisions

None
