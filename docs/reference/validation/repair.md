# Writing the record

Four verbs write a maintained record: `fix` (rank order), `move` (transitions), `land` (fragments), `delete` (removals). Everything any of them writes is bound by the same discipline.

## The rank repair's four guarantees

1. Deterministic — the same input MUST produce the same output, byte for byte.
2. Identity-preserving on legal input — a record already legal MUST pass through unchanged, including the presence or absence of a final newline.
3. Idempotent — running twice MUST equal running once.
4. Non-canonical — the repair MUST restore legality, never a canonical form. Any legal permutation is a fixed point; a legal order a human chose MUST NOT be disturbed.

The four bind `fix` and the rank-repair phase `move` runs (ADR-0010). `move`, `land`, and `delete` are invoked to change a legal record; what every writer owes on the bytes it touches is the shared discipline below.

## Mechanics every writer shares

- Every write is deterministic: the same record, the same invocation, and the same stamped instant MUST produce the same bytes. An instant a verb stamps — a transition event's instant, a defaulted close date — is read from the clock exactly once per operation.
- Entry blocks MUST be relocated as their original lines. Nothing re-serialises YAML, so comments, `note` fields, and blank lines survive every operation.
- Preflight before the first byte: every check the operation depends on MUST run first; a refused operation MUST leave every file byte-identical.
- Refusal on broken content: a writer MUST NOT operate on a record whose content checks fail or whose graph is cyclic — it MUST exit with the check status instead.
- Writes MUST land via a temporary file beside the target, renamed into place, only when content differs; temporary state MUST be cleaned up on interrupt.
- The zone lock MUST be held from first read to last rename, the rank repair included; see [../cli/conventions.md](../cli/conventions.md).
- All-or-nothing where an operation covers many items: `land` MUST land everything or nothing.
- A hook MUST NOT invoke a writer.

## Who may reorder what

Only the rank repair MAY reorder, and two verbs invoke it: `fix` on explicit request, and `move` after its bottom-of-lane insertion. `land` deliberately runs no repair, and MUST report any resulting ranking failure, leaving the repair as a separate act, because a landing that silently reorders is a machine making a rank claim. `delete` removes and MUST NOT reorder.
