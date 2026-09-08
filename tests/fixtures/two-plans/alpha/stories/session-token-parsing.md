# Session token parsing

## Goal

A request carrying a session token turns into an identity value that the rest of the request handling can read.

## Example

```text
$ curl -s -H "Cookie: session=abc" https://app.example.org/whoami
{"identity": "ada"}
```

## Core

One function reads the token and returns an identity or nothing. No caller parses the token itself.

## In scope

The cookie form of the token.

## Out of scope

The header form of the token, which no client sends yet.

## Reads

None

## Amends

None

## Sources

None

## Acceptance

- A valid token returns its identity, proven by the token parsing case.
- A malformed token returns nothing and logs no token text, proven by the malformed token case.

## Tasks

- Write the parsing function.
- Wire it into the request path.

## Rabbit holes

- A malformed token tempts a repair. The escape is to return nothing and let the caller redirect.

## Revisions

None
