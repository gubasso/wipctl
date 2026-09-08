# Profile composition

## Goal

A signed-in reader opens the profile page and sees their own name and settings, composed from the session identity.

## Example

```text
$ curl -s -H "Cookie: session=..." https://app.example.org/profile
{"name": "Ada", "timezone": "UTC"}
```

## Core

The page reads the identity the session carries and renders the fields that identity owns. Nothing else reaches the page.

## In scope

The name field and the timezone field.

## Out of scope

The avatar, which needs a store this record does not have yet.

## Reads

None

## Amends

None

## Sources

None

## Acceptance

- A request with a valid session renders the name and the timezone, proven by the profile request case.
- A request with no session renders nothing and returns the sign-in redirect, proven by the anonymous request case.

## Tasks

- Read the identity from the session.
- Render the two fields.

## Rabbit holes

- A missing timezone tempts a guess. The escape is to render the literal `unset` and stop.

## Revisions

None
