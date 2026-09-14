# The two-plan fixture

Two plan records, `alpha` and `beta`. Each carries one configuration file at `.wipctl/plan.toml`, holding both identities and a peer section, plus five lane files and one story document per entry. `scripts/check-schemas` validates every file here against the schema its name and position select, and `tests/schemas.rs` plants a defect in a copy of this tree for each rule it wants to see fail.

Four things in it are deliberate and must not be tidied away.

- `alpha` and `beta` name each other. That is what the attachment walk's visited set exists for: a walk with no memory of where it went never ends.
- The dependency graph carries a cycle that crosses the two records. `alpha#profile-composition` needs `beta#secure-session-storage`, and that entry needs back. The change that implements the checker uses this to prove the cycle is reported from either side with the same node set.
- `alpha` declares `platform` and no entry references it. That is the instance of the stale rule: an unreferenced row is legal while unattached, and validation reports it rather than failing on it.
- `alpha` declares two session adapters and one in-flight entry that claims a session. The `sessions` alias declares all three operations, which is the row that exercises the full capability ordering. The `detached` alias declares resume alone, which is the row a reader resolves to a null reading because inspection is unsupported there.

Nothing here is a model record to copy. It is input for gates, and two of the four facts above are faults a real record must not carry.
