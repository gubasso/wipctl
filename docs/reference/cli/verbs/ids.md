# wipctl ids

Every id the record holds, one per line, for consumers that complete or select.

## Usage

```text
wipctl ids
```

## Contract

- One id per line on stdout: entry ids from all five lanes in record order, then epic ids from the documents, then initiative ids, then question ids — one shared namespace, one stream. Every id is a slug ([../../record/ids.md](../../record/ids.md)). No header, no alignment, no sorting.
- The verb MUST NOT gate: a record the checker would reject still yields the ids it could read, exit 0. An empty record prints nothing, exits 0.
- No machine format: a list of strings has no structure to lose.
- The verb MUST NOT judge well-formedness — it reports what is there.

## Example

```text
$ wipctl ids
supervised-child-runtime
rate-limit-the-search-endpoint
profile-composition
secure-session-storage
session-hardening
trustworthy-by-default
Q-may-a-position-name-a-fragment
```
