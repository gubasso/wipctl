# Writing a story

One phase, from an itch to a gated entry. The heading contract the document must satisfy is [reference/record/documents.md](../reference/record/documents.md); this page is the order to work in.

1. Mint the identity:

   ```text
   $ wipctl new story "<title>"
   ```

   writes the document and its pending fragment, with the entry's `points` and `summary` left absent for you to fill. The id is the title's slug, checked against the live record and every tombstone: a taken or burned id is refused on the spot, and the answer is to rephrase the title (or pass `--id` with a qualifying postfix when the title is right as written). Hand-minting without the tool is the same grammar without the guarantees: slugify the title, confirm nothing live or tombstoned holds the slug, copy the shipped template to `stories/<slug>.md`, and keep the filename and the title line in agreement.

2. Name the outcome, not the mechanism. `Goal` says what becomes true, in the reader's terms.
3. Show it from the reader's side. `Example` carries concrete values and real output in a fenced block — a description of a transcript is not a transcript.
4. Separate `Core` from the ordered cuts in `In scope`. If the story wants four points, split it along the judgments its acceptance names, before writing more.
5. Wire the references. `Reads`: the individual sources a session must load, stated as claims linked to stable anchors, not line numbers. `Amends`: the documents the work must leave changed — or `None`. Mark a document the work will create with `new:`; where the work changes a rule the host identifies by id, type the clause: `ADDED`, `MODIFIED`, or `REMOVED`, then the rule id in inline code. [wiring-host-docs.md](./wiring-host-docs.md) is the full sequence.
6. Make acceptance falsifiable. Every assertion names the test that proves it; the point value counts the judgments in that section a test cannot settle.
7. Pre-authorise escapes in `Rabbit holes`: every known trap gets a written way out. An escape written after hitting the trap is a revision.
8. Write the difference and nothing else. Delete the sentence; if a durable fact disappeared, it was in the wrong file — durable facts live in the documents `Amends` names.
9. Complete the fragment — the entry's `points` and `summary`, and any field the capture left absent — and land it, or add the lane entry by hand; the drain refuses an incomplete fragment. Then:

   ```text
   $ wipctl validate
   ```

10. Keep the agreement visible: any later change to the agreement is one dated line under `Revisions`.

No gate catches: whether the point value names real judgments, whether the example is concrete, whether the prose is lean. Those are the reviewer's, by design — [reference/review-checklist.md](../reference/review-checklist.md) holds the list.
