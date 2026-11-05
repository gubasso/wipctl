# Diagnostics

The diagnostic classes, the messages of the class that relays another tool's output, and the faults and messages of each domain. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## The two diagnostic classes

| Class   | Meaning                                                                | Exit     |
| ------- | ---------------------------------------------------------------------- | -------- |
| failure | the record states something incorrectly and no tool may decide the fix | non-zero |
| warning | a fact the reader decides about                                        | zero     |

Rules that bind this section: `validation:a-warning-never-reaches-the-exit-code`.

## Relaying another tool's output

One class of failure comes from a tool the command drove rather than from the record. The replication substrate cannot complete a fetch, a fast-forward, or a push, or a slot is not as the tool left it. The message relays that tool's own output, states what changed on disk, and then orients the reader. The reader is as likely to be a coding agent as a person, and the orientation is written for both.

```text
git could not complete the operation                                      exit 1
  wipctl: sync of payments-acme stopped: git reported an error during fetch
  wipctl: ---- git output, first 200 lines; whole output in the file below ----
  <git's stdout and stderr, in order, with credentials in any url replaced
   by <redacted> and control bytes escaped, and nothing else changed>
  wipctl: ---- end of git output ----
  wipctl: whole output: <state dir>/sync/payments-acme-2026-09-06T11-04-12Z.log
  wipctl: on disk: the plan trunk on this machine is unchanged, and nothing
          was pushed
  wipctl: before you act, read the state in three places and make sure
          that they agree with what you expect:
  wipctl:   local    git -C <slot> log --oneline -10 <trunk>
  wipctl:   remote   git -C <slot> log --oneline -10 origin/<trunk>
  wipctl:   stray    git -C <slot> status --short; git -C <slot> branch --all;
                     git -C <slot> stash list
  wipctl: every transition on both sides must land in the result. A merge
          that drops one re-opens a story that was closed, forgets a claim
          someone made, or loses a capture.
  wipctl: do not force-push, do not reset the trunk, and do not settle a
          difference by picking a side textually. When both sides moved
          one entry, 'wipctl resolve <id> --keep here|remote' is the tool.
  wipctl: when the state is clear, re-run 'wipctl sync'

the slot is not as the tool left it                                        exit 1
  wipctl: sync of payments-acme stopped: the slot is not on the plan trunk
  wipctl: ---- git output, first 200 lines; whole output in the file below ----
  <git status --short and git branch --show-current, under the same rule>
  wipctl: ---- end of git output ----
  wipctl: whole output: <state dir>/sync/payments-acme-2026-09-06T11-04-12Z.log
  wipctl: on disk: nothing was fetched and nothing was pushed
  wipctl: something outside wipctl checked out a branch, left changes
          unstaged, or detached HEAD in the plan repository. Read the
          three places above before touching anything; the uncommitted
          state may be a plan change that has not landed yet.
  wipctl: return the slot to the trunk with the changes landed or
          deliberately discarded, then re-run 'wipctl sync'
```

Five properties hold across the class.

- The tool's output is relayed between two marker lines, in stream order, with nothing paraphrased and nothing summarized. A reader debugging a transport failure sees what the transport said.
- Two things change on the way through, and only these two. Credentials inside a url become `<redacted>`, and control bytes are escaped. A diagnostic is copied into terminals, transcripts, and continuous-integration logs. Relaying a credential leaks it further than it started, and relaying raw control bytes hands a remote server the reader's terminal.
- The excerpt is bounded and the whole output is kept. The message carries the first 200 lines and names a file holding everything, so fidelity is in the file and the terminal stays readable. The bound is a number in the rule, because an implementer who has to choose one will choose a different one.
- The on-disk statement comes next, so a reader knows whether anything moved before reading further. Then the three places to read, the same three in every message of the class, so an agent learns one procedure.
- The orientation names the failure that matters, which is a lost state. It names the verbs that are safe and the actions that are not, and it addresses the reader as you.

Every line of the class is a diagnostic, so all of it goes to the diagnostic stream. The relayed lines carry the prefix on the marker lines rather than on the tool's own text, so a reader can lift the block unchanged.

The plan trunk has one published branch and its history is append-only, so a stray branch here means one that something outside the tool created. The message says so rather than suggesting the plan has branches to reconcile.

Rules that bind this section: `messages:relayed-output-is-redacted-and-bounded`, `cli-conventions:stdout-carries-data-alone`.

## Agent session faults

Each fault names its own resolution.

