# Identifiers and tokens

The id grammar, the slug function, and every token that names a thing in the record or outside it: the prefixed id, the session reference, the source reference, and a node of the dependency graph. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## Grammar and slugification

```text
[a-z0-9]+(-[a-z0-9]+)*
```

A question id carries the same grammar behind the fixed prefix `Q-`.

```text
1. lowercase
2. apostrophes are removed rather than hyphenated      don't -> dont
3. every other run of characters outside a-z0-9 becomes a single hyphen
4. leading and trailing hyphens are trimmed
```

```text
id  ==  slugify(title)                     the ordinary case
id  ==  slugify(title) + "-" + <postfix>   where the slug alone was taken
```

Rules that bind this section: `ids:an-id-follows-one-grammar`.

## An id and a prefixed id

A dependency can name an entry in another plan, so two shapes appear where one used to.

```text
an id             one token; never parsed; names one thing in one record

a prefixed id     an alias, a separator, and an id; split exactly once on
                  the first separator; the alias half goes to the peer
                  table and the id half is carried whole
```

After the split, the id half is an ordinary id under the ordinary rule. So the opacity rule, `ids:an-id-is-opaque-to-every-consumer`, survives the addition: it is the id that is never parsed, and a prefixed id is a pair that was never one id.

Rules that bind this section: `ids:a-prefixed-id-is-split-once`, `ids:an-id-is-opaque-to-every-consumer`.

## The session reference

A session reference is one token, `<alias>#<key>`. The alias names a table in the `session_adapters` section. The key is whatever opaque token that adapter's commands accept: an identifier, a stable name, or another native locator. The record never parses it.

```yaml
- id: rate-limit-the-search-endpoint
  session: "sessions#4f19a2"
```

The token is double-quoted in a lane file, for the reason `lane-file:a-prefixed-dependency-is-quoted` gives about the comment marker. A session is optional, and its absence means the entry makes no session claim. That covers work performed without a resolvable coding session, so nobody needs a synthetic adapter alias.

Rules that bind this section: `agent-sessions:a-session-reference-is-one-token`, `lane-file:a-prefixed-dependency-is-quoted`.

## The source reference

A source reference is one token, `<alias>#<key>`. The alias names a table in the `sources` section. The key is the item's own name in that system, and the record never parses it.

```markdown
## Sources

- `issues#412` — the report this story answers
- `tickets#PAY-88` — the same work, tracked by the platform team
```

The section carries paths for no reason and resolves against no root, so it is not a path reference. Its item shape follows the same rule the other reference sections carry: the leading inline-code token is the datum, and everything after it is prose.

Rules that bind this section: `external-sources:a-source-ref-is-stated-and-never-verified`.

## A node, and how it is written for a reader

The graph is built over pairs, and the pair is what makes the proof sound.

```text
a node       (plan_id, entry_id). The plan id, not the alias, because an
             alias is local to the plan that declares it and two plans
             can spell one peer two ways.

an edge      one dependency, resolved in the record it was written in,
             and pointing at the pair it names.
```

Rendering is the other half, and it is relative to the plan the invocation resolved to.

```text
the invoking plan's own entry     the bare id
a peer this plan declares         <alias>#<id>, using this plan's alias
a plan no row of this plan names  <plan_id>:<id>
```

The third case is real whenever a cycle runs through a plan two hops away. Reprinting the middle plan's own alias prints a name that resolves in no table the reader has.

The third form uses a separator no dependency accepts, so it cannot be read as a local dependency. The plan identity resolves everywhere and belongs to no alias table.

This is also why a cycle report is stable when the same cycle is found from either side. The nodes are the same pairs, and only the rendering changes.

Rules that bind this section: `ranking:a-node-renders-relative-to-the-invoking-plan`.
