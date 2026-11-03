# Diagnostics

The diagnostic classes, and the messages of the class that relays another tool's output. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

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
- Two things change on the way through, and only these two. Credentials inside a url become `<redacted>`, and control bytes are escaped. A diagnostic is copied into terminals, transcripts, and continuous-integration logs. Relaying a credential leaks it further than it started, and relaying raw control bytes hands a remote server the reader's terminal. The excerpt is bounded and the whole output is kept. The message carries the first 200 lines and names a file holding everything, so fidelity is in the file and the terminal stays readable. The bound is a number in the rule, because an implementer who has to choose one will choose a different one.
- The on-disk statement comes next, so a reader knows whether anything moved before reading further. Then the three places to read, the same three in every message of the class, so an agent learns one procedure.
- The orientation names the failure that matters, which is a lost state. It names the verbs that are safe and the actions that are not, and it addresses the reader as you.

Every line of the class is a diagnostic, so all of it goes to the diagnostic stream. The relayed lines carry the prefix on the marker lines rather than on the tool's own text, so a reader can lift the block unchanged.

The plan trunk has one published branch and its history is append-only, so a stray branch here means one that something outside the tool created. The message says so rather than suggesting the plan has branches to reconcile.

Rules that bind this section: `messages:relayed-output-is-redacted-and-bounded`, `cli-conventions:stdout-carries-data-alone`.