- A session reference naming an undeclared alias. This is a failure, exit 1. The message names the alias, the entry, and the `session_adapters` section, and it offers the two resolutions: declare the alias, or correct the reference.
- A malformed reference token. This is a failure, exit 1. The message shows the token and the `<alias>#<key>` shape it must carry.
- A declared inspect command that is absent, exits non-zero, times out, or emits an unreadable object. This is a degradation rather than a failure. The reading is unknown, one line on the error stream names the alias and the command, and the view still reports every other row.
- A failed contact. This is a failure, exit 1. The message names the alias, the key, and that the message was not accepted, and it leaves the record unchanged.

Rules that bind this section: `agent-sessions:a-fault-takes-its-listed-class`.

## Attachment resolution faults

Each resolution failure names its own resolution.

- No `.wipctl/project.toml` on the upward walk. Exit 2, naming the scaffold at the project root.
- A `.wipctl` directory on the walk with no `project.toml` in it. Exit 2, naming that directory. The walk reports and stops rather than passing over it. Passing over it attaches the operator to a project further up the tree.
- The project is not attached on this machine. Exit 1, naming both attach forms.
- The attached plan repository is gone from disk. Exit 1, naming re-attachment.
- The two configuration files disagree. Exit 1, naming the host value, the plan value, and the choice between re-attaching and correcting whichever file is wrong.
- The scaffold finds an existing project file. Exit 1, reporting the id it found and naming attach for a machine that lacks the plan.
- A clone that is not a plan repository. Exit 1, naming the create form as the way to start one.
- A clone carrying no `plan_id`. Exit 1, naming the upgrade form that gives a record which predates the key an identity of its own.
- A clone whose uid is already in a slot. Exit 1, naming the slot that holds the plan and the location offered. The message says that one plan gets one slot. A copy meant to be a plan of its own needs an identity of its own, which no verb mints yet.

```text
two slots claim one project                                       exit 1
  wipctl: two slots hold a plan declaring this project
  wipctl:   payments      plan 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl:   payments-old  plan 4c81d0e7f39a4b25861d7c04e9a2f358
  wipctl: one project has one plan; detach the copy, and keep the plan
          whose plan_id the peers around you name
```

Rules that bind this section: `attachment:a-resolution-fault-takes-its-listed-exit`.

## External source faults

Each fault names its own resolution.

- A source reference naming an undeclared alias. This is a failure, exit 1. The message names the alias, the document, and the `sources` section, and offers the two resolutions: declare the alias, or correct the reference.
- A malformed reference token. This is a failure, exit 1. The message shows the token and the `<alias>#<key>` shape it must carry.
- A declared source that no reference reaches. This is a warning. It reaches the reader and never the exit code, because a source declared before its first reference is a legal first draft.
- A list command that is absent, exits non-zero, or emits unreadable output. This is a degradation, not a failure. One line on the error stream names the alias, the command, and the reader's next step. The view still exits on what the other sources answered.
- A view run where the `sources` section is absent. This is a usage error, exit 2. The message names the section and says that the project declares no source yet.

Rules that bind this section: `external-sources:a-fault-takes-its-listed-class`.

## Peer attachment messages

Each failure names its resolution.

```text
the last url would be removed                                             exit 1
  wipctl: peer 'payments' is left with no url
  wipctl: a row keeps at least one location; add the replacement first,
          or drop the peer with 'wipctl peer remove payments'

a row still in use                                                        exit 1
  wipctl: peer 'payments' cannot be removed: 2 entries still name it
  wipctl:   profile-composition   needs payments#secure-session-storage
  wipctl:   profile-export        needs payments#token-rotation
  wipctl: clear the references first, or rename the alias instead of
          removing the row

the clone declares another plan_id                                        exit 1
  wipctl: peer 'payments' cloned from
          https://git.example.org/acme/payments-plan.git
  wipctl:   the peers section expects 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl:   plan.toml declares 4c81d0e7f39a4b25861d7c04e9a2f358
  wipctl: the url serves a different plan; fix the url or use an identity
          that plan declares as canonical or superseded

every url failed                                                          exit 1
  wipctl: could not clone peer 'payments' from any of 2 urls
  wipctl: <the transport's own error for each, relayed>
  wipctl: check the urls and your access, then re-run
          'wipctl attach --peers'

the plan names no peers                                                   exit 0
  wipctl: this plan names no peers
  wipctl: a peer is declared in the peers section of .wipctl/plan.toml;
          add one with 'wipctl peer add <url>'
```

Rules that bind this section: `peer-attachment:a-failure-writes-its-listed-message`.

## Peer table messages

Each failure names its resolution.

