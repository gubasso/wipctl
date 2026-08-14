# wipctl ids

Every id the record holds, one per line, for consumers that complete or select.

## Usage

```text
wipctl ids [<plan-dir>]
```

## Contract

- One id per line on stdout: entry ids from all five lanes in record order, then epic ids from the documents, then question ids — one shared namespace, one stream. No header, no alignment, no sorting.
- The verb MUST NOT gate: a record the checker would reject still yields the ids it could read, exit 0. An empty record prints nothing, exits 0.
- No machine format: a list of strings has no structure to lose.
- The verb MUST NOT judge well-formedness — it reports what is there.

## Example

```text
$ wipctl ids
supervised-child-runtime-b47d
rate-limit-the-search-endpoint-a7f3
profile-composition-e01a
secure-session-storage-9c2e
session-hardening-c4d1
Q-may-a-position-name-a-fragment-1d55
```
