# Writing a story

One phase, from an itch to a gated entry. [specs/SPEC-documents.md](../specs/SPEC-documents.md) holds the heading contract the document must satisfy. This page is the order to work in.

1. Mint the identity:

   ```text
   $ wipctl new story "<title>"
   ```

   That writes the document and its pending fragment. The entry's `points` and `summary` are left absent for you to fill. The id is the title's slug, checked against the live record and every tombstone. A taken or burned id is refused on the spot. The answer is to rephrase the title, or to pass `--id` with a qualifying postfix when the title is right as written.

   Hand-minting without the tool is the same grammar without the guarantees. Slugify the title. Confirm that nothing live or tombstoned holds the slug. Copy the shipped template to `stories/<slug>.md`. Keep the filename and the title line in agreement.

2. Name the outcome, not the mechanism. `Goal` says what becomes true, in the reader's terms.
3. Show it from the reader's side. `Example` carries concrete values and real output in a fenced block. A description of a transcript is not a transcript.
4. Separate `Core` from the ordered cuts in `In scope`. If the story wants four points, split it along the judgments its acceptance names, before writing more.
5. Wire the references. `Reads` names the individual documents a session must load. State each as a claim linked to a stable anchor, never to a line number. `Amends` names the documents the work must leave changed, or the literal `None`. Mark a document the work will create with `new:`. Where the work changes a rule the host identifies by id, type the clause. Write `ADDED`, `MODIFIED`, or `REMOVED`, then the rule id in inline code. `Sources` names the outside items the work answers, one `<alias>#<key>` token per line, or the literal `None`. [wiring-host-docs.md](./wiring-host-docs.md) is the full sequence.
6. Make acceptance falsifiable. Every assertion names the test that proves it. The point value counts the judgments in that section a test cannot settle.
7. Pre-authorise escapes in `Rabbit holes`. Every known trap gets a written way out. An escape written after hitting the trap is a revision.
8. Write the difference and nothing else. Delete the sentence. If a durable fact disappeared, it was in the wrong file, because durable facts live in the documents `Amends` names.
9. Complete the fragment. Fill the entry's `points` and `summary`, and any field the capture left absent. Set the optional `class` when the cost of delay is immediate (`expedite`) or late (`intangible`). An absent class sorts as `standard`. Then land it, or add the lane entry by hand. The drain refuses an incomplete fragment.

   ```text
   $ wipctl validate
   ```

10. Keep the agreement visible. Any later change to the agreement is one dated line under `Revisions`.

No gate catches whether the point value names real judgments, whether the example is concrete, or whether the prose is lean. Those are the reviewer's, by design. [specs/SPEC-review.md](../specs/SPEC-review.md) holds the list.
