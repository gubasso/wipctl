# the registry credential is minted per release run

## Context and Problem Statement

`wipctl` is installed by the projects it serves, so it ships to a package registry and the release path is part of the product. A registry credential that can publish under the maintainer's name is the highest-value secret this repository touches, and the choice of how a release run obtains one is hard to reverse: a registry binds a trusted publisher to one repository and one workflow filename, and a stored token outlives every run that used it.

## Considered Options

- A long-lived registry token held as a repository secret
- A credential minted per run from the workflow's own signed identity
- Publishing only from a maintainer's machine, never from automation

## Decision Outcome

Chosen option: `a credential minted per run from the workflow's own signed identity` — the release run proves what it is rather than presenting something it holds, so there is no standing secret to leak, rotate, or revoke.

## Consequences

- Good: no publishing credential exists between releases, and a compromised repository secret cannot publish.
- Good: the credential is scoped to one run, so its blast radius ends when the run does.
- Bad: the first release cannot use it. A trusted publisher registers against a package that already exists, so one manual release with a short-lived, narrowly scoped token precedes it, and that token is revoked immediately after.
- Bad: the registration matches on the release workflow's filename, so renaming or splitting that workflow silently breaks publishing until the registration is updated. The workflow that builds binary artifacts is therefore a separate file that publishes nothing.
- Bad: automation now owns the version bump and the tag, so a hand-created tag desynchronizes the record from what was published.

## Status

Accepted

The release path this record governs is [guides/publishing-a-release.md](../guides/publishing-a-release.md).
