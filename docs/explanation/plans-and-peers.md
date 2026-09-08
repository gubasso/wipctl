# Plans and peers

One plan is one project's record. A story here can depend on a story in another project's plan, and the two records stay separate. This chapter says what a peer is, why a plan carries two names, and why the checker refuses rather than fetches.

## What a peer is

A peer is another project's plan repository, named by this plan and attached on this machine. Naming it is a committed fact in `peers.toml`. Attaching it is a local fact: a slot under the data directory holds a clone.

Once attached, a peer is a plan. It is validated in full, its tombstones are read, its lanes are read, and any verb that resolves a record can be pointed at it. There is one idea of a plan in this method, and a peer is an instance of it. Connected plans are one plan with several roots.

An entry depends on a peer's entry through the field it already has.

```yaml
- id: profile-composition
  type: story
  points: 2
  summary: "Compose a profile from the session identity the payments plan stores."
  needs: [session-token-parsing, "payments#secure-session-storage"]
```

Both ids mean one thing: closed before this entry is eligible. The first is looked up in this record. The second is looked up in the peer's. Eligibility, the graph, and the acyclicity proof treat them identically, because they are the same edge. [../specs/SPEC-lane-file.md](../specs/SPEC-lane-file.md) states the field and its two forms.

## Why a plan carries two names

`project_id` answers which plan on this machine. `plan_uid` answers which plan anywhere.

The first is a slug, minted against this machine's registry, and it is a path segment under the data directory. The second is 128 bits of randomness, minted once when the plan is created, and it is a value another repository commits.

A machine-local slug cannot be the second one. Two operators mint the same slug without ever meeting, because uniqueness is decided against one machine's registry and not globally. Two names for one thing is a real cost. It is paid because a reference outward has to survive a rename, a rehost, and a move between forges, and no server allocates it.

[../specs/SPEC-configuration.md](../specs/SPEC-configuration.md) holds both, and [../specs/SPEC-attachment.md](../specs/SPEC-attachment.md) says where each one enters an invocation.

## Why the alias is local

A peer row binds three names, and they are three separate facts.

```text
payments                                     alias      chosen by THIS plan
9f2c41a08b7d4e63a15c8f02d7e4b619             uid        declared by THAT plan
https://git.example.org/acme/payments.git    url        a hint; replaceable
```

Two plans can both call a peer `payments` and mean different things, and nothing has to agree for that to be safe. The alias resolves in the file it was written beside, and nowhere else. The uid is the identity, and it never changes. The url is a locator, and changing it changes nothing else.

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

Attaching is what makes the answer possible, and it is generous. `wipctl attach --peers` walks the whole declared set, transitively, and fills every slot. Checking is a proof, so it demands only the peers a dependency actually reaches. [../specs/SPEC-peers.md](../specs/SPEC-peers.md) names both sets and says why they differ.

Go and Nix state the same property in the same order: fetch the graph, then answer about it.

## What the trade gives up, and what it keeps

Given up:

- A plan whose dependencies reach peers cannot be validated alone. A clone of it on a bare machine is not enough. Declaring a peer nothing depends on costs nothing until an entry uses it.
- The other team's pace gates this team's board. An entry that needs an unclosed peer entry cannot enter the work lanes. That is what a dependency means, and it was the point.
- Another team's broken record fails this team's check. The diagnostic names the peer, the slot, and the failing check, so the reader knows whose record to fix.

Kept:

- The record still answers offline, and still answers from files.
- It now proves its graph acyclic across every plan it reaches, which no amount of prose in a note field does.
- One plan is still one project. Nothing merges, and no programme is created.

## The charter is not edited

Six phases of change land under this chapter, and the charter states the same pillars and no-gos it always did. That is a checked fact rather than an oversight, so each pillar a reader will suspect is named here with the reason it still holds.

```text
pillar 2, the host stays unassumed
  the host file is untouched; the uid lives in the plan repository

pillar 4, plain files, one live copy per project
  unchanged; a peer is one live copy of that project, read where it lives

no-go 1, no default for a value a project must declare
  every peer field is required; nothing is guessed

no-go 2, no second store of what the record or filesystem holds
  reading a peer is a read, never a copy; no peer state is stored here

no-go 4, no rank a machine computes
  a prefixed id decides legality, never priority
```

One property is new, and the charter does not state it. A record's validity now depends on the other records attached beside it. This page says so plainly rather than smuggling it in as an unstated consequence. [../specs/SPEC-validation.md](../specs/SPEC-validation.md) is where it is specified. If a later change decides the property belongs in the charter, that is a charter edit with its own reasoning.

## The half no linter holds

The checker proves the graph. It cannot tell the payments team that the profile team is waiting on them. Nothing in either record stores who depends on it, so a delete here breaks a reference there, and that plan learns at its next check.

That is a review responsibility, and [../specs/SPEC-review.md](../specs/SPEC-review.md) carries it. It is the charter's first pillar working as intended: a rule the linter cannot check is stated as a review responsibility rather than dropped.
