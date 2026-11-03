# Plans and peers

One plan is one project's record. A story here can depend on a story in another project's plan, and the two records stay separate. This chapter explains the two identities, the local names, and the checker's refusal to fetch.

## What a peer is

A peer is another project's plan repository, named by this plan and attached on this machine. Naming it is a committed fact in the plan configuration's `peers` section. Attaching it is a local fact: a locally named slot under the data directory holds a clone.

Once attached, a peer is a plan. It is validated in full, its tombstones are read, its lanes are read, and any verb that resolves a record can be pointed at it. There is one idea of a plan in this method, and a peer is an instance of it. Connected plans are one plan with several roots.

A slot holds one plan, whether it arrived as this machine's own project or as a peer another plan named. Once filled, nothing distinguishes the two.

An entry depends on a peer's entry through the field it already has.

```yaml
- id: profile-composition
  type: story
  points: 2
  summary: "Compose a profile from the session identity the payments plan stores."
  needs: [session-token-parsing, "payments#secure-session-storage"]
```

Both ids mean one thing: closed before this entry is eligible. The first is looked up in this record. The second is looked up in the peer's. Eligibility, the graph, and the acyclicity proof treat them identically, because they are the same edge. [../specs/SPEC-lane-file.md](../specs/SPEC-lane-file.md) states the field and its two forms. A dependency on a peer's whole epic is written as a dependency on that epic's last entry, which is what it means.

The separator is `#`. A comment marker in this format must be separated from what precedes it, so `payments#secure-session-storage` is one plain scalar and is never a comment. The quoting rule, `lane-file:a-prefixed-dependency-is-quoted`, is a choice about how the record reads, not a rescue from a parse error. One function splits a prefixed id, once, on the first separator. A second splitter is how the two halves come to disagree.

## Why a plan carries two identities

`project_id` answers which project this working tree belongs to. `plan_id` answers which plan record this is.

Each identity is 128 bits of randomness, minted once and written as 32 lowercase hexadecimal characters. Each is committed by the repository that owns the fact.

Two identities keep settlement local. A changed plan identity edits the plan repository, while the host keeps its project identity unchanged.

The opaque values cost readability. The slot name restores a readable path, and the derived project slug supplies messages. An identity appears only where identity is the subject.

[../specs/SPEC-configuration.md](../specs/SPEC-configuration.md) holds both, and [../specs/SPEC-attachment.md](../specs/SPEC-attachment.md) says where each one enters an invocation.

## Why the alias is local

A peer row binds three names, and they are three separate facts.

```text
payments                                     alias      chosen by THIS plan
9f2c41a08b7d4e63a15c8f02d7e4b619             plan id    declared by THAT plan
https://git.example.org/acme/payments.git    url        a hint; replaceable
```

Two plans can both call a peer `payments` and mean different things. The alias resolves only in its file. The plan id is immutable, while the url remains replaceable. An alias is local to the plan that declares it, so an alias means nothing outside the file that holds it.

A committed file is read by everyone who clones this plan, so a credential in it is a leak whatever the tool prints.

Two operations of the peer writer carry the weight. `add` makes the uid a machine's problem rather than a person's, which is what lets the design afford an opaque identity at all. `alias` makes the alias genuinely local, because a project renames a peer to whatever its own readers understand.

The well-known systems that solve this problem separate the same three names. Bazel shipped without the alias layer, hit name clashes in diamond dependencies, and retrofitted repository mapping. Nix binds a local input name to a flake reference. Go declares the module path in the module itself. Each is a citation for further reading, and this page is complete without following any of them.

## Why the checker refuses rather than fetches

This is the design's centre. The chain is short and each step follows from the one above it.

```text
every answer is a function of committed files on this machine
      ↓
a peer's record is committed files on this machine once it is attached
      ↓
so reading it is the read the checker already does on its own lanes
      ↓
a peer that is not attached is a file the checker cannot read
      ↓
therefore the checker fails and names the attach, rather than fetching
and making its answer depend on the network and the hour
```

The refusal is loud on purpose. A check that runs on one machine and quietly does not run on another is worse than a check that fails. The first teaches a reader that a green result means something it does not.