```text
a peer row with no plan_id                                                exit 1
  wipctl: peer 'payments' declares no plan_id
  wipctl: a peer is identified by its plan_id, not by its url; read the
          identity from that plan's plan file and add it to the peers
          section

a peer row with an empty url list                                         exit 1
  wipctl: peer 'payments' declares no url
  wipctl: a peer nobody can locate is not a peer; add at least one
          location with 'wipctl peer url payments --add <url>'

a row naming this plan                                                    exit 1
  wipctl: peer 'mirror' names this plan
  wipctl:   plan_id 9f2c41a08b7d4e63a15c8f02d7e4b619 is this plan's own
  wipctl: a peer is another plan; a dependency inside this record is a
          bare needs id, so remove the row

a url carrying credentials                                                exit 1
  wipctl: peer 'payments' declares a url with credentials in it
  wipctl:   https://<redacted>@git.example.org/acme/payments-plan.git
  wipctl: a committed file is read by everyone who clones this plan;
          remove the credentials from the url and let the transport get
          them from the machine

a url with an unaccepted scheme                                           exit 1
  wipctl: peer 'payments' declares a url wipctl will not open
  wipctl:   ext::sh -c 'anything'
  wipctl: a peer url carries one of the schemes the specification accepts;
          correct the row, and the url is not attempted meanwhile

two rows claiming one plan_id                                             exit 1
  wipctl: peers 'payments' and 'billing' name one plan
  wipctl:   plan_id 9f2c41a08b7d4e63a15c8f02d7e4b619
  wipctl: one plan gets one alias; drop the row you do not use

the view runs against a plan with no table                                exit 0
  wipctl: this plan names no peers
  wipctl: a peer is declared in the peers section of .wipctl/plan.toml;
          see 'wipctl help peers'
```

Rules that bind this section: `peers:a-table-fault-writes-its-listed-message`.

## Plan targeting messages

```text
an alias the table does not declare                                       exit 2
  wipctl: unknown plan 'payments'
  wipctl: --plan selects a peer this plan declares; run 'wipctl peers'
          to see the aliases, or add it with 'wipctl peer add <url>'

an alias a peer declares, but this plan does not                          exit 2
  wipctl: unknown plan 'gamma'
  wipctl: 'gamma' is declared by peer 'beta', not by this plan. An alias
          is local to the plan that declares it, so --plan reads this
          plan's peers section only. Declare it here with 'wipctl peer
          add <url>', or run the verb from inside that plan.

--plan on a verb that resolves nothing                                    exit 2
  wipctl: 'init' does not take --plan
  wipctl: init creates the plan for the project you are in, so there is
          nothing for --plan to redirect; run it from that project

--plan with --all                                                         exit 2
  wipctl: --plan and --all cannot be combined
  wipctl: --plan names one plan and --all names every plan in the
          closure; choose one

the named plan is not attached                                            exit 1
  wipctl: plan 'payments' is declared but not attached on this machine
  wipctl: run 'wipctl attach --peers' to fill its slot
```

Rules that bind this section: `plan-targeting:a-refusal-writes-its-listed-message`.

## Peer dependency resolution messages

One resolver turns a prefixed dependency into an entry, and these are its messages.

```text
the target was deleted in the peer                                        exit 1
  wipctl: needs payments#secure-session-storage: that entry was deleted
          in the payments plan on 2026-08-30
  wipctl: the id is burned there for the life of that record; drop the
          dependency, or ask its owners what replaced the work

the target was renamed in the peer                                        exit 1
  wipctl: needs payments#secure-session-storage: that entry was renamed
          in the payments plan on 2026-08-30
  wipctl:   it is now  payments#session-identity-storage
  wipctl: update the reference on profile-composition

the target was never there                                                exit 1
  wipctl: needs payments#secure-session-storage: no such entry and no
          tombstone in the payments plan at 91a47c0d
  wipctl: check the id, or the slot may be behind; run
          'wipctl sync --all' and try again

a peer in the read set is not attached                                    exit 1
  wipctl: peer 'payments' is not attached on this machine
  wipctl: run 'wipctl attach --peers' to clone every plan this one names

a declared row nothing references                                         exit 0
  wipctl: stale: no entry needs anything from peer 'platform'
  wipctl: drop the row with 'wipctl peer remove platform', or leave it
          for the work that will use it

a peer row naming a settled identity                                      exit 0
  wipctl: peer 'payments' names an identity that plan superseded
  wipctl:   the row says   aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  wipctl:   the plan says  22222222222222222222222222222222
  wipctl: the row still resolves; update it when you next edit this
          table, and nothing breaks if you do not
```

Three answers replace one guess. The third names the revision, because at that point the slot being behind is the likeliest explanation left.

Rules that bind this section: `validation:the-resolver-writes-its-listed-message`.

## Slot report messages

```text
a slot name that no longer matches its project                    exit 0
  wipctl: slot 'payments' holds a project whose name derives as
          'payments-ben'
  wipctl: the slot keeps the name it was given, and nothing is wrong
          with the record; run 'wipctl fix --registry' when you want the
          tree to read the new name
```

Rules that bind this section: `doctor:a-drifted-slot-is-healthy`.
