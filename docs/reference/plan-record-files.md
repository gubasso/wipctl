# Plan record files

The files of the plan record and the commits that change them: the lane files and their fields, the pending fragment, the touch list, the transition journal, the commit grammar, and the layout of the zone. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## The lane file

```yaml
lane: todo
stories:
  - id: rate-limit-the-search-endpoint
    type: story
    points: 2
    delay_cost: immediate
    summary: "Callers of the search endpoint are limited per token, and the limit is announced in the response headers rather than discovered by being cut off."
    needs: [secure-session-storage]
    touches: [src/http/limit.rs, src/http/mod.rs]
    epic: session-hardening
```

## Entry fields

| Field          | Requirement            | Meaning                                                                                                 |
| -------------- | ---------------------- | ------------------------------------------------------------------------------------------------------- |
| `id`           | required               | the title's slug; the story document's filename stem                                                    |
| `type`         | required               | `story`, `spike`, or `chore`                                                                            |
| `points`       | required               | `1`, `2`, or `3`, counting judgments                                                                    |
| `delay_cost`   | optional               | `immediate` or `deferred`; absence means the ordinary middle position                                   |
| `summary`      | required               | one double-quoted line, 60 to 400 characters, no surrounding whitespace                                 |
| `needs`        | optional               | one-line flow sequence of unique ids this entry depends on, each naming this record or an attached peer |
| `touches`      | required               | one-line flow sequence of at most 12 unique exact paths the work probably changes                       |
| `epic`         | optional               | one id naming an existing epic document                                                                 |
| `tags`         | optional               | one-line flow sequence of unique slugs; nothing gates on a tag                                          |
| `branch`       | optional               | the code branch carrying the work                                                                       |
| `session`      | work lanes only        | one `<alias>#<key>` token naming the coding session acting on the entry, through a declared adapter     |
| `delivered`    | closed lane only       | the commit that delivered the work                                                                      |
| `outcome`      | required when closed   | `done`, `cut`, or `reshaped`                                                                            |
| `closed`       | required when closed   | the close date, and the velocity source                                                                 |
| `succeeded_by` | required when reshaped | one id                                                                                                  |
| `note`         | optional               | free prose about handling; nothing parses it                                                            |

Points read as follows. `1` is confirmation only: named tests settle acceptance, and at most one unchanged contract is involved. `2` is one judgment: an interface, name, message, or existing contract needs human reasoning. `3` is two judgments, or one that is hard to reverse, such as a schema or a security control.

Rules that bind this section: `lane-file:the-points-scale-counts-judgments`, `lane-file:a-summary-is-one-bounded-line`, `touch-prediction:a-touch-list-is-bounded-and-exact`, `lane-file:a-close-field-appears-only-when-closed`, `lane-file:a-reshaped-close-names-its-successor`.

## Lane semantics

| Lane      | Meaning                                                     |
| --------- | ----------------------------------------------------------- |
| `backlog` | agreed, not scheduled                                       |
| `todo`    | scheduled; the computed head is what to start               |
| `doing`   | in flight; every dependency closed and no blocking question |
| `review`  | waiting on a reader; the same entry gate as in flight       |
| `closed`  | finished, ordered by close date ascending                   |

Rules that bind this section: `ranking:a-work-lane-entry-is-unblocked`.

## The pending fragment

```yaml
lane: backlog
captured: "2026-08-14T09:41:07Z"
entry:
  id: rate-limit-the-search-endpoint
  type: story
  points: 2
  summary: "Callers of the search endpoint are limited per token, and the limit is announced in the response headers rather than discovered by being cut off."
  needs: ["payments#secure-session-storage"]
  touches: [src/http/limit.rs]
  epic: session-hardening
```

A fragment's `needs` accepts the same two forms a lane entry's does, and the double-quoting rule applies here too.

A fragment's `touches` mirrors the lane entry's field and carries the same bound, and it is optional here for the reason points and summary are. Capture writes what it knows, and the fragment is committed as captured. The drain refuses every field a lane entry requires, so a fragment reaching the drain without its prediction is refused by name, and `pending-fragment:a-fragment-requires-only-id-and-type` is unchanged. A fragment carries no session claim, because a claim belongs to a work lane and a fragment claims a planning one.

Rules that bind this section: `lane-file:a-prefixed-dependency-is-quoted`, `touch-prediction:a-touch-list-is-bounded-and-exact`, `pending-fragment:a-fragment-requires-only-id-and-type`, `pending-fragment:an-unknown-field-is-a-failure`.

## The touch list

```yaml
touches: [src/http/limit.rs, src/http/mod.rs, docs/specs/SPEC-http.md]
```

```yaml
touches: []
```

The key is required on every lane entry, so an absent key is an incomplete entry rather than a second meaning. A textual sentinel such as `none` states nothing the empty sequence does not.

The list is a one-line flow sequence, because `lane-file:the-yaml-subset-is-narrow` admits nothing else.

The checker reads each outbound item's leading inline-code token, removes the `new:` promise marker where the item carries one, and requires the resulting exact path in `touches`. The check spans a document and a lane file, so it belongs to the cross-file half rather than to either schema.

Rules that bind this section: `touch-prediction:a-touch-list-is-bounded-and-exact`, `touch-prediction:a-touch-list-is-stated-never-verified`, `touch-prediction:an-amended-path-is-a-touched-path`, `lane-file:the-yaml-subset-is-narrow`, `validation:every-check-has-one-owner`.

## The transition journal

One append-only stream per entry, one event per line, exactly three non-empty tab-separated fields.

```text
2026-08-14T09:52:31Z	todo	doing
2026-08-15T16:03:10Z	doing	review
2026-08-16T11:20:44Z	review	closed
```

Field one is the instant, field two is the lane left, and field three is the lane entered or a terminal token.

Rules that bind this section: `transition-journal:an-event-is-three-non-empty-fields`.

## The commit grammar

```text
plan: capture <id>
plan: move <id> from <from> to <to>
plan: close <id> as <outcome>
plan: land <n> fragments
plan: rename <old> to <new>
plan: delete <id>
plan: reconcile <n> changes
plan: resolve <id> keeping <side>
```

Rules that bind this section: `transactions:the-commit-grammar-is-the-tools`.

## The zone

```text
plan-repo/
  .wipctl/plan.toml             the plan's whole configuration: both
                                identities, the cadence, the peers it
                                names, and the sources it references
  README.md                     orientation for a reader arriving at the zone
  AGENTS.md                     the method, self-sufficient, written by the scaffold
  charter.md                    what the project is for, pillars, no-gos, cadence
  open-questions.md             questions that block named entries
  watches.md                    outside items that block named entries; earned
  lanes/                        backlog, todo, doing, review, closed
  stories/<id>.md               one document per story, optional sibling directory
  epics/<id>.md                 one document per epic; earned, not scaffolded
  initiatives/<id>.md           one document per initiative; earned the same way
  journal/<id>.tsv              one transition stream per entry; created on first move
  pending/<id>.yml              one fragment per captured entry; created on demand
```

Rules that bind this section: `zone-layout:every-lane-file-exists`.
