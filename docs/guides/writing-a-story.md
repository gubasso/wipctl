# Writing a story

One phase, from an itch to a gated entry. The heading contract the document must satisfy is [reference/record/documents.md](../reference/record/documents.md); this page is the order to work in.

1. Mint the identity:

   ```text
   $ wipctl new story "<title>"
   ```

   writes the document and its pending fragment, with the entry's `points` and `summary` left absent for you to fill — or copy the shipped template to `stories/<slug>-<uid>.md` with a hand-minted uid (four lowercase hex from any source of randomness; a collision is surfaced — the gate names claimants in one record, and parallel clones collide as a version-control conflict — and re-minted, never prevented).

2. Name the outcome, not the mechanism. `Goal` says what becomes true, in the reader's terms.
3. Show it from the reader's side. `Example` carries concrete values and real output in a fenced block — a description of a transcript is not a transcript.
4. Separate `Core` from the ordered cuts in `In scope`. If the story wants four points, split it along the judgments its acceptance names, before writing more.
5. Wire the references. `Governed by`: the individual sources a session must load, stated as claims linked to stable anchors, not line numbers. `Amends`: the documents the work must leave changed — or `None`. Mark a document the work will create with `new:`. [wiring-host-docs.md](./wiring-host-docs.md) is the full sequence.
6. Make acceptance falsifiable. Every assertion names the test that proves it; the point value counts the judgments in that section a test cannot settle.
7. Pre-authorise escapes in `Rabbit holes`: every known trap gets a written way out. An escape written after hitting the trap is a revision.
8. Write the difference and nothing else. Delete the sentence; if a durable fact disappeared, it was in the wrong file — durable facts live in the documents `Amends` names.
9. Complete the fragment — the entry's `points` and `summary`, and any field the capture left absent — and land it, or add the lane entry by hand; the drain refuses an incomplete fragment. Then:

   ```text
   $ wipctl validate
   ```

10. Keep the agreement visible: any later change to the agreement is one dated line under `Revisions`.

No gate catches: whether the point value names real judgments, whether the example is concrete, whether the prose is lean. Those are the reviewer's, by design — [reference/review-checklist.md](../reference/review-checklist.md) holds the list.
