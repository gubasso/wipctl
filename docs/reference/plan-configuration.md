# Plan configuration

The two configuration files of a plan, section by section: the project file in the host, and the plan configuration at the plan zone root. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it. The session adapter section is on [session adapters](./session-adapters.md#the-generic-adapter).

## Configuration shape

```toml
# .wipctl/project.toml
project_id = "a1b2c3d4e5f60718293a4b5c6d7e8f90"

# .wipctl/plan.toml
project_id = "a1b2c3d4e5f60718293a4b5c6d7e8f90"
plan_id = "9f2c41a08b7d4e63a15c8f02d7e4b619"
superseded_plan_ids = ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]

[ranking]
preferences = ["delay-cost", "points-ascending"]
```

Both identities are 128 random bits written as exactly 32 lowercase hexadecimal characters. Each is minted once, immutable, opaque, and parsed by nothing.

The plan file also holds its window and its optional `peers`, `sources`, and `session_adapters` tables. The superseded list keeps discarded plan identities resolvable.

Rules that bind this section: `configuration:the-plan-declares-a-global-identity`.

## The declared source

The `sources` section sits in `.wipctl/plan.toml` at the plan zone root. It names each system the project's artifacts reference, one table per alias. The host repository gains nothing: its footprint stays the one project file.

```toml
# the sources section of .wipctl/plan.toml

[sources.issues]
url_template = "https://tracker.example/issues/{key}"
list_command = ["<the command that lists open items>", "--format", "json"]

[sources.tickets]
url_template = "https://tickets.example/browse/{key}"
list_command = ["<the command that lists open items>"]
```

The `sources` half of `plan.schema.json` owns the section's shape, in the configuration spec's companion directory. The cross-file checker owns every fact spanning the section and a document.

Rules that bind this section: `external-sources:a-declared-source-carries-a-template-and-a-command`.

## The peers section

The `peers` section sits in `.wipctl/plan.toml` at the plan zone root. There is one table per peer, keyed by the alias.

```toml
[peers.payments]
plan_id = "9f2c41a08b7d4e63a15c8f02d7e4b619"
urls = ["https://git.example.org/acme/payments-plan.git"]

[peers.platform]
plan_id = "4c81d0e7f39a4b25861d7c04e9a2f358"
urls = [
  "https://git.example.org/acme/platform-plan.git",
  "https://git.example.net/mirrors/platform-plan.git",
]
```

```text
<alias>    the table key. The slug grammar, [a-z0-9]+(-[a-z0-9]+)*. Local
           to this plan and meaningful nowhere else.

plan_id    required. The peer's plan identity, 32 lowercase hexadecimal
           characters. This is the identity, and everything else is a hint.

urls       required. A non-empty array of locations where the peer's plan
           repository can be cloned. Order is the order the walk tries them.
```

This table is the only place an alias resolves. A dependency written as `<alias>#<id>` reads the alias here, in the record the dependency was written in, and never in any other plan's table. The shape is the `peers` half of `plan.schema.json`, in the configuration spec's companion directory.

Rules that bind this section: `peers:a-row-carries-a-uid-and-a-location`, `peers:an-alias-is-local-and-unique`, `plan-targeting:the-flag-selects-from-this-plans-table`.

## Ranking preferences

The scaffold writes both preference names in that order. `delay-cost` places `immediate` first, an absent value in the ordinary middle position, and `deferred` last. `points-ascending` places smaller work first. A project can omit either preference or declare an empty list.

The ranking domain owns the accepted preference names and their uniqueness. The configuration domain owns the required declaration. An empty list states that the project chooses no ranking preference. It is not a default.

Rules that bind this section: `ranking:a-delay-cost-preference-orders-before-size`, `lane-file:absent-delay-cost-is-ordinary`, `configuration:the-ranking-section-is-required`, `ranking:a-preference-name-is-known-and-appears-once`, `configuration:no-key-has-a-read-time-default`.