Attaching is what makes the answer possible, and it is generous. `wipctl attach --peers` walks the whole declared set, transitively, and fills every slot. Checking is a proof, so it demands only the peers a dependency actually reaches. [Two sets, named once](#two-sets-named-once) names both sets and says why they differ.

Go and Nix state the same property in the same order: fetch the graph, then answer about it.

## The closure the checker reads

Two consequences follow, and each is a thing a reader otherwise finds the hard way. Adding one prefixed dependency can make a run start failing on a machine where it passed a minute ago. The new id moved a peer into the read set. Attachment is also transitive through checking, not only through the walk. Reading a peer means checking it, so that peer's read set joins this one.

## Two sets, named once

```text
the declared set   every row of this plan's peers section, plus every row
                   of each of their peers sections, transitively. This is
                   what the peer attachment walk fills.

the read set       every peer a prefixed needs id in this record reaches,
                   plus the peers those peers' own prefixed ids reach,
                   transitively. This is what validation requires attached.
```

The read set is a subset of the declared set. The difference is exactly the rows nothing references. Those rows are legal, the walk fills them, and validation reports them stale rather than demanding them. A machine that attached less than everything then gets a short list rather than a wall.

## Filling a slot

The walk that fills a peer's slot has five steps, which [verb mechanics](../reference/verb-mechanics.md#filling-the-slots) lists. The sequence has five steps because each one prevents a distinct failure.

The reason is the lock's own contract. A writer holds the lock from the deciding read to the commit, and hold times are subsecond. A clone over a network is neither bounded nor subsecond. Cloning straight into the slot is the other way to get this wrong. A reader resolving that uid then sees a directory that is not yet a verified plan. Re-reading the table under the lock is what makes two concurrent additions of one plan a refusal rather than a duplicate row.

The walk fills rows no entry references, because a row this plan declared is a plan this plan meant to work with. The walk fetches so that no check ever fetches. What it fills is a superset of what validation demands, and the peers domain names both sets.

## What the table is not

- The table is not a registry. Nothing allocates an alias or approves an identity. Two plans that name one peer write independent rows.
- The table is not authoritative about the peer. The peer's own `.wipctl/plan.toml` is, and the walk refuses a clone that disagrees with the row.
- The table is not a cache. Reading a peer is a read of that peer's slot, and no peer state is stored here.

## The peer view and the health report

`peers` is a reader in the family of `epics` and `initiatives`, and a view belongs beside the views. The health report names whether each declared peer is attached and stops there. A report that repeats a view is a second copy of it.

A prefixed dependency that resolves to no entry has no message in the peer view, because validation answers it. A fourth, vaguer message in the peer view gives one condition two answers, and the vaguer one is the one a reader meets first.

## The plan flag

The plan flag reaches one hop, because the closure is what the checker proves over and an alias names only what its own table declares. `init` is the one verb worth a sentence. It creates a plan for the project the caller is inside, so `wipctl --plan payments init` reads as an instruction to create a plan inside a plan. That is not a thing, and refusing it is better than choosing a meaning for it.

## Parity, and what it costs

A peer is a plan, so every verb that resolves a record accepts the flag, writers included. `wipctl --plan payments board` reads that plan's board. `wipctl --plan payments move <id> doing` moves an entry in that plan, holds that plan's writer lock, and commits to that plan's trunk.

Write access is the operator's business. The tool refuses nothing on the basis of who owns the peer, because it has no permission model and no pillar allows one. A push the forge rejects is reported by replication in the relayed-transport class, which is where a permission failure surfaces.

The cost is worth stating. A verb that writes can now write into another team's record from a shell that happens to be in this project. That is what parity means. The mitigation is that the alias is already in a committed table, not that the tool second-guesses the operator.

## What it does not change

- Resolution for a bare invocation. Without the flag, everything resolves as it did.
- Whether a verb is multi-plan. The flag selects one plan, so the batch mode stays the one the replication domain defines.
- How far a name reaches. The flag resolves through this plan's own table, so it can only name a plan this record declared itself.
- Whether a plan exists. An alias whose slot is empty is the ordinary unattached-peer failure, naming the walk that fills it.

## What the trade gives up, and what it keeps

Given up:

- A plan whose dependencies reach peers cannot be validated alone. A clone of it on a bare machine is not enough. Declaring a peer nothing depends on costs nothing until an entry uses it.
- The other team's pace gates this team's board. An entry that needs an unclosed peer entry cannot enter the work lanes. That is what a dependency means, and it was the point.
- Another team's broken record fails this team's check. The diagnostic names the peer, the slot, and the failing check, so the reader knows whose record to fix.

Kept:

- The record still answers offline, and still answers from files.
- It now proves its graph acyclic across every plan it reaches, which no amount of prose in a note field does.
- One plan is still one project. Nothing merges, and no programme is created.

## The charter still holds

The peer design preserves the charter's pillars and no-gos. Each one a reader might suspect is named here with the reason it holds.

```text
pillar 2, the host stays unassumed
  the host file is untouched; the uid lives in the plan repository

pillar 4, plain files, one live copy per project
  unchanged; a peer is one live copy of that project, read where it lives

no-go 1, no default for a value a project must declare
  every peer field is required; nothing is guessed

no-go 2, no second store of what the record or filesystem holds
  reading a peer is a read, never a copy; no peer state is stored here

no-go 4, no priority a machine authors
  a prefixed id affects derived eligibility and order as a fact a person wrote
```

A record's validity depends on the other records attached beside it. This page says so plainly rather than hiding it as an unstated consequence. [../specs/SPEC-validation.md](../specs/SPEC-validation.md) is where it is specified.

## The half no linter holds

The checker proves the graph. It cannot tell the payments team that the profile team is waiting on them. Nothing in either record stores who depends on it, so a delete here breaks a reference there, and that plan learns at its next check.

That is a review responsibility, and [../specs/SPEC-review.md](../specs/SPEC-review.md) carries it. It is the charter's first pillar working as intended: a rule the linter cannot check is stated as a review responsibility rather than dropped.
