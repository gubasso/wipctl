# Views

The glyph tiers every view draws from, and the peer view. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## Glyph tiers

| Tier             | Use                                                                                                            |
| ---------------- | -------------------------------------------------------------------------------------------------------------- |
| ASCII            | the universal fallback, selected by the plain flag                                                             |
| Block Elements   | the default drawing tier; every glyph is display-width 1                                                       |
| Box Drawing      | junctions for the ladder graph, from the four direction bits                                                   |
| Braille patterns | a density tier, weakest in font coverage, announced as braille by screen readers; no baseline view requires it |

A cross-plan marker is drawn from the default tier, and its plain-flag form comes from the ASCII tier, like every other badge. It carries one fact, which is that a dependency of this row reaches another plan, so one width-1 glyph is the whole of it.

Rules that bind this section: `rendering:a-column-label-is-ascii`, `rendering:a-cross-plan-marker-is-one-glyph`.

## The peer view

```text
wipctl peers [--json]
```

The view reads the table, this record's five lane files, and the slot of each attached peer.

```text
$ wipctl peers
payments   9f2c41a08b7d4e63a15c8f02d7e4b619   attached  91a47c0d
           https://git.example.org/acme/payments-plan.git
           2 entries need it
             profile-composition   needs secure-session-storage   todo
             profile-export        needs token-rotation           closed

platform   4c81d0e7f39a4b25861d7c04e9a2f358   not attached
           https://git.example.net/mirrors/platform-plan.git
           run 'wipctl attach --peers'
```

The machine format is `peer-list.schema.json`, in the companion directory of the peers spec. A field is absent rather than null where the state does not have it, which is the rule an entry's optional fields already follow. Stale is derived from the reference list rather than stated beside it, so the schema holds the two in agreement.

A prefixed dependency that resolves to no entry has no message in the peer view. The validation domain owns that condition and answers it three ways from the peer's own tombstones.

Rules that bind this section: `cli-conventions:a-machine-format-is-verb-local`, `validation:a-prefixed-dependency-resolves-or-fails`.
