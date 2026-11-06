# wipctl

wipctl is a planning method and the tooling that proves a project's plan record obeys it. The two ship together and are judged as one. The method plans with bounded stories, judgment-counting points, five lanes, and recorded transitions. The tooling is one command. It scaffolds a project's plan, answers what to start next, and takes work atomically. It validates the record and moves entries between lanes as recorded events. It captures and lands new work, replicates the plan between machines, and renders the record as terminal views.

A project's plan lives in a plan repository of its own. That is a plain git repository. It holds five lane files, one document per story, epic, and initiative, a charter, open questions, a transition journal, and an on-demand capture directory. Every worktree and clone of the project on a machine shares it, and its operator chooses where it is hosted.

The project itself carries exactly one committed file, `.wipctl/project.toml`, naming its identity. That file is the entire footprint. Everything else belongs to the project.

## Install

Build and install the command from a checkout:

```bash
git clone https://github.com/gubasso/wipctl.git
cd wipctl
cargo install --path .
```

[docs/specs/](./docs/README.md) specifies the command surface the binary implements. Read it for the exact verbs, arguments, and exit codes, rather than inferring them from the build.

## Development

Tooling is pinned in a Nix flake devShell, so a contributor gets the same toolchain the gates run against:

```bash
direnv allow   # loads the devShell on entering the directory
nix develop    # or enter it ad hoc
```

Checks live in the pre-commit hooks and the `justfile`, and nowhere else. Every tool they call comes from the devShell. Run each one through it:

```bash
nix develop --command just lint        # the commit stage
nix develop --command just lint-push   # the push stage
nix develop --command just check-all   # both stages, the run that equals CI
```

In a shell that direnv loaded, run the same recipes directly, or with `direnv exec . just <recipe>` from outside.

Install the hooks once with `pre-commit install` inside the devShell. The commit stage runs the cheap checks: formatting, lints, spelling, the documentation gates, the unit tests, and the secret scan. The push stage runs the costly ones: the integration tests, the release build, the package build, the package verification, and the dependency, history, and networked checks. The push stage repeats none of the commit stage. CI runs the two stages and nothing beside them, so a push that passed its hooks passes CI. [docs/specs/SPEC-quality-gates.md](./docs/specs/SPEC-quality-gates.md) states what this repository owes every artifact it ships, and it binds any change that adds one.

Three workflows run outside the two stages:

- `pr-title.yml` checks the title and the body of a pull request. Both exist only at the forge, so no hook can read them. The commit-msg hooks hold the same rules for each local commit message.
- `release-plz.yml` verifies the package before it publishes, with the toolchain and the `release-plz` binary of the devShell, and the `publish-dry` hook runs the same check at the push stage.
- `release.yml` builds the release artifacts and runs no check of the code. The `dist-generate-check` hook holds it equal to `dist-workspace.toml`.

## Documentation

Everything about the product lives under [docs/](./docs/README.md), organised by reader need. There are guides for task sequences, specs for binding rules, explanation for the method and the design, and decisions for why. The spec zone is the normative specification this build conforms to. `AGENTS.md` states how the two relate while the build is under way.

Start at [docs/explanation/overview.md](./docs/explanation/overview.md) to learn the method, or at [docs/specs/SPEC-zone-layout.md](./docs/specs/SPEC-zone-layout.md) to build against the specification.

## License

Licensed under either of [LICENSE-APACHE](./LICENSE-APACHE) (Apache License 2.0) or [LICENSE-MIT](./LICENSE-MIT) (MIT license) at your option. One licence covers both deliverables, the method and the tooling. [docs/decisions/ADR-the-method-and-the-tooling-share-one-licence.md](./docs/decisions/ADR-the-method-and-the-tooling-share-one-licence.md) records why.

The tree carries one third-party text under its own terms. `.spec-driven-docs/upstreams/simpleenglish/` holds a vendored writing pattern, and its `LICENSE` file sits beside the files it covers. The published package excludes that directory.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
