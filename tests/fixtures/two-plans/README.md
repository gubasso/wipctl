# The two-plan fixture

Two plan records, `alpha` and `beta`, each carrying both identities, five lane files, a peer table, and one story document per entry. `scripts/check-schemas` validates every file here against the schema its name and position select, and `tests/schemas.rs` plants a defect in a copy of this tree for each rule it wants to see fail.

Three things in it are deliberate and must not be tidied away.

- `alpha` and `beta` name each other. That is what the attachment walk's visited set exists for: a walk with no memory of where it went never ends.
- The dependency graph carries a cycle that crosses the two records. `alpha#profile-composition` needs `beta#secure-session-storage`, and that entry needs back. The change that implements the checker uses this to prove the cycle is reported from either side with the same node set.
- `alpha` declares `platform` and no entry references it. That is the instance of the stale rule: an unreferenced row is legal while unattached, and validation reports it rather than failing on it.

Nothing here is a model record to copy. It is input for gates, and two of the three facts above are faults a real record must not carry.
